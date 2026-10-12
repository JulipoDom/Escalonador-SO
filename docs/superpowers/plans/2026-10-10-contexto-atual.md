# Contexto atual — estado e próximas etapas

> **Para agentes:** consultar o pedido autorizado, o [plano das correções](2026-10-11-correcoes-revisao-relatorio.md) e o [HISTORICO](../../../HISTORICO.md) antes de implementar. Usar execução delegada e revisão independente conforme [AGENTS](../../../AGENTS.md), com um autor por arquivo. As verificações de etapas anteriores não substituem as da entrega atual.

**Base:** `cb6d5f6`; branch local `codex/correcoes-relatorio-ep1`. A sexta etapa está registrada em `bf093f1`, integrada à `main` e publicada conforme o histórico. Os experimentos e o relatório existem desde `cb6d5f6`. A correção atual foi autorizada, implementada, verificada e revisada localmente. O commit `884d7f5` registra as correções e foi publicado somente em `origin/codex/correcoes-relatorio-ep1`, com push de código 0. Não houve merge; `main` local/remota permaneceu em `cb6d5f6`. Este registro documental acompanha a publicação das correções na mesma branch; identifica o commit `884d7f5`, sem atribuir um hash ao próprio registro.

**Arquitetura:** a tabela possui os BCPs e as filas guardam somente `ProcessId`. Interpretador, quantum, política de créditos e operações de E/S são consumidos pelo passo e por `executar_laco` até a conclusão. `simular` integra eventos, estatísticas e registradores finais reais; `main` carrega e valida entradas antes de publicar o log no diretório corrente.

**Tecnologia:** Rust stable, edição 2021, Cargo e somente `std` no escalonador. O gerador de relatório usa Python e ReportLab e também produz TEX do mesmo conteúdo.

**Fontes:** [enunciado de 2026](../../../ep1.pdf), [README](../../../README.md), [AGENTS](../../../AGENTS.md), [HISTORICO](../../../HISTORICO.md) e [índice de planos](../README.md). O enunciado de 2026 prevalece sobre documentos antigos.

## Estado e contratos vigentes

- As seis etapas do núcleo e da execução principal estão implementadas. Seus commits, testes e revisões históricos estão no HISTORICO e nos planos específicos; a execução integrada está em `bf093f1`.
- Cada instrução, inclusive E/S e SAIDA, avança o PC uma vez e conta em I. E/S e SAIDA prevalecem sobre o limite do quantum. Erros posteriores preservam o progresso válido; quantum zero e PC inválido são recusados conforme os contratos da CPU.
- Créditos são descontados por despacho sem ficar negativos. Redistribuir somente quando todos os ativos, inclusive bloqueados, têm zero. Reinserção após quantum precede empates; retorno de E/S fica depois dos iguais; exceções de créditos zero usam FIFO.
- E/S espera dois surtos posteriores; novos bloqueados não perdem espera no próprio surto. Sem prontos, avança somente a espera lógica. Erro da CPU após despacho deixa Executando fora das filas, sem recuperação automática.
- `executar_laco` termina sem limite artificial e devolve os registradores finais reais; entrada vazia com quantum positivo devolve mapa vazio. `simular` recusa entrada vazia pelos denominadores, conta D/P e I/D e publica eventos e resumo reais.
- Sucesso de `main` exige publicação final, com código 0 e streams vazios; erros retornam código 1 e diagnóstico em stderr. Eventos reais parciais podem permanecer após erro. Entradas e log são relativos ao diretório corrente do binário.
- Há 15 quantums nos logs da raiz e de `experimentos/`: série uniforme principal 1…10 e exploração complementar 12,15,20,25,30. A carga tem 139 instruções, 33 segmentos e saturação em q21.
- A fonte única de conteúdo e dados do relatório é `gerar_relatorio.py`, que valida programas e ambas as coleções de logs antes de produzir PDF e TEX. Resolve entradas/saídas pela raiz do script; não executa novos experimentos nem roda o binário Rust. ReportLab está documentado em `requirements-relatorio.txt`; o escalonador continua sem dependências externas.
- A geração do relatório é restrita à identidade semântica original dos dez programas (arquivo, nome, prioridade e instruções), aos 15 quantums previstos e à escolha compatível com q7; divergência é recusada antes de renderizar ou publicar. Carregamento, validação e coleta seguem genéricos para testes. UTF-8 é lido por bytes com linhas LF/CRLF conforme Rust. `quantum.txt` aceita valor positivo; q3 identifica a entrada original fornecida, não a configuração atual obrigatória.
- O TEX é editável no editor LaTeX; a regeneração pelo script substitui PDF e TEX. Alterações duráveis do conteúdo devem ser feitas no gerador. O PDF gerado pelo script é o artefato de entrega.

