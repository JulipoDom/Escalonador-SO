# Correções da revisão do EP1 — plano de execução

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development. Execução delegada por tarefas, com revisão independente e verificação final.

**Goal:** corrigir os achados da revisão autorizada pelo usuário e entregar PDF, fonte e documentação coerentes com os dados reais.
**Architecture:** manter o núcleo e os logs oficiais; publicar um arquivo novo para o pathname de cada log, evitando truncar aliases. O gerador valida os eventos contra os programas e calcula seus dados antes de escrever PDF e TEX a partir do mesmo conteúdo. Documentação registra estado e verificações reais.
**Tech Stack:** Rust stable edição 2021, Cargo e somente std no escalonador; Python std e ReportLab para o PDF, com dependência documentada; LaTeX autossuficiente para edição/preview.
**Spec:** enunciado `ep1.pdf`, pedido de correção do usuário e parecer `/home/fernando/.codex/visualizations/2026/10/11/01a12d62-4f5a-7e60-b8b8-47723aa1f681/revisao-ep1-2026-10-11.md`.

## Global Constraints

- Preservar byte a byte os 12 arquivos oficiais, `ep1.pdf`, os 30 logs versionados e o acervo `docs/fluxo/`.
- Sem CLI configurável, lote automático ou CSV no escalonador; sem unsafe, threads ou tempo real.
- Escrever cada evento no log visível e propagar falhas, sem resumo de sucesso após erro.
- Um autor por arquivo; correções voltam ao autor. Paralelizar somente tarefas independentes, conforme AGENTS.
- Manter q7 como recomendação fundamentada para esta carga; não alegar ótimo universal, responsividade aceitável, throughput ou custo real não medidos.
- Fonte única para conteúdo/dados do relatório: o gerador produz PDF e TEX coerentes; dados extraídos/validados dos logs, sem nova simulação embutida no gerador.
- Executar correções no checkout atual limpo, em branch local `codex/correcoes-relatorio-ep1`; inicialmente sem operações Git; o usuário agora autorizou commit e publicação apenas desta branch, mantendo a main intacta.

## Review Focus

- Log preexistente como symlink, hardlink ou diretório: preservar o alvo e informar falhas reais sem truncá-lo.
- Logs com resumo incorreto, instruções faltantes, evento fora da ordem, interrupção falsa ou registradores finais incorretos: recusar antes de sobrescrever saídas.
- Execução do gerador a partir de outro diretório: resolver entradas e saídas a partir do arquivo do projeto.
- Argumentação: distinguir contagem D do EP de troca física de contexto e R=(I/D)/q de utilização da CPU real.
- PDF/TEX: conteúdo e números alinhados, tabelas legíveis e gráficos sem labels sobrepostos.

### Task 1: Preservação na publicação dos logs

**Files:** `src/registro.rs`, `tests/registro_estatisticas.rs`, `tests/execucao.rs`, `tests/logs_simulacao.rs` se necessário. Autor: saída/execução.
**Interfaces:** manter `RegistroLog::criar`, `registrar`, `escrever_resumo` e `finalizar` sem mudar a API pública.

- [x] Escrever regressões antes da correção: log03 ligado a entrada, outro log e hardlink; preservar bytes do alvo e criar log normal visível a cada evento. Destino diretório deve falhar sem alteração.
- [x] Executar testes e registrar RED real sobre o comportamento atual.
- [x] Criar arquivo temporário exclusivo no mesmo diretório e renomeá-lo para o destino antes dos eventos; não abrir/truncar o alvo preexistente. Limpar temporário quando a publicação falhar.
- [x] Adaptar testes `/dev/full` ao contrato seguro: link de destino pode ser substituído por arquivo regular; manter testes reais de falha de escrita/flush usando arquivo `/dev/full` em teste interno do escritor, sem API de teste na produção.
- [x] Rodar testes específicos, suíte completa e formatação restrita aos arquivos próprios; relatório com comandos, resultados e limitações, sem commit.

### Task 2: Relatório e geração verificável

**Files:** `gerar_relatorio.py`, `relatorio_ep1.tex`, `relatorio_ep1.pdf`, `tests/test_relatorio.py`, `requirements-relatorio.txt`. Autor: relatório.
**Interfaces:** gerador importável sem produzir saídas ou importar renderizador; execução `python3 gerar_relatorio.py` usa raiz do próprio script. Funções públicas de carga/validação e coleta de dados testáveis com std unittest. Renderizador ReportLab importado somente na geração. PDF e TEX gerados da mesma representação de seções, tabelas e gráficos.

