#!/usr/bin/env python3
"""Valida os experimentos e gera PDF/TEX com uma representação comum.

Importar este módulo não carrega ReportLab e não produz arquivos.
"""
from pathlib import Path
from dataclasses import dataclass
import re
import math
import sys
import hashlib
import json

RAIZ = Path(__file__).resolve().parent
QUANTUMS = tuple(range(1, 11)) + (12, 15, 20, 25, 30)
# Identidade semântica da entrada fornecida; nunca uma fonte de medições.
ASSINATURA_CARGA_OFICIAL = 'd99cfad878cd332f4ec96cb8ddf214723e2d47f29a54d3511fdd9202ae4c892b'


@dataclass(frozen=True)
class Programa:
    arquivo: str
    nome: str
    prioridade: int
    instrucoes: tuple[str, ...]


def linhas_rust(texto):
    """Equivalente a BufRead::lines: LF e remoção de CR somente em CRLF.

    CR isolado, tabulação vertical, form feed e separadores Unicode
    permanecem dentro da linha, sem normalização universal de newlines.
    """
    partes = texto.split('\n')
    terminadas = len(partes) - 1
    if partes[-1] == '':
        partes.pop()
    return [p[:-1] if i < terminadas and p.endswith('\r') else p
            for i, p in enumerate(partes)]


def ler_linhas(caminho):
    return linhas_rust(Path(caminho).read_bytes().decode('utf-8'))


def carregar_entradas(raiz):
    """Lê programas e configurações sem alterar os arquivos oficiais.

    Nomes repetidos são recusados: o log identifica processos pelo nome,
    impedindo sua reconstituição inequívoca nesse caso.
    """
    pasta = Path(raiz) / 'programas'
    arquivos = sorted(p for p in pasta.iterdir()
                      if p.is_file() and re.fullmatch(r'[0-9]{2}\.txt', p.name))
    if not arquivos:
        raise ValueError('nenhum programa NN.txt encontrado')
    prioridades = ler_linhas(pasta / 'prioridades.txt')
    if len(prioridades) != len(arquivos):
        raise ValueError('exigida uma prioridade por programa')
    if any(not re.fullmatch(r'[0-9]+', v) or int(v) > 2**32 - 1 for v in prioridades):
        raise ValueError('prioridade fora do intervalo u32')
    quanta = ler_linhas(pasta / 'quantum.txt')
    if (len(quanta) != 1 or not re.fullmatch(r'[0-9]+', quanta[0])
            or not 0 < int(quanta[0]) <= 2 * sys.maxsize + 1):
        raise ValueError('quantum.txt exige um inteiro positivo usize')
    programas = []
    nomes = set()
    for arquivo, prioridade in zip(arquivos, prioridades):
        linhas = ler_linhas(arquivo)
        if not linhas or not linhas[0].strip() or linhas[0] in nomes:
            raise ValueError(f'{arquivo.name}: nome vazio ou repetido')
        comandos = linhas[1:]
        if not 1 <= len(comandos) <= 21 or comandos[-1] != 'SAIDA' or 'SAIDA' in comandos[:-1]:
            raise ValueError(f'{arquivo.name}: limite de 21 instruções e SAIDA única final exigidos')
        for comando in comandos:
            if comando in ('COM', 'E/S', 'SAIDA'):
                continue
            atribuicao = re.fullmatch(r'[XY]=([+-]?[0-9]+)', comando)
            if not atribuicao or not -2**63 <= int(atribuicao[1]) < 2**63:
                raise ValueError(f'{arquivo.name}: instrução inválida: {comando!r}')
        nomes.add(linhas[0])
        programas.append(Programa(arquivo.name, linhas[0], int(prioridade), tuple(comandos)))
    return programas


@dataclass(frozen=True)
class Medicao:
    quantum: int
    processos: int
    despachos: int
    instrucoes: int
    registradores: dict[str, tuple[int, int]]

    @property
    def trocas(self):
        return self.despachos / self.processos

    @property
    def instr_quantum(self):
        return self.instrucoes / self.despachos

    @property
    def razao(self):
        return self.instr_quantum / self.quantum