## Correções autorizadas e limites

O [plano vigente de correções](2026-10-11-correcoes-revisao-relatorio.md) cobre publicação segura de logs preexistentes como links, geração verificável do relatório e atualização documental. q7 é uma recomendação prática para esta carga: menor quantum testado que atinge pelo menos 85% da média máxima de instruções por despacho. Não afirmar ótimo universal, throughput, custo real ou responsividade geral não medidos; distinguir D do EP de troca física de contexto e `(I/D)/q` de utilização real da CPU.

Preservar byte a byte as 12 entradas oficiais, `ep1.pdf`, os 30 logs versionados e o acervo `docs/fluxo/`. Sem CLI configurável, lote automático, CSV, `unsafe`, threads ou tempo real no escalonador. O [plano antigo](2026-10-10-boilerplate.md) e `docs/fluxo/` são acervo de `58835cb`, anterior a `ffc8849`; não reativar suas funcionalidades.

## Método e verificação

- Ler as skills antes de aplicá-las; planejamento proporcional com `writing-plans` e `brainstorming` quando houver decisões abertas. Informar limitações reais de ferramentas.
- Executar com `subagent-driven-development`; paralelizar somente tarefas independentes. Coordenador integra; um autor por arquivo; correções voltam ao autor. Até três especialistas simultâneos mais o coordenador.
- Para comportamentos implementados, usar TDD com falhas reais; diante de falhas inesperadas, usar depuração sistemática. Revisão independente deve preceder conclusão; registrar resultados e limitações reais no HISTORICO.
- Após cessarem edições, executar `cargo check --all-targets`, `cargo test`, `cargo fmt --check`, Clippy quando disponível e `git diff --check`. Não formatar globalmente durante edição concorrente.
- Executar `python3 -m unittest discover -s tests -p 'test_relatorio.py'` e gerar o relatório com `python3 gerar_relatorio.py`; dependência e instruções de instalação estão no README. Conferir execução do gerador de outro diretório e recusa de logs inválidos antes de substituir saídas.
- Reexecutar os 15 quantums em cópias temporárias e comparar logs byte a byte; conferir entradas, PDF do enunciado e acervo contra a base. Não gerar logs experimentais na raiz durante essas verificações.
- Inspecionar todas as páginas do PDF após a última geração, abrir TEX no editor e conferir compilação integrada. Registrar o resultado real, inclusive limitações ou falhas.

## Verificações desta correção

O coordenador aprovou check, fmt, Clippy, build e a suíte Rust com 127 aprovados, zero falhas e zero ignorados; seis testes de lib e 121 de integração, zero testes de main/doctests. As 15 reexecuções oficiais em cópias temporárias tiveram código 0, streams vazios e logs idênticos às duas coleções; cópias e 54 arquivos protegidos preservados, sem temporários residuais. Python: 24 testes aprovados, zero falhas/ignorados, py_compile e geração de PDF/TEX a partir de `/tmp` aprovados. Todas as sete páginas A4 do PDF foram rasterizadas e inspecionadas, com fontes Vera Roman/Bold efetivamente usadas incorporadas e gráficos sem sobreposição. `git diff --check` aprovado. Revisão da publicação de logs e revisão documental do snapshot anterior aprovadas em conformidade e qualidade, sem achados; reavaliação do relatório e revisão independente final aprovadas em conformidade e qualidade, sem achados pendentes.

O TEX está aberto no editor, mas a compilação integrada não foi verificada: o bundle Tectonic v33 não estava no cache e o download pelo relay falhou, sem diagnóstico de erro de fonte. O PDF ReportLab final foi verificado independentemente dessa infraestrutura.

## Próximos passos

- [x] Registrar as correções em `884d7f5` e publicar somente `codex/correcoes-relatorio-ep1`, sem merge e com `main` intacta.

- [x] Executar verificações da implementação, geração e PDF; resultados registrados no HISTORICO.
- [x] Concluir revisão independente final das correções autorizadas: conformidade e qualidade aprovadas, sem achados pendentes.
- [ ] Conferir o material final e preparar o ZIP com código, logs e PDF conforme o enunciado, após autorização para empacotamento.
- [ ] Confirmar com a disciplina a situação administrativa do prazo de 04/10/2026 e eventual submissão; nenhuma submissão está registrada nesta entrega.

Novas capacidades e operações Git além do commit/push da branch autorizada exigem pedido próprio. Os relatos históricos no HISTORICO descrevem o estado de cada entrega à época; não são pendências atuais.
