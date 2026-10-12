"""Contratos do gerador: importação leve, dados reais e recusa de adulteração."""
import subprocess
import shutil
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))
import gerar_relatorio as relatorio


class ImportacaoTests(unittest.TestCase):
    def test_importacao_nao_exige_renderizador_nem_escreve_arquivos(self):
        # Captura importação prematura de Matplotlib/ReportLab e geração automática.
        with tempfile.TemporaryDirectory() as pasta:
            codigo = (
                f"import sys; sys.path.insert(0, {str(ROOT)!r}); "
                "import gerar_relatorio; "
                "assert 'reportlab' not in sys.modules"
            )
            resultado = subprocess.run([sys.executable, '-c', codigo], cwd=pasta,
                                       capture_output=True, text=True)
            self.assertEqual(resultado.returncode, 0, resultado.stderr)
            self.assertEqual(list(Path(pasta).iterdir()), [])


class FixtureEntradas:

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.raiz = Path(self.tmp.name)
        pasta = self.raiz / 'programas'
        pasta.mkdir()
        (pasta / '01.txt').write_text('A\nX=2\nE/S\nY=3\nSAIDA\n')
        (pasta / '02.txt').write_text('B\nCOM\nSAIDA\n')
        (pasta / 'prioridades.txt').write_text('2\n1\n')
        (pasta / 'quantum.txt').write_text('2\n')

    def carregar(self):
        try:
            return relatorio.carregar_entradas(self.raiz)
        except NotImplementedError as erro:
            self.fail(str(erro))


class EntradasTests(FixtureEntradas, unittest.TestCase):
    def test_carrega_instrucoes_e_prioridades_na_ordem_dos_arquivos(self):
        programas = self.carregar()
        self.assertEqual([(p.nome, p.prioridade) for p in programas], [('A', 2), ('B', 1)])
        self.assertEqual(programas[0].instrucoes, ('X=2', 'E/S', 'Y=3', 'SAIDA'))

    def test_recusa_prioridade_sem_programa(self):
        (self.raiz / 'programas/prioridades.txt').write_text('2\n')
        with self.assertRaises(ValueError):
            self.carregar()

    def test_recusa_instrucao_e_saida_prematura(self):
        for corpo in ('X =2\nSAIDA\n', 'SAIDA\nCOM\n'):
            with self.subTest(corpo=corpo):
                (self.raiz / 'programas/01.txt').write_text('A\n' + corpo)
                with self.assertRaises(ValueError):
                    self.carregar()

    def test_recusa_registrador_fora_do_intervalo_i64(self):
        (self.raiz / 'programas/01.txt').write_text('A\nX=9223372036854775808\nSAIDA\n')
        with self.assertRaises(ValueError):
            self.carregar()

    def test_recusa_programa_com_22_instrucoes(self):
        (self.raiz / 'programas/01.txt').write_text('A\n' + 'COM\n' * 21 + 'SAIDA\n')
        with self.assertRaises(ValueError):
            self.carregar()

    def test_recusa_nomes_ambiguos_para_reconstituir_log(self):
        (self.raiz / 'programas/02.txt').write_text('A\nSAIDA\n')
        with self.assertRaises(ValueError):
            self.carregar()

    def test_nao_interpreta_separadores_adicionais_como_linhas(self):
        # Rust mantém estes caracteres dentro da instrução, que é inválida.
        for separador in ('\x0b', '\x0c', '\r', '\x85', '\u2028', '\u2029'):
            with self.subTest(separador=repr(separador)):
                (self.raiz / 'programas/01.txt').write_bytes(
                    ('A\nCOM' + separador + 'SAIDA\n').encode('utf-8'))
                with self.assertRaises(ValueError):
                    self.carregar()

    def test_crlf_aceito_e_cr_final_sem_lf_preservado(self):
        (self.raiz / 'programas/01.txt').write_bytes(b'A\r\nX=2\r\nE/S\r\nY=3\r\nSAIDA\r\n')
        self.assertEqual(self.carregar()[0].instrucoes, ('X=2', 'E/S', 'Y=3', 'SAIDA'))
        (self.raiz / 'programas/quantum.txt').write_bytes(b'2\r')
        with self.assertRaises(ValueError):
            self.carregar()

    def test_recusa_quantum_oficial_invalido(self):
        (self.raiz / 'programas/quantum.txt').write_text('0\n')
        with self.assertRaises(ValueError):
            self.carregar()


LOG_Q2 = """Carregando A
Carregando B
Executando A
E/S iniciada em A
Interrompendo A após 2 instruções
Executando B
B terminado. X=0. Y=0
Executando A
A terminado. X=2. Y=3
MEDIA DE TROCAS: 1.5
MEDIA DE INSTRUCOES: 2
QUANTUM: 2
"""


