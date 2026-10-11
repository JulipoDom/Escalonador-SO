# Contexto atual — estado e próximas etapas

> **Para agentes:** ao executar trabalho autorizado, usar `superpowers:subagent-driven-development` por tarefa; `superpowers:executing-plans` é alternativa quando a delegação estiver indisponível. A etapa CPU marcada abaixo tem evidência de implementação; as caixas abertas indicam trabalho futuro.

**Objetivo:** registrar a primeira etapa autorizada e orientar as próximas implementações do EP1 de 2026 sem ampliar o escopo.
**Arquitetura:** a tabela possui os BCPs; filas guardam somente `ProcessId`. Interpretador e quantum executam sobre um BCP; as partes restantes do núcleo retornam `ErroEp::NaoImplementado` específico. Modelos, carregamento e escrita de eventos fornecidos já têm base implementada.
**Tecnologia:** Rust stable, edição 2021, Cargo e somente `std`.
**Fontes:** [enunciado de 2026](../../../ep1.pdf), [README](../../../README.md), [AGENTS](../../../AGENTS.md) e [HISTORICO](../../../HISTORICO.md). O enunciado de 2026 prevalece sobre documentos antigos; está em `ep1.pdf` na raiz do projeto.

## Limites e estado

- A primeira etapa de interpretador e quantum foi autorizada e implementada conforme o [plano específico](2026-10-10-etapa1-interpretador-quantum.md). Este roteiro não autoriza as etapas restantes.
- A CPU preserva identidade, programa, estado, prioridade, créditos e espera. PC inválido retorna `EstadoInvalido` sem mutação da instrução inválida; quantum zero retorna `Entrada` antes de ler PC. E/S e SAIDA contam e prevalecem sobre o limite; erros posteriores preservam progresso válido.
- Sem CLI configurável, lote automático, CSV, `unsafe`, threads ou tempo real.
- Preservar byte a byte os 12 arquivos oficiais em `programas/`.
- A execução atual valida entradas e encerra com código 1 por núcleo pendente, sem criar logs.
- Testes de recusa com `NaoImplementado` comprovam contratos pendentes, não simulação.
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
- [ ] **Filas e política — primeiro próximo passo proposto:** editar `src/filas.rs`, `src/politica.rs` e `tests/politica.rs`; consumir IDs da tabela. Testar empates, reinserções, desconto saturado por despacho, bloqueados com crédito, prioridades zero e redistribuição quando todos os ativos zerarem.
- [ ] **Transições e E/S:** depois de interpretador e filas, editar `src/simulador.rs` e `tests/simulador.rs`. Testar espera de dois surtos posteriores, surto que bloqueia sem descontar a própria espera, surtos curtos, repetição do PID, liberações FIFO e ausência de IDs duplicados.
- [ ] **Laço de simulação:** integrar os contratos anteriores em `src/simulador.rs`; testar tabela vazia ao término e ociosidade que avança espera lógica sem instruções, créditos ou despachos.
- [ ] **Estatísticas e logs reais:** depois do laço, editar `src/estatisticas.rs`, `src/registro.rs` e `tests/registro_estatisticas.rs`. Testar D/P e I/D em ponto flutuante, denominadores definidos, E/S e SAIDA contadas, despachos repetidos e propagação de falhas de escrita.
- [ ] **Execução integrada:** somente com núcleo funcional, editar `src/main.rs` e `tests/execucao.rs` para produzir eventos e resumo provenientes da simulação. Atualizar o teste de recusa apenas quando a integração real o substituir.
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
- [ ] Na base ainda pendente, conferir execução sem argumentos com código 1, erro explícito de núcleo pendente e ausência de logs; não esperar sucesso de `--help` ou `--validar` antigos.
- [ ] Comparar os 12 arquivos oficiais byte a byte com a base versionada e executar `git diff --check`.
- [ ] Conferir documentação contra o resultado observado. Só declarar capacidade implementada ou verificação aprovada com evidência desta entrega.
- [ ] Publicar apenas o escopo autorizado; commit e push não autorizam implementar etapas futuras deste plano.