def validar_log(texto, programas, quantum):
    """Reconstitui cada surto do log contra os programas e a política do EP.

    Contagens, PC, X/Y, carga, ordem, créditos, E/S, término e resumo são
    conferidos; ociosidade lógica avança espera sem gerar despachos.
    """
    if not programas or isinstance(quantum, bool) or not isinstance(quantum, int) or quantum <= 0:
        raise ValueError('programas não vazios e quantum positivo exigidos')
    linhas = linhas_rust(texto)
    indice = 0

    def exigir(esperado):
        nonlocal indice
        atual = linhas[indice] if indice < len(linhas) else '<fim do arquivo>'
        if atual != esperado:
            raise ValueError(f'linha {indice + 1}: esperado {esperado!r}; encontrado {atual!r}')
        indice += 1

    contextos = [{'p': p, 'pc': 0, 'x': 0, 'y': 0, 'credito': p.prioridade,
                  'espera': 0, 'terminado': False} for p in programas]
    prontos = sorted(range(len(programas)), key=lambda i: -programas[i].prioridade)
    bloqueados = []
    for pid in prontos:
        exigir('Carregando ' + programas[pid].nome)
    despachos = instrucoes = 0
    finais = {}

    def inserir(pid, antes):
        credito = contextos[pid]['credito']
        todos_zero = credito == 0 and all(contextos[i]['credito'] == 0 for i in prontos)
        bloqueado_positivo = any(contextos[i]['credito'] > 0 for i in bloqueados)
        ativos = [c for c in contextos if not c['terminado']]
        if (todos_zero and bloqueado_positivo) or all(c['p'].prioridade == 0 for c in ativos):
            prontos.append(pid)
            return
        prontos.sort(key=lambda i: -contextos[i]['credito'])
        pos = next((j for j, i in enumerate(prontos)
                    if contextos[i]['credito'] < credito
                    or (antes and contextos[i]['credito'] == credito)), len(prontos))
        prontos.insert(pos, pid)

    def avancar(snapshot):
        for pid in snapshot:
            contextos[pid]['espera'] -= 1
        liberados = [i for i in bloqueados if contextos[i]['espera'] == 0]
        for pid in liberados:
            bloqueados.remove(pid)
            inserir(pid, False)

    while prontos or bloqueados:
        if not prontos:
            avancar(tuple(bloqueados))
            continue
        ativos = [c for c in contextos if not c['terminado']]
        if all(c['credito'] == 0 for c in ativos):
            for c in ativos:
                c['credito'] = c['p'].prioridade
            prontos.sort(key=lambda i: -contextos[i]['credito'])
        pid = prontos.pop(0)
        c = contextos[pid]
        p = c['p']
        exigir('Executando ' + p.nome)
        c['credito'] = max(0, c['credito'] - 1)
        snapshot = tuple(bloqueados)
        contagem = 0
        parada = None
        while contagem < quantum:
            if c['pc'] >= len(p.instrucoes):
                raise ValueError(f'{p.nome}: PC fora do programa')
            comando = p.instrucoes[c['pc']]
            c['pc'] += 1
            contagem += 1
            if comando.startswith(('X=', 'Y=')):
                c[comando[0].lower()] = int(comando[2:])
            if comando in ('E/S', 'SAIDA'):
                parada = comando
                break
        if parada == 'SAIDA':
            exigir(f"{p.nome} terminado. X={c['x']}. Y={c['y']}")
            c['terminado'] = True
            finais[p.nome] = (c['x'], c['y'])
        else:
            if parada == 'E/S':
                exigir('E/S iniciada em ' + p.nome)
                c['espera'] = 2
                bloqueados.append(pid)
            unidade = 'instrução' if contagem == 1 else 'instruções'
            exigir(f'Interrompendo {p.nome} após {contagem} {unidade}')
            if parada is None:
                inserir(pid, True)
        despachos += 1
        instrucoes += contagem
        avancar(snapshot)
    dado = Medicao(quantum, len(programas), despachos, instrucoes, finais)
    for prefixo, esperado in [('MEDIA DE TROCAS: ', dado.trocas),
                             ('MEDIA DE INSTRUCOES: ', dado.instr_quantum)]:
        if indice >= len(linhas) or not linhas[indice].startswith(prefixo):
            raise ValueError(f'linha {indice + 1}: resumo ausente: {prefixo}')
        try:
            valor = float(linhas[indice][len(prefixo):])
        except ValueError as erro:
            raise ValueError(f'linha {indice + 1}: média inválida') from erro
        if not math.isfinite(valor) or not math.isclose(valor, esperado, rel_tol=1e-12, abs_tol=1e-12):
            raise ValueError(f'linha {indice + 1}: média {valor} difere de {esperado}')
        indice += 1
    exigir(f'QUANTUM: {quantum}')
    if indice != len(linhas):
        raise ValueError(f'linha {indice + 1}: evento adicional após o resumo')
    return dado