class LogsTests(FixtureEntradas, unittest.TestCase):
    def validar(self, texto=LOG_Q2, quantum=2):
        try:
            return relatorio.validar_log(texto, self.carregar(), quantum)
        except NotImplementedError as erro:
            self.fail(str(erro))

    def test_reconstitui_pc_registradores_e_media_real(self):
        dado = self.validar()
        self.assertEqual((dado.quantum, dado.processos, dado.despachos, dado.instrucoes),
                         (2, 2, 3, 6))
        self.assertEqual((dado.trocas, dado.instr_quantum), (1.5, 2.0))
        self.assertEqual(dado.registradores, {'A': (2, 3), 'B': (0, 0)})

    def test_recusa_resumo_falso_nan_e_quantum_incorreto(self):
        for original, falso in [('TROCAS: 1.5', 'TROCAS: 1'),
                                ('INSTRUCOES: 2', 'INSTRUCOES: nan'),
                                ('QUANTUM: 2', 'QUANTUM: 1')]:
            with self.subTest(falso=falso), self.assertRaises(ValueError):
                self.validar(LOG_Q2.replace(original, falso))

    def test_recusa_contagem_adulterada_e_evento_es_ausente(self):
        for texto in [LOG_Q2.replace('após 2', 'após 1'),
                      LOG_Q2.replace('E/S iniciada em A\n', '')]:
            with self.subTest(texto=texto), self.assertRaises(ValueError):
                self.validar(texto)

    def test_recusa_registradores_finais_incorretos(self):
        with self.assertRaises(ValueError):
            self.validar(LOG_Q2.replace('X=2. Y=3', 'X=2. Y=0'))

    def test_recusa_incompleto_duplicacao_e_evento_apos_resumo(self):
        for texto in [LOG_Q2.replace('A terminado. X=2. Y=3\n', ''),
                      LOG_Q2 + 'Executando A\n',
                      LOG_Q2.replace('Executando B', 'Executando A')]:
            with self.subTest(texto=texto), self.assertRaises(ValueError):
                self.validar(texto)

    def test_es_e_saida_no_limite_contam_sem_interrupcao_extra(self):
        (self.raiz / 'programas/01.txt').write_text('A\nX=-2\nSAIDA\n')
        (self.raiz / 'programas/02.txt').unlink()
        (self.raiz / 'programas/prioridades.txt').write_text('2\n')
        texto = ('Carregando A\nExecutando A\nA terminado. X=-2. Y=0\n'
                 'MEDIA DE TROCAS: 1\nMEDIA DE INSTRUCOES: 2\nQUANTUM: 2\n')
        dado = self.validar(texto)
        self.assertEqual((dado.despachos, dado.instrucoes), (1, 2))
        with self.assertRaises(ValueError):
            self.validar(texto.replace('A terminado.',
                         'Interrompendo A após 2 instruções\nA terminado.'))

    def test_repeticao_pid_e_espera_ociosa_nao_contam_despachos(self):
        (self.raiz / 'programas/01.txt').write_text('A\nE/S\nSAIDA\n')
        (self.raiz / 'programas/02.txt').unlink()
        (self.raiz / 'programas/prioridades.txt').write_text('0\n')
        texto = ('Carregando A\nExecutando A\nE/S iniciada em A\n'
                 'Interrompendo A após 1 instrução\nExecutando A\n'
                 'A terminado. X=0. Y=0\nMEDIA DE TROCAS: 2\n'
                 'MEDIA DE INSTRUCOES: 1\nQUANTUM: 2\n')
        dado = self.validar(texto)
        self.assertEqual((dado.despachos, dado.instrucoes), (2, 2))

    def test_log_crlf_aceito_sem_engolir_separadores_adicionais(self):
        dado = self.validar(LOG_Q2.replace('\n', '\r\n'))
        self.assertEqual((dado.despachos, dado.instrucoes), (3, 6))
        for separador in ('\x0b', '\x0c', '\r', '\x85', '\u2028'):
            with self.subTest(separador=repr(separador)), self.assertRaises(ValueError):
                self.validar(LOG_Q2.replace('Carregando A\n', 'Carregando A' + separador))

    def test_coleta_exige_colecoes_identicas(self):
        (self.raiz / 'experimentos').mkdir()
        (self.raiz / 'log02.txt').write_text(LOG_Q2)
        (self.raiz / 'experimentos/log02.txt').write_text(LOG_Q2 + '\n')
        try:
            with self.assertRaises(ValueError):
                relatorio.coletar_dados(self.raiz, (2,))
        except NotImplementedError as erro:
            self.fail(str(erro))

    def test_coleta_extrai_eventos_sem_resultados_estaticos(self):
        (self.raiz / 'experimentos').mkdir()
        (self.raiz / 'log02.txt').write_text(LOG_Q2)
        (self.raiz / 'experimentos/log02.txt').write_text(LOG_Q2)
        try:
            programas, dados = relatorio.coletar_dados(self.raiz, (2,))
        except NotImplementedError as erro:
            self.fail(str(erro))
        self.assertEqual(len(programas), 2)
        self.assertEqual((dados[0].despachos, dados[0].instrucoes), (3, 6))


