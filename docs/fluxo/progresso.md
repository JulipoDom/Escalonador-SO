# Registro — plano: docs/superpowers/plans/2026-10-10-boilerplate.md

Skills lidas: using-superpowers, brainstorming, writing-plans, subagent-driven-development, dispatching-parallel-agents, test-driven-development, using-git-worktrees, requesting-code-review, verification-before-completion. Depuração sistemática e recebimento de revisão serão lidos integralmente quando aplicáveis.

Decisão: projeto em ep1-escalonador/, conforme seção 5. Custo se errado: mover pasta. Usuário pediu executar arquitetura existente; não reabrir aprovação. Sem Git: adaptar scripts, commits e revisão por SHAs para arquivos e registro persistente. Custo: sem histórico Git desta sessão. Paralelismo autorizado com responsáveis exclusivos.

Pré-flight: Task 2 produz modelos para Tasks 3–7; Task 4 produz tabela para política/CPU; Task 6 produz RegistroLog para Task 5/7; Task 3 produz carregador para Task 7. Contratos seção 6 prevalecem. Tasks 1–9 consistentes com escopo; implementações de política/CPU/métricas continuam pendentes. Task 8 só lê APIs, sem conflito de arquivos.

| Tarefas | Interface compartilhada | Conferência |
|---|---|---|
| 2 → 3,4,5,6,7 | modelos/erros seção 6 | consumidores usam os mesmos campos/tipos |
| 4 → 5 | TabelaProcessos | propriedade de BCP só na tabela |
| 6 → 5,7 | RegistroLog | assinatura simular recebe &mut RegistroLog |
| 3 → 7 | carregar_programas, ler_quantum | caminhos &Path e Result conforme contrato |
| 5 → 7 | simular pendente | CLI deve falhar antes de publicar resultados |
| 6 → 7 | executar_experimentos | lote retorna erro sem CSV/logs |
| 8 ← 2–7 | documentação das APIs | somente leitura, sem edição compartilhada |
| 1–9 | consistência interna | escopo, arquivos e verificações compatíveis |

Task 1: concluída — fontes lidas pelo coordenador e analista independente; auditoria 10/139/58/3. Entradas extraídas sem transformação.
Task 2: em execução — agente modelos.
Task 8: em execução — agente documentacao.

Retomada após interrupção: arquivos e relatórios inspecionados; não refazer entregas completas. Task 2 implementada (4 testes de modelos), Task 4 implementada estruturalmente (3 testes), Task 8 primeira versão concluída. Coordenador confirmou cargo test: 7 aprovados, 0 falhas, 0 ignorados nesse estado parcial. Task 3 tinha somente skeleton; responsável retomou. Task 6 ainda sem arquivos; responsável retomou. Analista de requisitos assume revisão independente Tasks 2/4.

Revisão independente Tasks 2/4 por agente requisitos: conformidade e qualidade aprovadas, sem achados. Não julgou execução real de política/CPU (reservada), componentes fora do pacote ou histórico TDD; coordenador confirmou 7 testes disponíveis. Task 5 iniciada com agente cpu; Task 6 módulos declarados e RED em andamento. Clippy instalado: incluir na verificação final.

Task 3: implementação entregue, autor verificou 7/7 testes. Task 6: escritor entregue, autor verificou 6/6 incluindo flush /dev/full; métricas/lote stubs. Task 5: contratos entregues, autor verificou 3/3 e 23/23 da suíte disponível; RED foi import ausente, não falha comportamental (limitação registrada). Task 7: CLI em RED. Revisor independente distinto iniciado para Tasks 3/5/6/8 e, após integração, revisão ampla.

Revisor independente: Tasks 3/5/6/8 aprovadas por leitura, sem achados. Pontos não julgados resolvidos pelo coordenador: execução real de núcleo/médias está fora do escopo autorizado; formatador só recebe resumo, não valida equivalência matemática (cálculo reservado), mas valida denominadores; links de programa ignorados por decisão de arquivos regulares documentada; sem limite adicional de tamanho para entradas locais; Windows não disponível para execução. Histórico RED/GREEN segue relatos autores, verificação final será atual. CLI e sua documentação aguardam revisão integrada. Nenhum desses itens representa resultado de simulação.

Skills systematic-debugging e receiving-code-review lidas integralmente antes da análise de falhas/pareceres. Falhas transitórias por módulos/imports concorrentes foram rastreadas à integração ainda indisponível; nenhuma expectativa de teste foi relaxada. Demais testes dos autores usaram RED/GREEN dos comportamentos; stubs CPU tiveram somente RED de compilação, limitação explícita. A ausência de núcleo não será ocultada por testes aprovados de contratos.

Task 7 entregue: CLI autor verificou 7/7, suíte30/30. Task 8 finalizada contra CLI. Integração coordenador: cargo fmt corrigiu ordem dos módulos em lib.rs; check --all-targets/test30/fmt--check/clippy aprovados; help/validar código0, individual/lote código1 sem resultados; 12 entradas byte a byte iguais. Clippy inicialmente acusou default em unit struct num teste logs; autor corrigiu construção sem mudar comportamento, Clippy passou.

Revisão final independente por revisor: sem outros achados, stdout CLI print!/println! causa panic em /dev/full reproduzido (status101). Decisão: tratar falha de saída como ErroEp::Io, status1, coerente com erros do projeto; correção remetida ao especialista CLI com teste RED→GREEN (reclassificado importante pela ausência de panic). Custo se decisão errada: pequeno ajuste de tratamento de saída. Núcleo/cálculo/Windows/histórico TDD/tamanho arquivos não julgados conforme escopo e limitações já registrados.

Task 9 concluída: revisão final independente de CLI corrigida ADDRESSED; revisor reproduziu help/validar stdout /dev/full status1 Erro de E/S sem panic, nenhuma quebra nova. Coordenador verificou versão final: check --all-targets, teste32/32 (0 ignorados), fmt--check, clippy-Dwarnings, help/validar e buildrelease, todos código0. Binário release validou oficiais; individual e lote código1 sem arquivos. Entradas novamente byte a byte preservadas. Todas Tasks1–9 concluídas no escopo boilerplate. Relatório docs/fluxo/verificacao-final.md contém comandos e saídas reais. Nenhuma funcionalidade reservada declarada implementada.