def coletar_dados(raiz=RAIZ, quantums=QUANTUMS):
    """Valida a entrada e ambas as coleções, devolvendo programas e medições."""
    raiz = Path(raiz)
    programas = carregar_entradas(raiz)
    dados = []
    for quantum in quantums:
        nome = f'log{quantum:02}.txt'
        arquivo = raiz / nome
        copia = raiz / 'experimentos' / nome
        if arquivo.read_bytes() != copia.read_bytes():
            raise ValueError(f'{nome}: coleções raiz/experimentos divergem')
        try:
            dados.append(validar_log(arquivo.read_bytes().decode('utf-8'), programas, quantum))
        except ValueError as erro:
            raise ValueError(f'{nome}: {erro}') from erro
    return programas, dados


def selecionar_quantum(dados, maximo, limiar=0.85):
    """Menor quantum testado que alcança a fração escolhida da média natural."""
    candidatos = [d for d in dados if d.instr_quantum >= limiar * maximo]
    if not candidatos:
        raise ValueError('nenhum quantum testado alcança o critério')
    return min(candidatos, key=lambda d: d.quantum)


def decimal(valor, casas=2):
    return f'{valor:.{casas}f}'.replace('.', ',')


def criar_documento(programas, dados):
    """Fonte comum de conteúdo, tabelas e séries para os dois renderizadores."""
    identidade = [(p.arquivo, p.nome, p.prioridade, p.instrucoes) for p in programas]
    assinatura = hashlib.sha256(json.dumps(
        identidade, ensure_ascii=False, separators=(',', ':')).encode('utf-8')).hexdigest()
    if assinatura != ASSINATURA_CARGA_OFICIAL:
        raise ValueError('relatório restrito à carga oficial dos dez programas fornecidos '
                         '(arquivos, nomes, prioridades e instruções)')
    if sorted(d.quantum for d in dados) != list(QUANTUMS):
        raise ValueError('relatório exige as quinze medições do recorte oficial')
    trechos = []
    for p in programas:
        tamanho = 0
        for instrucao in p.instrucoes:
            tamanho += 1
            if instrucao in ('E/S', 'SAIDA'):
                trechos.append(tamanho)
                tamanho = 0
    total = sum(len(p.instrucoes) for p in programas)
    maximo = total / len(trechos)
    escolhido = selecionar_quantum(dados, maximo)
    if escolhido.quantum != 7:
        raise ValueError('medições incompatíveis com a carga oficial e sua narrativa q7')
    por_q = {d.quantum: d for d in dados}
    minimo = min(d.despachos for d in dados)
    custo = (escolhido.despachos / minimo - 1) * 100
    reducao1 = (1 - escolhido.despachos / por_q[1].despachos) * 100
    reducao3 = (1 - escolhido.despachos / por_q[3].despachos) * 100
    cobertura = escolhido.instr_quantum / maximo * 100
    p = lambda texto: ('p', texto)
    h = lambda texto: ('h', texto)
    tabela = lambda cabecalho, linhas: ('table', (cabecalho, linhas))
    grafico = lambda numero, legenda, atributo, eixo, teto: ('chart', {
        'caption': f'Figura {numero}. {legenda}', 'ylabel': eixo, 'maxy': teto,
        'yticks': [0, 5, 10, 15] if teto == 15 else (list(range(6)) if teto == 5 else [0, .25, .5, .75, 1]),
        'points': [(d.quantum, getattr(d, atributo)) for d in dados],
    })
    return [
        {'title': 'EP1 - Escalonador de Processos', 'blocks': [
            h('Relatório de avaliação do sistema'),
            p('Sistemas Operacionais - 2026. Implementação em Rust, edição 2021.'),
            h('Integrantes'),
            p('Cristian de Lima Meira - USP: 16860754'),
            p('Fernando Silva - USP: 15636702'),
            p('Julio Domingos - USP: 16836502'),
            h('Objetivo e resultado'),
            p('Avaliar o escalonador de uma máquina fictícia com um processador, registradores X/Y e '
              'instruções de atribuição, COM, E/S e SAIDA. A avaliação usa os dez programas oficiais '
              'e compara as médias exigidas pelo EP em quinze valores de quantum.'),
            p(f'Adotamos quantum {escolhido.quantum}: é o menor valor testado que alcança pelo menos '
              f'85% da média máxima de instruções por despacho desta carga. Ele atinge {decimal(cobertura)}% '
              f'dessa média e reduz os despachos em {decimal(reducao1)}% em relação a q1. '
              'O limiar de 85% é uma escolha prática dos autores, não uma prova de ótimo universal.'),
            p('Os resultados vêm dos eventos dos logs, conferidos contra as instruções e prioridades. '
              'PDF e TEX são derivados do mesmo conteúdo em gerar_relatorio.py. As diferenças de '
              'diagramação entre os dois formatos não alteram textos, tabelas ou séries numéricas.'),
        ]},
        {'title': '1. Método e carga de trabalho', 'blocks': [
            p('A série principal contém q1 a q10, com passo unitário: dez valores uniformemente '
              'distribuídos no intervalo útil de 1 a 10 instruções. Esse intervalo cobre a queda '
              'mais acentuada dos despachos e o critério de escolha. Os pontos q12, q15, q20, q25 '
              'e q30 são uma exploração complementar, com espaçamento não uniforme.'),
            p('O limite de um programa é 21 instruções. Os pontos q25 e q30 excedem esse limite '
              'e servem para confirmar a saturação; não são necessários para representar novos '
              'efeitos de preempção. Prioridades e instruções foram mantidas em todos os casos.'),
            tabela(['Arquivo', 'Programa', 'Instruções', 'E/S', 'Prioridade'], [
                [x.arquivo, x.nome, str(len(x.instrucoes)), str(x.instrucoes.count('E/S')), str(x.prioridade)]
                for x in programas]),
            h('Contagens e validação'),
            p(f'A carga possui {len(programas)} processos, {total} instruções e {len(trechos)} trechos '
              f'terminados por E/S ou SAIDA. A média natural desses trechos é {total}/{len(trechos)} '
              f'= {decimal(maximo, 4)} instruções; o maior tem {max(trechos)} instruções.'),
            p('Para cada quantum, o gerador exige igualdade byte a byte dos logs da raiz e de '
              'experimentos/. Em seguida reconstitui carga, fila de prontos, créditos, esperas, '
              'PC, X/Y, instruções por surto, eventos e resumo. Todos os quinze casos são validados '
              'antes da escrita de qualquer saída. Isso confere os artefatos existentes; não '
              'substitui uma nova execução do binário Rust.'),
        ]},
        {'title': '2. Resultados e definição das métricas', 'blocks': [
            p('P é o número inicial de processos; D é o número de despachos concluídos; I é o total '
              'de instruções executadas. A média de trocas do EP é D/P, incluindo a última execução '
              'e despachos consecutivos do mesmo PID. Ela não representa apenas mudanças físicas '
              'entre processos distintos. E/S e SAIDA contam em I; ociosidade não conta em D ou I.'),
            p('A média de instruções por quantum é I/D. Definimos também R = (I/D)/q: a fração '
              'média do limite de instruções utilizada em cada despacho. R é uma razão de contagens '
              'lógicas, não uma medida de utilização da CPU em tempo real.'),
            tabela(['q', 'D', 'I', 'D/P', 'I/D', 'R (%)'], [
                [str(d.quantum), str(d.despachos), str(d.instrucoes), decimal(d.trocas),
                 decimal(d.instr_quantum, 4), decimal(d.razao * 100)] for d in dados]),
            p('Tabela 1. Medições extraídas e validadas a partir dos logs. Nas quinze execuções, '
              f'P = {len(programas)} e I = {total}. Médias são calculadas em ponto flutuante; '
              'o arredondamento acima é apenas de apresentação.'),
        ]},
        {'title': '3. Efeito do quantum nas médias', 'blocks': [
            grafico(1, 'Média de trocas do EP por processo em função de q.', 'trocas', 'D/P', 15),
            p('A maior redução ocorre nos primeiros valores de quantum. Depois, aumentá-lo '
              'reduz cada vez menos os despachos, porque os trechos de CPU terminam em E/S ou SAIDA.'),
            grafico(2, 'Média de instruções executadas por despacho em função de q.',
                    'instr_quantum', 'I/D', 5),
            p('Uma E/S ou SAIDA encerra o surto mesmo quando ainda resta quantum. Por isso I/D '
              f'não acompanha q indefinidamente: o limite desta carga é {decimal(maximo, 4)}.'),
        ]},
        {'title': '4. Razão utilizada e saturação', 'blocks': [
            grafico(3, 'Fração média do quantum utilizada em cada despacho.', 'razao', 'R = (I/D)/q', 1.05),
            p('R diminui com q porque os surtos naturais são curtos em boa parte da carga. Esse '
              'gráfico compara a média executada com o limite permitido, como pede o enunciado. '
              'Uma razão menor não significa, por si só, CPU desperdiçada: o processo libera o '
              'processador ao bloquear ou terminar.'),
            p(f'D/P e I/D não são critérios independentes: seu produto é I/P = '
              f'{total}/{len(programas)} = {decimal(total / len(programas), 1)}. Um gráfico entre '
              'essas duas médias apenas repete essa identidade; não demonstra um compromisso '
              'com responsividade ou tempo de espera.'),
            p(f'Sem preempção interna aos trechos, cada um exige um despacho: D mínimo = '
              f'{len(trechos)} e I/D máximo = {decimal(maximo, 4)}. A saturação ocorre em '
              f'q = {max(trechos)}, o tamanho do maior trecho. Esse valor é deduzido das '
              'instruções, não uma execução adicional incluída na tabela. q25 e q30 confirmam '
              'o mesmo mínimo observado. Quantum grande mantém a política de créditos; '
              'não transforma a fila de prontos em FIFO.'),
        ]},
        {'title': '5. Escolha do quantum', 'blocks': [
            p('O critério escolhido é atingir pelo menos 85% da média máxima de instruções '
              'por despacho com o menor quantum testado. Ele combina uma meta explícita '
              'de redução dos despachos com um limite menor de instruções consecutivas '
              'por execução; não otimiza uma função de tempo real.'),
            tabela(['q', 'D', 'I/D', 'Fração do máximo (%)'], [
                [str(por_q[q].quantum), str(por_q[q].despachos), decimal(por_q[q].instr_quantum, 4),
                 decimal(por_q[q].instr_quantum / maximo * 100)] for q in (6, 7, 8)]),
            p(f'q6 alcança {decimal(por_q[6].instr_quantum / maximo * 100)}% do máximo e não '
              f'atinge a meta. q7 alcança {decimal(por_q[7].instr_quantum / maximo * 100)}% e é '
              'o primeiro valor testado que a satisfaz. q8 tem os mesmos D e I/D de q7, '
              'mas permite uma instrução adicional consecutiva; portanto q7 é preferível '
              'segundo o critério adotado.'),
            h('Benefícios e custo da escolha'),
            p(f'Em q7, D/P = {decimal(escolhido.trocas)} e I/D = '
              f'{decimal(escolhido.instr_quantum, 4)}. Há {decimal(reducao1)}% menos despachos '
              f'que em q1 e {decimal(reducao3)}% menos que em q3. Em contrapartida, q7 usa '
              f'{decimal(custo)}% mais despachos que o mínimo observado de {minimo} '
              f'({decimal(minimo / len(programas))} por processo).'),
            p(f'O tamanho médio natural dos trechos é {decimal(maximo, 4)}, não 7. A escolha de q7 decorre '
              'da meta de 85%, e não de uma suposta igualdade com esse tamanho médio. q3 '
              'é o valor da entrada original fornecida; os resultados não permitem '
              'inferir que seus autores priorizaram responsividade.'),
            h('Limites da conclusão'),
            p('Não foram medidos tempo de resposta, espera, throughput em tempo real ou custo '
              'de salvar/carregar contexto. Quantum menor limita cada surto, mas prioridades '
              'e empates podem produzir execuções consecutivas do mesmo PID. A recomendação '
              'é específica desta carga, desta política e do limiar escolhido; outra carga '
              'ou outra meta pode justificar outro quantum.'),
        ]},
        {'title': '6. Política, implementação e reprodução', 'blocks': [
            h('Estruturas e decisões de escalonamento'),
            p('O BCP armazena PID, nome/programa, PC, estado, prioridade, créditos, X/Y e espera. '
              'A tabela possui os BCPs; prontos e bloqueados guardam somente ProcessId. '
              'A fila de prontos é ordenada por créditos, e bloqueados seguem ordem de chegada.'),
            p('Créditos começam na prioridade e diminuem em uma unidade por despacho, sem '
              'ficar negativos. A redistribuição usa as prioridades de todos os processos '
              'ativos, inclusive bloqueados, somente quando todos têm zero créditos. Quando '
              'prontos têm zero e bloqueados ainda têm crédito, aplica-se FIFO aos prontos.'),
            p('E/S conta como instrução ao começar, bloqueia por dois surtos posteriores e '
              'retoma na instrução seguinte. O surto que inicia a E/S não desconta sua própria '
              'espera. Sem prontos, a espera avança por ticks lógicos. SAIDA também conta, '
              'remove o processo e registra X/Y finais.'),
            h('Desempate e custo do contexto'),
            p('Na carga inicial, empates seguem a ordem dos arquivos. Após quantum, a '
              'reinserção ocorre antes dos créditos iguais; no retorno de E/S, depois deles, '
              'respeitando as exceções FIFO. Manter o processo que já rodava quando seus '
              'créditos empatam evita salvar e carregar outro contexto em uma máquina real '
              'e pode preservar dados em cache. Esses custos não são modelados pelo EP. '
              'Mesmo ao repetir o PID, o novo surto conta em D conforme a convenção exigida.'),
            h('Reprodução e alcance'),
            p('Execute o binário Escalonador com programas/ no diretório corrente. Para '
              'regenerar este relatório, instale a dependência em requirements-relatorio.txt '
              'e execute python3 gerar_relatorio.py de qualquer diretório. O gerador lê os '
              'dados relativos à sua própria localização, valida as duas coleções de logs '
              'e só então produz relatorio_ep1.pdf e relatorio_ep1.tex. A importação e os '
              'testes de validação usam apenas a biblioteca padrão de Python.'),
            p('gerar_relatorio.py é a fonte comum do relatório; ReportLab desenha os gráficos '
              'vetoriais no PDF e o TEX contém os mesmos pontos em desenhos autossuficientes. '
              'O enunciado ep1.pdf fundamenta métricas, regras e material de entrega. '
              'O relatório avalia os artefatos locais; não comprova a data ou a realização '
              'da submissão via eDisciplinas. A entrega exige um ZIP com número USP, código, '
              'logs e este PDF, enviado por um único integrante.'),
        ]},
    ]


