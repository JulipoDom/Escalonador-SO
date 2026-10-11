# Contexto atual — estado e próximas etapas

> **Para agentes:** ao executar trabalho autorizado, usar `superpowers:subagent-driven-development` por tarefa; `superpowers:executing-plans` é alternativa quando a delegação estiver indisponível. As etapas CPU, filas/política, passo integrado e laço do núcleo marcadas abaixo têm evidência de implementação; as caixas abertas indicam trabalho futuro.

**Objetivo:** registrar as quatro etapas autorizadas e orientar as próximas implementações do EP1 de 2026 sem ampliar o escopo.
**Arquitetura:** a tabela possui os BCPs; filas guardam somente `ProcessId`. Interpretador e quantum executam sobre um BCP; filas, política de créditos, operações de espera de E/S e passo integrado de despacho e transições são consumidos por `executar_laco` até a conclusão, com mapa de registradores finais reais. `simular` com resumo/registro e estatísticas permanecem reservados com `ErroEp::NaoImplementado` específico. Modelos, carregamento e escrita de eventos fornecidos já têm base implementada.
**Tecnologia:** Rust stable, edição 2021, Cargo e somente `std`.
**Fontes:** [enunciado de 2026](../../../ep1.pdf), [README](../../../README.md), [AGENTS](../../../AGENTS.md) e [HISTORICO](../../../HISTORICO.md). O enunciado de 2026 prevalece sobre documentos antigos; está em `ep1.pdf` na raiz do projeto.

## Limites e estado

- A primeira etapa de interpretador e quantum foi autorizada e implementada conforme o [plano específico](2026-10-10-etapa1-interpretador-quantum.md). Foi integrada à `main` no commit `7b92a6a`. A segunda etapa implementa filas, política e operações de espera de E/S conforme o [plano específico](2026-10-10-etapa2-filas-politica.md), registrada em `8d4d172`, integrada e publicada na `main`. A terceira etapa integra despacho, transições e E/S em um passo conforme o [plano específico](2026-10-11-etapa3-transicoes-es.md), registrada no commit `57099c4` e integrada localmente à `main` por fast-forward autorizado de `7ced42f`, sem push. A quarta etapa implementa o laço completo do núcleo conforme o [plano específico](2026-10-11-etapa4-laco-simulacao.md), com código e testes registrados no commit `75941d0` após revisão independente pré-commit sem achados e autorização de dois commits separados. Os contextos estão em preparação para o segundo commit autorizado, na branch `codex/etapa4-laco-simulacao`, sem merge ou push. Este roteiro não autoriza as etapas restantes.
- A CPU preserva identidade, programa, estado, prioridade, créditos e espera. PC inválido retorna `EstadoInvalido` sem mutação da instrução inválida; quantum zero retorna `Entrada` antes de ler PC. E/S e SAIDA contam e prevalecem sobre o limite; erros posteriores preservam progresso válido.
- `executar_passo` valida toda a estrutura antes de mutar, despacha com desconto de crédito e aplica Quantum, E/S ou SAIDA. O resultado distingue Surto com BCP terminado opcional, Ocioso com IDs liberados e Concluido para estado vazio. SAIDA devolve o contexto proprietário; ociosidade avança só um tick lógico sem CPU ou alteração de créditos.
- Erro da CPU após despacho preserva Executando fora das filas, crédito descontado e progresso válido; esperas anteriores não avançam. Não há recuperação automática: uma nova chamada rejeita esse estado inconsistente antes de mutar.
- `executar_laco` prepara tabela e filas, captura registradores finais dos BCPs terminados e consome surtos e ociosidade até `Concluido`, sem limite artificial. Entrada vazia com quantum positivo devolve mapa vazio; quantum zero é recusado antes da preparação e erros são propagados. Não coleta medidas nem eventos.
- Sem CLI configurável, lote automático, CSV, `unsafe`, threads ou tempo real.
- Preservar byte a byte os 12 arquivos oficiais em `programas/`.
- A execução atual não chama `executar_laco`: valida entradas e encerra com código 1 e `Não implementado: integração da simulação com estatísticas e logs`, sem criar logs.
- Testes de recusa com `NaoImplementado` comprovam contratos pendentes; testes do laço comprovam a conclusão do núcleo e os registradores finais, sem integração de resumo ou registro.
- Não produzir logs, contagens, medições, resultados experimentais ou RED fictícios.
- O [plano antigo](2026-10-10-boilerplate.md) e `docs/fluxo/` são acervo de `58835cb`, anterior a `ffc8849`.