class PublicacaoTests(unittest.TestCase):
    def test_log_final_invalido_preserva_saidas_e_cwd_diferente(self):
        # Captura gerar antes de validar todos os logs ou usar cwd como raiz.
        with tempfile.TemporaryDirectory() as pasta, tempfile.TemporaryDirectory() as cwd:
            raiz = Path(pasta)
            shutil.copy(ROOT / 'gerar_relatorio.py', raiz)
            (raiz / 'programas').mkdir()
            (raiz / 'experimentos').mkdir()
            (raiz / 'programas/01.txt').write_text('A\nSAIDA\n')
            (raiz / 'programas/prioridades.txt').write_text('1\n')
            (raiz / 'programas/quantum.txt').write_text('1\n')
            for quantum in relatorio.QUANTUMS:
                texto = ('Carregando A\nExecutando A\nA terminado. X=0. Y=0\n'
                         f'MEDIA DE TROCAS: 1\nMEDIA DE INSTRUCOES: 1\nQUANTUM: {quantum}\n')
                if quantum == 30:
                    texto = texto.replace('X=0', 'X=9')
                for pai in (raiz, raiz / 'experimentos'):
                    (pai / f'log{quantum:02}.txt').write_text(texto)
            for extensao in ('pdf', 'tex'):
                (raiz / f'relatorio_ep1.{extensao}').write_bytes(b'preservar')
            resultado = subprocess.run([sys.executable, str(raiz / 'gerar_relatorio.py')],
                                       cwd=cwd, capture_output=True, text=True)
            self.assertEqual(resultado.returncode, 1, resultado.stderr)
            self.assertIn('log30.txt', resultado.stderr)
            for extensao in ('pdf', 'tex'):
                self.assertEqual((raiz / f'relatorio_ep1.{extensao}').read_bytes(), b'preservar')
            self.assertEqual(list(Path(cwd).iterdir()), [])


class RecorteRelatorioTests(unittest.TestCase):
    def test_modelo_recusa_carga_legitima_fora_do_recorte_oficial(self):
        programas = [relatorio.Programa('01.txt', 'A', 1, ('SAIDA',))]
        dados = [relatorio.Medicao(q, 1, 1, 1, {'A': (0, 0)}) for q in relatorio.QUANTUMS]
        with self.assertRaises(ValueError):
            relatorio.criar_documento(programas, dados)

    def test_origem_divergente_recusada_antes_de_publicar(self):
        # Mesma dinâmica, nomes e origem diferentes: não é a carga oficial.
        with tempfile.TemporaryDirectory() as pasta:
            raiz = Path(pasta)
            shutil.copy(ROOT / 'gerar_relatorio.py', raiz)
            shutil.copytree(ROOT / 'programas', raiz / 'programas')
            (raiz / 'experimentos').mkdir()
            arquivo = raiz / 'programas/01.txt'
            arquivo.write_bytes(arquivo.read_bytes().replace(b'TESTE-1\n', b'OUTRO\n', 1))
            for q in relatorio.QUANTUMS:
                nome = f'log{q:02}.txt'
                conteudo = (ROOT / nome).read_bytes().replace(b'TESTE-1 ', b'OUTRO ').replace(b'TESTE-1\n', b'OUTRO\n')
                for pai in (raiz, raiz / 'experimentos'):
                    (pai / nome).write_bytes(conteudo)
            for extensao in ('pdf', 'tex'):
                (raiz / f'relatorio_ep1.{extensao}').write_bytes(b'preservar')
            resultado = subprocess.run([sys.executable, str(raiz / 'gerar_relatorio.py')],
                                       cwd=raiz, capture_output=True, text=True)
            self.assertEqual(resultado.returncode, 1, resultado.stderr)
            self.assertIn('carga oficial', resultado.stderr)
            for extensao in ('pdf', 'tex'):
                self.assertEqual((raiz / f'relatorio_ep1.{extensao}').read_bytes(), b'preservar')


class CriterioTests(unittest.TestCase):
    def test_escolhe_menor_quantum_que_alcanca_limiar(self):
        dados = [relatorio.Medicao(6, 10, 41, 139, {}),
                 relatorio.Medicao(8, 10, 38, 139, {}),
                 relatorio.Medicao(7, 10, 38, 139, {})]
        try:
            escolhido = relatorio.selecionar_quantum(dados, 139 / 33)
        except (AttributeError, NotImplementedError):
            self.fail('seleção pelo critério ainda ausente')
        self.assertEqual(escolhido.quantum, 7)


if __name__ == '__main__':
    unittest.main()