def geometria_grafico(grafico):
    largura, altura = 470, 200
    esquerda, base, w, h = 44, 32, 410, 150
    maxq = max(q for q, _ in grafico['points'])
    pontos = [(esquerda + (q - 1) / (maxq - 1) * w,
               base + valor / grafico['maxy'] * h) for q, valor in grafico['points']]
    return largura, altura, esquerda, base, w, h, maxq, pontos


def renderizar_pdf(caminho, documento):
    # ReportLab é opcional para a importação e para os testes de validação.
    import reportlab
    from reportlab.pdfbase import pdfmetrics
    from reportlab.pdfbase.ttfonts import TTFont
    fontes = Path(reportlab.__file__).parent / 'fonts'
    pdfmetrics.registerFont(TTFont('EPRegular', str(fontes / 'Vera.ttf')))
    pdfmetrics.registerFont(TTFont('EPBold', str(fontes / 'VeraBd.ttf')))
    from reportlab.lib import colors
    from reportlab.lib.pagesizes import A4
    from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
    from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, PageBreak
    from reportlab.graphics.shapes import Drawing, Line, Circle, String
    from xml.sax.saxutils import escape
    estilos = getSampleStyleSheet()
    estilos.add(ParagraphStyle('TextoEP', fontName='EPRegular', fontSize=10,
                              leading=14, spaceAfter=10, textColor=colors.HexColor('#233044')))
    estilos.add(ParagraphStyle('TituloEP', fontName='EPBold', fontSize=20,
                              leading=25, spaceAfter=18, textColor=colors.HexColor('#123754')))
    estilos.add(ParagraphStyle('SecaoEP', fontName='EPBold', fontSize=12,
                              leading=16, spaceBefore=8, spaceAfter=8,
                              textColor=colors.HexColor('#123754')))
    estilos.add(ParagraphStyle('LegendaEP', fontName='EPRegular', fontSize=9,
                              leading=12, spaceAfter=12))
    story = []
    azul = colors.HexColor('#24759a')
    for pagina in documento:
        if story:
            story.append(PageBreak())
        story.append(Paragraph(escape(pagina['title']), estilos['TituloEP']))
        for tipo, conteudo in pagina['blocks']:
            if tipo in ('p', 'h'):
                story.append(Paragraph(escape(conteudo), estilos['TextoEP' if tipo == 'p' else 'SecaoEP']))
            elif tipo == 'table':
                cabecalho, linhas = conteudo
                tabela = Table([cabecalho] + linhas, repeatRows=1, hAlign='LEFT')
                tabela.setStyle(TableStyle([
                    ('FONTNAME', (0, 0), (-1, -1), 'EPRegular'),
                    ('FONTNAME', (0, 0), (-1, 0), 'EPBold'),
                    ('FONTSIZE', (0, 0), (-1, -1), 9),
                    ('TEXTCOLOR', (0, 0), (-1, 0), colors.white),
                    ('BACKGROUND', (0, 0), (-1, 0), colors.HexColor('#123754')),
                    ('ROWBACKGROUNDS', (0, 1), (-1, -1), [colors.white, colors.HexColor('#eef4f7')]),
                    ('ALIGN', (0, 0), (-1, -1), 'CENTER'),
                    ('LEFTPADDING', (0, 0), (-1, -1), 11),
                    ('RIGHTPADDING', (0, 0), (-1, -1), 11),
                    ('TOPPADDING', (0, 0), (-1, -1), 5),
                    ('BOTTOMPADDING', (0, 0), (-1, -1), 5),
                    ('LINEBELOW', (0, 0), (-1, 0), .5, colors.HexColor('#123754')),
                ]))
                story += [tabela, Spacer(1, 12)]
            else:
                largura, altura, x, y, w, h, maxq, pontos = geometria_grafico(conteudo)
                drawing = Drawing(largura, altura)
                for valor in conteudo['yticks']:
                    yy = y + valor / conteudo['maxy'] * h
                    drawing.add(Line(x, yy, x + w, yy, strokeColor=colors.HexColor('#dfe7ec'), strokeWidth=.5))
                    drawing.add(String(x - 6, yy - 3, decimal(valor, 2 if conteudo['maxy'] < 2 else 0), textAnchor='end', fontSize=8, fontName='EPRegular'))
                drawing.add(Line(x, y, x, y + h, strokeColor=colors.HexColor('#687786')))
                drawing.add(Line(x, y, x + w, y, strokeColor=colors.HexColor('#687786')))
                for tick in (1, 5, 10, 15, 20, 25, 30):
                    if tick <= maxq:
                        xx = x + (tick - 1) / (maxq - 1) * w
                        drawing.add(String(xx, y - 13, str(tick), textAnchor='middle', fontSize=8, fontName='EPRegular'))
                drawing.add(String(x, y + h + 9, conteudo['ylabel'], fontSize=9, fontName='EPRegular'))
                drawing.add(String(x + w / 2, 3, 'Quantum q (instruções)', textAnchor='middle', fontSize=9, fontName='EPRegular'))
                for a, b in zip(pontos, pontos[1:]):
                    drawing.add(Line(*a, *b, strokeColor=azul, strokeWidth=1.7))
                for xx, yy in pontos:
                    drawing.add(Circle(xx, yy, 2.5, fillColor=azul, strokeColor=azul))
                story += [drawing, Paragraph(escape(conteudo['caption']), estilos['LegendaEP'])]
    def rodape(canvas, doc):
        canvas.setTitle('EP1 - Escalonador de Processos - Relatório de avaliação')
        canvas.setAuthor('Cristian de Lima Meira; Fernando Silva; Julio Domingos')
        canvas.setFont('EPRegular', 8)
        canvas.setFillColor(colors.HexColor('#687786'))
        canvas.drawString(54, 30, 'EP1 - Sistemas Operacionais - 2026')
        canvas.drawRightString(A4[0] - 54, 30, str(doc.page))
    SimpleDocTemplate(str(caminho), pagesize=A4, rightMargin=54, leftMargin=54,
                      topMargin=48, bottomMargin=48).build(story, onFirstPage=rodape, onLaterPages=rodape)