## Método e dependências

- [ ] Antes de implementar, conferir o pedido autorizado, o enunciado e o estado no HISTORICO; detalhar o recorte com `writing-plans` e usar `brainstorming` somente para decisões abertas.
- [ ] Ler cada skill antes de aplicá-la; informar ferramentas indisponíveis. Usar `using-git-worktrees` se for necessária uma área isolada.
- [ ] Coordenador define contratos, integra e mantém Cargo, `src/lib.rs` e AGENTS; até três especialistas simultâneos, um editor por arquivo.
- [ ] Delegar entrada/modelos, núcleo, saída, execução e documentação conforme AGENTS; paralelizar apenas tarefas sem dependências nem arquivos compartilhados.
- [ ] Para cada comportamento autorizado, usar `test-driven-development`: escrever teste que falhe pelo comportamento ausente, executar e registrar a falha real, implementar o mínimo, executar e refatorar com testes aprovados.
- [ ] Se houver falha inesperada, usar `systematic-debugging` antes de propor correção; não chamar erro de compilação acidental de evidência RED do comportamento.
- [ ] Fazer revisão independente com `requesting-code-review`; avaliar achados com `receiving-code-review` e devolver correções ao editor. Depois verificar novamente.
- [ ] Atualizar HISTORICO na mesma entrega, distinguindo implementado, pendente, proposta e verificações realmente executadas.

## Sequência proposta após autorização