- [x] TDD para extração real e recusa de logs adulterados; testes pequenos calculados à mão, erro de resumo/registradores/contagem, e execução a partir de cwd diferente. Nenhum teste deve exigir modificar entradas oficiais.
- [x] Validar todas entradas e todos logs antes de publicar PDF/TEX; comparar coleções raiz/experimentos e usar os eventos para recompor PC, X/Y e instruções por surto. Remover números estáticos como fonte de resultados.
- [x] Explicar série principal uniforme 1..10 e exploração complementar 12,15,20,25,30, justificando limite de 21 instruções e redundância dos pontos de saturação.
- [x] Corrigir percentual para15,15%; média natural139/33=4,2121 e saturação q21. Substituir falso trade-off por gráfico da razão R=(I/D)/q. Explicar D/P × I/D=13,9.
- [x] Definir q7 como menor quantum testado a alcançar pelo menos85% da média máxima da carga, um critério prático escolhido pelos autores, não ótimo universal. q6 atinge80,49%,q7 86,84%; q8 não melhora D/I e aumenta o limite de ocupação. Explicar custo da escolha:15,15% mais despachos que mínimo, contra72,66% menos queq1 e35,59% menos queq3.
- [x] Se incorporar resposta ao primeiro despacho, recalcular em instruções lógicas a partir dos logs e rotular como medida complementar; não usar tempo real nem inferir responsividade geral. Sem gráfico de pontos dependentes com labels repetidos.
- [x] Discutir explicitamente desempate mantendo processo (salvamento/carregamento do contexto evitado em máquina real) e convenção de D do EP inclusive PID repetido; manter créditos ativos em quantum grande.
- [x] Eliminar linguagem de TDD/revisões históricas não verificável como prova do algoritmo; nomes e números USP fornecidos preservados. Manter narrativa técnica concisa, fonte coerente, sem data fictícia de submissão ou alegação de atendimento integral sem ressalvas.
- [x] Documentar ReportLab em requirements; usar runtime disponível. Executar marcador PDF antes da primeira edição/autoria do PDF, renderizar todas páginas e revisar layout. Devolver resultados e caminho do relatório de implementação.

### Task 3: Documentação vigente

**Files:** `README.md`, `HISTORICO.md`, `docs/superpowers/README.md`, `docs/superpowers/plans/2026-10-10-contexto-atual.md`. Autor: documentação. Coordenador: comentário de `src/lib.rs` e plano presente.

- [x] Corrigir afirmações de etapa6 sem commit, main anterior e inexistência de experimentos/relatório, usando commits existentes bf093f1 e cb6d5f6.
- [x] Registrar correções locais sem commit e geração oficial via Python/ReportLab; TEX é gerado do mesmo conteúdo e editável no editor, regeneração substitui ambos.
- [x] Descrever instruções de geração/testes e preservar a exigência apenas std do binário Rust; documentar diretório corrente do binário e raiz do script para gerador.
- [x] Atualizar próximos passos para revisão final/empacotamento e situação administrativa do prazo como pendência a confirmar, sem inventar submissão.
- [x] Incorporar somente resultados finais realmente enviados pelo coordenador; não usar contagens antecipadas.

### Task 4: Integração e revisão

- [x] Revisão independente de cada tarefa sobre diff e requisitos; encaminhar achados aos respectivos autores.
- [x] Cessar edições antes de checks globais: cargo check --all-targets, cargo test, cargo fmt --check, cargo clippy --all-targets -- -D warnings, git diff --check; executar unittest do gerador.
- [x] Reexecutar15 quantums em cópias temporárias e comparar byte a byte com logs versionados; comparar entradas/protegidos com HEAD.
- [x] Abrir TEX no editor e usar compilador integrado; corrigir erros nos limites da ferramenta. PDF de entrega gerado pelo gerador documentado; inspecionar todas páginas após última geração.
- [x] Revisão final do conjunto, estados/documentação atualizados e PDF apresentado ao usuário; sem operações Git de publicação.

Estado de verificação: Rust127/0/0 e Python24/0/0; checks e QA PDF7páginas aprovados. Compilação TEX tentada no editor e não verificada por falta do bundle/cache e falha de download, sem diagnóstico de fonte. Revisão final do conjunto aprovada em conformidade e qualidade, sem achados pendentes; correções publicadas no commit `884d7f5` em `origin/codex/correcoes-relatorio-ep1`; sem merge na main.

Autorização de integração Git: o usuário pediu commit das correções e push de `codex/correcoes-relatorio-ep1`, sem merge e sem alterar `main`. Verificar o commit e o ref remoto antes de registrar publicação concluída.

Publicação verificada: commit `884d7f5` (`fix: correct EP1 report and preserve log targets`) criado com 17 arquivos e publicado em `origin/codex/correcoes-relatorio-ep1` (push código 0). A consulta aos refs remotos confirmou esse commit na branch e `main` em `cb6d5f6`; a main local e origin/main também permanecem em `cb6d5f6`. Esta atualização documental registra fatos da publicação já concluída, sem antecipar seu próprio commit.