def escapar_tex(texto):
    especiais = {'\\': r'\textbackslash{}', '&': r'\&', '%': r'\%', '$': r'\$',
                 '#': r'\#', '_': r'\_', '{': r'\{', '}': r'\}', '~': r'\textasciitilde{}',
                 '^': r'\textasciicircum{}'}
    return ''.join(especiais.get(c, c) for c in texto)


def renderizar_tex(caminho, documento):
    linhas = [r'\documentclass[10pt,a4paper]{article}', r'\usepackage[utf8]{inputenc}',
              r'\usepackage[T1]{fontenc}', r'\usepackage[portuguese]{babel}',
              r'\usepackage[margin=1.9cm]{geometry}', r'\usepackage{xcolor}',
              r'\usepackage{array}', r'\setlength{\parindent}{0pt}',
              r'\setlength{\parskip}{7pt}', r'\definecolor{epazul}{RGB}{36,117,154}',
              r'\begin{document}', '% Gerado de gerar_relatorio.py; editar a fonte comum.']
    for indice, pagina in enumerate(documento):
        if indice:
            linhas.append(r'\clearpage')
        linhas.append(r'\section*{' + escapar_tex(pagina['title']) + '}')
        for tipo, conteudo in pagina['blocks']:
            if tipo == 'p':
                linhas += [escapar_tex(conteudo), '']
            elif tipo == 'h':
                linhas.append(r'\subsection*{' + escapar_tex(conteudo) + '}')
            elif tipo == 'table':
                cabecalho, dados = conteudo
                linhas += [r'\begin{center}', r'\small',
                           r'\begin{tabular}{' + 'c' * len(cabecalho) + '}', r'\hline',
                           ' & '.join(r'\textbf{' + escapar_tex(c) + '}' for c in cabecalho) + r' \\', r'\hline']
                linhas += [' & '.join(escapar_tex(c) for c in linha) + r' \\' for linha in dados]
                linhas += [r'\hline', r'\end{tabular}', r'\end{center}']
            else:
                largura, altura, x, y, w, h, maxq, pontos = geometria_grafico(conteudo)
                def put(xx, yy, corpo):
                    return r'\put(' + f'{xx:.3f},{yy:.3f}' + '){' + corpo + '}'
                linhas += [r'\begin{center}', r'\setlength{\unitlength}{1pt}',
                           r'\begin{picture}(' + f'{largura},{altura}' + ')']
                for valor in conteudo['yticks']:
                    yy = y + valor / conteudo['maxy'] * h
                    linhas.append(r'{\color{gray!25}' + put(x, yy, r'\line(1,0){' + str(w) + '}') + '}')
                    linhas.append(put(x - 6, yy, r'\makebox(0,0)[r]{\scriptsize '
                                      + decimal(valor, 2 if conteudo['maxy'] < 2 else 0) + '}'))
                linhas += [put(x, y, r'\line(0,1){' + str(h) + '}'),
                           put(x, y, r'\line(1,0){' + str(w) + '}')]
                for tick in (1, 5, 10, 15, 20, 25, 30):
                    if tick <= maxq:
                        xx = x + (tick - 1) / (maxq - 1) * w
                        linhas.append(put(xx, y - 13, r'\makebox(0,0){\scriptsize ' + str(tick) + '}'))
                linhas += [put(x, y + h + 9, r'\makebox(0,0)[l]{\small '
                               + escapar_tex(conteudo['ylabel']) + '}'),
                           put(x + w / 2, 3, r'\makebox(0,0){\small Quantum q (instruções)}'),
                           r'\color{epazul}\linethickness{1pt}']
                for a, b in zip(pontos, pontos[1:]):
                    linhas.append(r'\qbezier' + f'({a[0]:.3f},{a[1]:.3f})'
                                  f'({(a[0] + b[0]) / 2:.3f},{(a[1] + b[1]) / 2:.3f})'
                                  f'({b[0]:.3f},{b[1]:.3f})')
                linhas += [put(xx, yy, r'\circle*{4}') for xx, yy in pontos]
                linhas += [r'\end{picture}', r'\end{center}',
                           r'{\small ' + escapar_tex(conteudo['caption']) + '}', '']
    linhas.append(r'\end{document}')
    Path(caminho).write_text('\n'.join(linhas) + '\n', encoding='utf-8')


def gerar(raiz=RAIZ):
    """Valida tudo antes de renderizar e publica somente arquivos completos."""
    import os
    import tempfile
    raiz = Path(raiz)
    programas, dados = coletar_dados(raiz)
    documento = criar_documento(programas, dados)
    with tempfile.TemporaryDirectory(prefix='.relatorio-', dir=raiz) as pasta:
        pasta = Path(pasta)
        renderizar_pdf(pasta / 'relatorio_ep1.pdf', documento)
        renderizar_tex(pasta / 'relatorio_ep1.tex', documento)
        for nome in ('relatorio_ep1.pdf', 'relatorio_ep1.tex'):
            os.replace(pasta / nome, raiz / nome)
    return dados


def main():
    try:
        gerar()
    except (OSError, ValueError, ImportError) as erro:
        print(f'Falha ao gerar relatório: {erro}', file=sys.stderr)
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main())