- [x] **Interpretador e quantum:** `src/interpretador.rs`, `src/simulador.rs` e `tests/simulador.rs` implementados sem alterar contratos de modelos. TDD real com atribuições, COM, PC único, E/S e SAIDA contadas, retomadas, fronteiras, quantum zero e erros com progresso válido. Autor: 14 testes no alvo `simulador` e 39 na suíte aprovados, zero falhas/ignorados; revisão independente sem achados. Verificações finais do coordenador e revisões independentes de documentação e conjunto aprovadas, sem achados, registradas no HISTORICO.
- [x] **Filas, política e operações de espera de E/S:** `src/filas.rs`, `src/politica.rs`, `tests/filas.rs` e `tests/politica.rs` implementados com TDD real. Carga, seleção e reordenação estável; reinserções antes/depois de iguais e exceções FIFO; desconto saturado e redistribuição de todos os ativos; bloqueio com espera 2, avanço apenas de snapshot anterior e liberação FIFO. Validações antes da mutação e seleção sem alterar estado. Revisão independente de conformidade e qualidade sem achados. Coordenador: check, 59 testes aprovados (zero falhas/ignorados), fmt e Clippy aprovados; execução ainda recusa o laço, sem logs, e 12 entradas idênticas à base `7b92a6a`.
- [x] **Integração de transições e E/S em um passo:** `src/simulador.rs` e novo `tests/transicoes.rs` implementados com TDD real; CPU e stub de `simular` preservados. Despacho, transições, contexto terminado, snapshots anteriores, dois surtos posteriores de E/S, surtos curtos e consecutivos do mesmo PID, liberação FIFO, tick ocioso, conclusão vazia e abortos da CPU cobertos por 18 testes calculados à mão. Revisão independente do núcleo em conformidade e qualidade sem achados. Coordenador: check, 77 testes aprovados (zero falhas/ignorados), fmt, Clippy e diff check aprovados; execução com código 1, stdout vazio, erro explícito de laço pendente e sem logs. As 12 entradas, PDF, acervo e boilerplate local conferem byte a byte com a base `7ced42f`. Revisões documental e final daquela entrega aprovadas sem achados. Revisão pré-commit aprovada sem achados; commit `57099c4` integrado localmente à `main` por fast-forward autorizado, sem push. Suíte pós-merge: 77 aprovados, zero falhas e zero ignorados.
- [x] **Laço completo do núcleo:** `executar_laco` em `src/simulador.rs` consome `executar_passo` até a conclusão e devolve mapa de registradores finais reais. Quinze testes novos em `tests/laco.rs`, com expectativas calculadas à mão e uma execução dos arquivos oficiais, cobrem vazio, quantum zero, retomadas, E/S consecutivas, prioridades zero, redistribuições, fronteiras e erros após progresso. TDD real; revisões independentes de conformidade e qualidade do núcleo aprovadas sem achados. Coordenador: check, 92 testes aprovados (zero falhas/ignorados), fmt, Clippy e diff check aprovados; execução principal continua com código 1, stdout vazio, diagnóstico específico de integração pendente e sem logs. Entradas, PDF, acervo e boilerplate preservados byte a byte contra `94a9482`. `simular` permanece reservado à integração com estatísticas e registro; `main` apenas atualiza diagnóstico/comentário. Revisões documental e final desta entrega aprovadas em conformidade e qualidade, sem achados críticos, importantes ou menores.
- [ ] **Estatísticas e logs reais — próximo passo proposto:** depois do laço, editar `src/estatisticas.rs`, `src/registro.rs` e `tests/registro_estatisticas.rs`. Testar D/P e I/D em ponto flutuante, denominadores definidos, E/S e SAIDA contadas, despachos repetidos e propagação de falhas de escrita.
- [ ] **Execução integrada:** somente depois das estatísticas e logs reais, editar `src/main.rs` e `tests/execucao.rs` para produzir eventos e resumo provenientes da simulação. Atualizar o teste de recusa apenas quando a integração real o substituir.
- [ ] **Experimentos e relatório:** após validação, executar manualmente ao menos dez quantums uniformemente distribuídos em intervalo útil e reunir logs, médias, tabelas e gráficos reais para justificar a escolha no PDF.

Cada implementação futura exige autorização e um plano específico com APIs, dependências e testes concretos, após conferir as assinaturas existentes. Cada etapa recebe testes pequenos calculados à mão e revisão própria antes de consumir seus resultados na etapa dependente; este documento é um roteiro de fluxo e contexto.

## Foco da revisão

- PC e contagem de E/S/SAIDA: cada instrução avança e consome uma única unidade.
- Créditos de bloqueados e prioridades zero: redistribuição considera todos os ativos.
- E/S com surtos curtos ou PID repetido: esperar dois surtos posteriores e liberar em FIFO.
- Filas e término: evitar IDs duplicados, BCP perdido e laço infinito em ociosidade.
- Estatísticas e escrita: usar valores reais, divisão definida e erros de saída propagados.

## Verificação antes de concluir ou publicar

- [ ] Depois de cessarem todas as edições e tratar a revisão, usar `verification-before-completion`; não executar formatação global durante edição concorrente.
- [ ] Executar `cargo check --all-targets`, `cargo test`, `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` quando Clippy estiver disponível; registrar aprovados, falhas e ignorados.
- [ ] Enquanto a integração estiver pendente, conferir execução sem argumentos com código 1, erro explícito de integração com estatísticas e logs pendente e ausência de logs; não esperar sucesso de `--help` ou `--validar` antigos.
- [ ] Comparar os 12 arquivos oficiais byte a byte com a base versionada e executar `git diff --check`.
- [ ] Conferir documentação contra o resultado observado. Só declarar capacidade implementada ou verificação aprovada com evidência desta entrega.
- [ ] Publicar apenas o escopo autorizado; commit e push não autorizam implementar etapas futuras deste plano.
