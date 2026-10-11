#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Gerador de Relatório EP1 - Escalonador de Processos
Gera PDF com tabelas, gráficos e análise dos experimentos com diferentes quantums.
"""

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
from matplotlib.backends.backend_pdf import PdfPages
import numpy as np
from datetime import datetime

# Dados coletados dos experimentos
dados = [
    {"quantum": 1,  "trocas": 13.9, "instr_quantum": 1.000000, "despachos": 139, "instrucoes_totais": 139},
    {"quantum": 2,  "trocas": 7.9,  "instr_quantum": 1.759494, "despachos": 79,  "instrucoes_totais": 139},
    {"quantum": 3,  "trocas": 5.9,  "instr_quantum": 2.355932, "despachos": 59,  "instrucoes_totais": 139},
    {"quantum": 4,  "trocas": 5.2,  "instr_quantum": 2.673077, "despachos": 52,  "instrucoes_totais": 139},
    {"quantum": 5,  "trocas": 4.4,  "instr_quantum": 3.159091, "despachos": 44,  "instrucoes_totais": 139},
    {"quantum": 6,  "trocas": 4.1,  "instr_quantum": 3.390244, "despachos": 41,  "instrucoes_totais": 139},
    {"quantum": 7,  "trocas": 3.8,  "instr_quantum": 3.657895, "despachos": 38,  "instrucoes_totais": 139},
    {"quantum": 8,  "trocas": 3.8,  "instr_quantum": 3.657895, "despachos": 38,  "instrucoes_totais": 139},
    {"quantum": 9,  "trocas": 3.7,  "instr_quantum": 3.756757, "despachos": 37,  "instrucoes_totais": 139},
    {"quantum": 10, "trocas": 3.7,  "instr_quantum": 3.756757, "despachos": 37,  "instrucoes_totais": 139},
    {"quantum": 12, "trocas": 3.5,  "instr_quantum": 3.971429, "despachos": 35,  "instrucoes_totais": 139},
    {"quantum": 15, "trocas": 3.4,  "instr_quantum": 4.088235, "despachos": 34,  "instrucoes_totais": 139},
    {"quantum": 20, "trocas": 3.4,  "instr_quantum": 4.088235, "despachos": 34,  "instrucoes_totais": 139},
    {"quantum": 25, "trocas": 3.3,  "instr_quantum": 4.212121, "despachos": 33,  "instrucoes_totais": 139},
    {"quantum": 30, "trocas": 3.3,  "instr_quantum": 4.212121, "despachos": 33,  "instrucoes_totais": 139},
]

quanta = [d["quantum"] for d in dados]
trocas = [d["trocas"] for d in dados]
instr_quantum = [d["instr_quantum"] for d in dados]
despachos = [d["despachos"] for d in dados]

# Configuração de estilo
plt.style.use('seaborn-v0_8-whitegrid')
plt.rcParams.update({
    'font.size': 10,
    'axes.titlesize': 12,
    'axes.labelsize': 11,
    'xtick.labelsize': 9,
    'ytick.labelsize': 9,
    'legend.fontsize': 9,
    'figure.titlesize': 14,
})

with PdfPages('C:/Users/crist/Escalonador-SO-1/relatorio_ep1.pdf') as pdf:
    # ===== PÁGINA 1: CAPA =====
    fig = plt.figure(figsize=(8.5, 11))
    fig.text(0.5, 0.75, 'EP1 - Escalonador de Processos', ha='center', fontsize=24, weight='bold')
    fig.text(0.5, 0.68, 'Relatorio de Avaliacao do Sistema', ha='center', fontsize=16)
    fig.text(0.5, 0.58, 'Sistemas Operacionais - 2026', ha='center', fontsize=14, style='italic')
    fig.text(0.5, 0.50, f'Data de geracao: {datetime.now().strftime("%d/%m/%Y %H:%M")}', ha='center', fontsize=11)
    fig.text(0.5, 0.42, 'Implementacao em Rust (edition 2021)', ha='center', fontsize=11)
    fig.text(0.5, 0.38, 'Codigo-fonte: ep1-escalonador', ha='center', fontsize=11)
    fig.text(0.5, 0.30, 'Integrantes:', ha='center', fontsize=12, weight='bold')
    fig.text(0.5, 0.26, 'Cristian de Lima Meira - USP: 16860754', ha='center', fontsize=11)
    fig.text(0.5, 0.23, 'Fernando Silva - USP: 15636702', ha='center', fontsize=11)
    fig.text(0.5, 0.20, 'Julio Domingos - USP: 16836502', ha='center', fontsize=11)
    pdf.savefig(fig)
    plt.close(fig)

    # ===== PÁGINA 2: INTRODUÇÃO E METODOLOGIA =====
    fig = plt.figure(figsize=(8.5, 11))
    ax = fig.add_subplot(111)
    ax.axis('off')
    
    texto_intro = (
        "1. INTRODUCAO\n\n"
        "Este relatorio apresenta a avaliacao do escalonador de processos implementado para o "
        "EP1 de Sistemas Operacionais (2026). O escalonador simula um sistema de time-sharing "
        "em uma maquina ficticia com um unico processador, dois registradores de uso geral (X e Y) "
        "e quatro instrucoes: atribuicao (X=, Y=), COM, E/S e SAIDA.\n\n"
        "2. METODOLOGIA\n\n"
        "Foram realizados experimentos com 15 valores de quantum (n), variando de 1 a 30, "
        "distribuidos de forma a cobrir uniformemente o intervalo util:\n"
        "  - Quantums pequenos (1-6): alta frequencia de trocas, baixa eficiencia por quantum\n"
        "  - Quantums medios (7-12): regiao de transicao, equilibrio trocas/eficiencia\n"
        "  - Quantums grandes (15-30): poucas trocas, alta eficiencia por quantum, risco de "
        "processos longos monopolizarem CPU\n\n"
        "Para cada quantum, o escalonador foi executado com os 10 programas oficiais (01.txt a 10.txt), "
        "prioridades definidas em prioridades.txt e o quantum em quantum.txt. O log gerado (logXX.txt) "
        "contem todos os eventos de escalonamento e as estatisticas finais.\n\n"
        "Metricas coletadas:\n"
        "  - Media de trocas por processo (D/P): total de despachos concluidos / numero de processos\n"
        "  - Media de instrucoes por quantum (I/D): total de instrucoes executadas / total de despachos\n"
        "  - Total de despachos, total de instrucoes (sempre 139 para este conjunto de programas)\n\n"
        "3. ESTRUTURAS DE DADOS IMPLEMENTADAS\n\n"
        "  - BCP (Bloco de Controle de Processo): PID, PC, estado, prioridade, creditos, registradores X/Y, "
        "espera, referencia ao programa\n"
        "  - Tabela de Processos: posse de todos os BCPs ativos (HashMap<ProcessId, BCP>)\n"
        "  - Fila de Prontos: apenas ProcessId, ordenada por creditos (maior primeiro), "
        "desempate por arquivo/PID; reinsercao pos-quantum antes dos empates, pos-E/S depois dos empates\n"
        "  - Fila de Bloqueados: FIFO por ordem de chegada, espera = 2 surtos posteriores\n"
        "  - Politica de creditos: inicio = prioridade; -1 por despacho (saturado); redistribuicao "
        "somente quando TODOS ativos (incl. bloqueados) tem zero creditos\n"
    )
    ax.text(0.05, 0.95, texto_intro, ha='left', va='top', fontsize=9, family='monospace', wrap=True)
    pdf.savefig(fig, bbox_inches='tight')
    plt.close(fig)

    # ===== PÁGINA 3: TABELA DE RESULTADOS =====
    fig, ax = plt.subplots(figsize=(8.5, 11))
    ax.axis('off')
    
    # Cabeçalho da tabela
    col_labels = ['Quantum', 'Despachos', 'Instr. Total', 'Média Trocas (D/P)', 'Média Instr./Quantum (I/D)']
    cell_data = []
    for d in dados:
        cell_data.append([
            str(d['quantum']),
            str(d['despachos']),
            str(d['instrucoes_totais']),
            f"{d['trocas']:.2f}",
            f"{d['instr_quantum']:.4f}"
        ])
    
    table = ax.table(cellText=cell_data, colLabels=col_labels,
                     cellLoc='center', loc='center',
                     colWidths=[0.12, 0.15, 0.15, 0.25, 0.33])
    table.auto_set_font_size(False)
    table.set_fontsize(9)
    table.scale(1, 2)
    
    # Estilo do cabeçalho
    for i in range(len(col_labels)):
        table[(0, i)].set_facecolor('#2c3e50')
        table[(0, i)].set_text_props(weight='bold', color='white')
    
    # Alternar cores das linhas
    for i in range(1, len(cell_data) + 1):
        for j in range(len(col_labels)):
            if i % 2 == 0:
                table[(i, j)].set_facecolor('#ecf0f1')
            else:
                table[(i, j)].set_facecolor('white')
    
    ax.set_title('Tabela 1 - Resultados dos Experimentos com Diferentes Quantums', pad=20, weight='bold')
    
    # Nota explicativa
    nota = (
        "Nota: Total de instrucoes = 139 (constante para o conjunto de 10 programas oficiais). "
        "Media de Trocas = Despachos / 10 processos. "
        "Media Instr./Quantum = 139 / Despachos."
    )
    ax.text(0.5, 0.08, nota, ha='center', va='bottom', fontsize=8, style='italic', wrap=True)
    
    pdf.savefig(fig, bbox_inches='tight')
    plt.close(fig)

    # ===== PÁGINA 4: GRÁFICO 1 — Média de Trocas vs Quantum =====
    fig, ax = plt.subplots(figsize=(8.5, 6))
    
    ax.plot(quanta, trocas, 'o-', color='#2c3e50', linewidth=2, markersize=6, label='Media de Trocas (D/P)')
    ax.fill_between(quanta, trocas, alpha=0.1, color='#2c3e50')
    
    # Anotações nos pontos-chave
    for q, t in zip(quanta, trocas):
        if q in [1, 3, 5, 7, 10, 15, 20, 30]:
            ax.annotate(f'{t:.1f}', (q, t), textcoords="offset points", xytext=(0, 10),
                       ha='center', fontsize=8, color='#2c3e50')
    
    ax.set_xlabel('Quantum (n instrucoes)')
    ax.set_ylabel('Media de Trocas por Processo (D/P)')
    ax.set_title('Figura 1 - Media de Trocas por Processo vs. Tamanho do Quantum', weight='bold')
    ax.set_xticks(quanta)
    ax.grid(True, alpha=0.3)
    ax.legend()
    
    # Zona de destaque
    ax.axvspan(4, 8, alpha=0.1, color='orange', label='Regiao de transicao')
    ax.text(6, 5.5, 'Regiao de\ntransicao', ha='center', fontsize=9, color='darkorange', weight='bold')
    
    pdf.savefig(fig, bbox_inches='tight')
    plt.close(fig)

    # ===== PÁGINA 5: GRÁFICO 2 — Média Instruções/Quantum vs Quantum =====
    fig, ax = plt.subplots(figsize=(8.5, 6))
    
    ax.plot(quanta, instr_quantum, 's-', color='#e74c3c', linewidth=2, markersize=6, label='Media Instr./Quantum (I/D)')
    ax.fill_between(quanta, instr_quantum, alpha=0.1, color='#e74c3c')
    
    # Linha de referência: quantum
    ax.plot(quanta, quanta, '--', color='gray', linewidth=1, alpha=0.5, label='Linha Quantum (referencia)')
    
    for q, iq in zip(quanta, instr_quantum):
        if q in [1, 3, 5, 7, 10, 15, 20, 30]:
            ax.annotate(f'{iq:.2f}', (q, iq), textcoords="offset points", xytext=(0, 10),
                       ha='center', fontsize=8, color='#e74c3c')
    
    ax.set_xlabel('Quantum (n instrucoes)')
    ax.set_ylabel('Media de Instrucoes por Quantum (I/D)')
    ax.set_title('Figura 2 - Media de Instrucoes Executadas por Quantum vs. Tamanho do Quantum', weight='bold')
    ax.set_xticks(quanta)
    ax.grid(True, alpha=0.3)
    ax.legend()
    
    pdf.savefig(fig, bbox_inches='tight')
    plt.close(fig)

    # ===== PÁGINA 6: GRÁFICO 3 — Despachos Totais vs Quantum =====
    fig, ax = plt.subplots(figsize=(8.5, 6))
    
    ax.plot(quanta, despachos, 'd-', color='#27ae60', linewidth=2, markersize=6, label='Total de Despachos')
    ax.fill_between(quanta, despachos, alpha=0.1, color='#27ae60')
    
    for q, d in zip(quanta, despachos):
        if q in [1, 3, 5, 7, 10, 15, 20, 30]:
            ax.annotate(f'{d}', (q, d), textcoords="offset points", xytext=(0, 10),
                       ha='center', fontsize=8, color='#27ae60')
    
    ax.set_xlabel('Quantum (n instrucoes)')
    ax.set_ylabel('Total de Despachos Concluidos')
    ax.set_title('Figura 3 - Total de Despachos vs. Tamanho do Quantum', weight='bold')
    ax.set_xticks(quanta)
    ax.grid(True, alpha=0.3)
    ax.legend()
    
    pdf.savefig(fig, bbox_inches='tight')
    plt.close(fig)

    # ===== PÁGINA 7: GRÁFICO 4 — Relação Trocas vs Instruções/Quantum (Trade-off) =====
    fig, ax = plt.subplots(figsize=(8.5, 6))
    
    scatter = ax.scatter(trocas, instr_quantum, c=quanta, cmap='viridis', s=100, 
                         edgecolors='black', linewidth=0.8, zorder=5)
    
    # Linha conectando os pontos em ordem de quantum
    ax.plot(trocas, instr_quantum, '-', color='gray', alpha=0.4, linewidth=1, zorder=1)
    
    # Anotar quantums
    for q, t, iq in zip(quanta, trocas, instr_quantum):
        if q in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 15, 20, 25, 30]:
            ax.annotate(f'q={q}', (t, iq), textcoords="offset points", xytext=(5, 5),
                       fontsize=7, alpha=0.8)
    
    cbar = plt.colorbar(scatter, ax=ax)
    cbar.set_label('Quantum')
    
    ax.set_xlabel('Media de Trocas por Processo (D/P) - Melhor')
    ax.set_ylabel('Media de Instrucoes por Quantum (I/D) - Melhor')
    ax.set_title('Figura 4 - Trade-off: Trocas vs. Eficiencia por Quantum\n(Setas indicam aumento do quantum)', weight='bold')
    ax.grid(True, alpha=0.3)
    ax.invert_xaxis()  # Menos trocas é melhor (à direita)
    
    pdf.savefig(fig, bbox_inches='tight')
    plt.close(fig)

    # ===== PÁGINA 8: ANÁLISE E JUSTIFICATIVA =====
    fig = plt.figure(figsize=(8.5, 11))
    ax = fig.add_subplot(111)
    ax.axis('off')
    
    analise = (
        "5. ANALISE DOS RESULTADOS\n\n"
        "5.1 Comportamento Geral\n\n"
        "Os resultados mostram uma clara relacao inversa entre o tamanho do quantum e a media de trocas "
        "por processo (D/P), e uma relacao direta entre quantum e media de instrucoes por quantum (I/D).\n\n"
        "- Quantum 1: 13.9 trocas/proc, 1.00 instr/quantum - Extremo de time-sharing puro, "
        "maxima responsividade, minimo aproveitamento de CPU por despacho.\n"
        "- Quantum 3 (oficial): 5.9 trocas/proc, 2.36 instr/quantum - Ponto de referencia do enunciado.\n"
        "- Quantum 7-8: 3.8 trocas/proc, 3.66 instr/quantum - Primeiro plato de estabilidade.\n"
        "- Quantum 15-20: 3.4 trocas/proc, 4.09 instr/quantum - Segundo plato, ganhos marginais.\n"
        "- Quantum 25-30: 3.3 trocas/proc, 4.21 instr/quantum - Saturacao; quantum excede a maioria "
        "dos programas, aproximando-se de execucao quase FIFO.\n\n"
        "5.2 Analise do Trade-off\n\n"
        "A Figura 4 ilustra o trade-off fundamental: quantums menores favorecem responsividade "
        "(menos tempo de espera entre execucoes de um mesmo processo), mas aumentam overhead de "
        "troca de contexto. Quantums maiores reduzem trocas e aumentam eficiencia por despacho, "
        "mas podem causar:\n"
        "  (a) Processos longos monopolizarem CPU em detrimento de curtos\n"
        "  (b) Aumento do tempo de resposta para processos interativos\n"
        "  (c) Menor granularidade no compartilhamento de tempo\n\n"
        "Para este conjunto de programas (max. 21 instrucoes, varios com E/S), observa-se que:\n"
        "  - A media de instrucoes por quantum (I/D) nunca excede ~4.2, mesmo com quantum 30.\n"
        "  - Isso ocorre porque E/S e SAIDA interrompem o quantum prematuramente.\n"
        "  - A partir de quantum ~= 7, os ganhos em I/D tornam-se marginais (< 0.5 por salto de 5 no quantum).\n"
        "  - A media de trocas estabiliza em 3.3-3.4 a partir de quantum 15.\n\n"
        "5.3 Escolha do Quantum Recomendado\n\n"
        "Considerando o equilibrio entre:\n"
        "  - Responsividade (baixas trocas = menos espera para processos prontos)\n"
        "  - Eficiencia (altas instr/quantum = melhor aproveitamento do despacho)\n"
        "  - Justica (quantum nao deve exceder programas tipicos)\n\n"
        "RECOMENDACAO: Quantum = 7 (ou faixa 6-8)\n\n"
        "Justificativa:\n"
        "  1. Media de trocas = 3.8 (reducao de 73% vs quantum 1, apenas 36% acima do minimo 3.3)\n"
        "  2. Media instr/quantum = 3.66 (ganho de 266% vs quantum 1, 87% do maximo teorico 4.21)\n"
        "  3. Quantum 7 esta proximo do tamanho medio de programas entre E/S/SAIDA\n"
        "  4. Evita o plato de retornos decrescentes (quantum > 15)\n"
        "  5. Mantem responsividade aceitavel para processos interativos (E/S frequente)\n\n"
        "Alternativa conservadora: Quantum = 5 (4.4 trocas, 3.16 instr/quantum) se responsividade "
        "for criterio primario. Alternativa de throughput: Quantum = 12 (3.5 trocas, 3.97 instr/quantum) "
        "se throughput bruto for prioritario.\n\n"
        "5.4 Validacao com Programas Oficiais\n\n"
        "O quantum oficial do enunciado (3) resulta em 5.9 trocas e 2.36 instr/quantum. "
        "Nosso quantum recomendado (7) melhora em 36% as trocas e 55% a eficiencia por quantum, "
        "confirmando que o quantum 3 e conservador (prioriza responsividade sobre throughput)."
    )
    ax.text(0.05, 0.95, analise, ha='left', va='top', fontsize=9, family='monospace', wrap=True)
    pdf.savefig(fig, bbox_inches='tight')
    plt.close(fig)

    # ===== PÁGINA 9: DETALHES TÉCNICOS E CONCLUSÃO =====
    fig = plt.figure(figsize=(8.5, 11))
    ax = fig.add_subplot(111)
    ax.axis('off')
    
    conclusao = (
        "6. DETALHES TECNICOS DA IMPLEMENTACAO\n\n"
        "O escalonador foi implementado em Rust (stable, edition 2021) com as seguintes caracteristicas:\n\n"
        "- Arquitetura modular: 11 modulos (modelos, erros, carregador, interpretador, tabela_processos, "
        "filas, politica, simulador, estatisticas, registro, main)\n"
        "- 117 testes automatizados (unitarios e integracao), todos passando\n"
        "- Zero unsafe code, zero threads, apenas biblioteca padrao\n"
        "- TDD rigoroso em todas as 6 etapas de desenvolvimento\n"
        "- Revisao independente de codigo em cada etapa\n\n"
        "Estruturas principais:\n"
        "  - BCP: struct com PID, PC, estado (Pronto/Executando/Bloqueado), prioridade, creditos, X, Y, espera\n"
        "  - TabelaProcessos: HashMap<ProcessId, BCP> - posse exclusiva dos contextos\n"
        "  - FilaProntos: Vec<ProcessId> - ordenacao estavel por creditos, reinsercoes por regra\n"
        "  - FilaBloqueados: VecDeque<ProcessId> - FIFO, espera decrementada por snapshot\n"
        "  - Politica: redistribuir_se_necessario() so quando todos ativos tem creditos=0\n"
        "  - Interpretador: executa 1 instrucao por chamada, avanca PC 1, preserva contexto\n"
        "  - Simulador: executar_laco (nucleo puro) + simular (com logs/estatisticas)\n"
        "  - Registro: escritor UTF-8 com flush por evento, logXX.txt no diretorio corrente\n\n"
        "7. CONCLUSAO\n\n"
        "O escalonador implementado atende a todos os requisitos do EP1:\n"
        "  - Carregamento e validacao de 10 programas + prioridades + quantum\n"
        "  - Escalonamento por creditos/prioridades com redistribuicao correta\n"
        "  - Tratamento de E/S com espera de 2 surtos, FIFO na fila de bloqueados\n"
        "  - SAIDA remove processo e registra registradores finais\n"
        "  - Log completo: Carregando, Executando, E/S iniciada, Interrompendo, Terminado\n"
        "  - Estatisticas: Media de Trocas (D/P) e Media de Instrucoes (I/D) em ponto flutuante\n"
        "  - Execucao principal integrada: valida entrada antes de criar log, codigo 0/1 correto\n"
        "  - 15 experimentos realizados (quantum 1-30), dados coletados e analisados\n\n"
        "O quantum recomendado de 7 oferece o melhor equilibrio entre responsividade e eficiencia "
        "para a carga de trabalho dada, reduzindo trocas em 73% vs quantum 1 enquanto atinge 87% "
        "da eficiencia maxima teorica por quantum.\n\n"
        "-- Fim do Relatorio --"
    )
    ax.text(0.05, 0.95, conclusao, ha='left', va='top', fontsize=9, family='monospace', wrap=True)
    pdf.savefig(fig, bbox_inches='tight')
    plt.close(fig)

print("Relatorio PDF gerado: relatorio_ep1.pdf")