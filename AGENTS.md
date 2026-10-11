# Desenvolvimento

Rust stable, edição 2021, Cargo e somente std. O pedido atual é manter apenas a base necessária ao EP1 de 2026; ele substitui as opções e ferramentas extras do boilerplate original. Consulte `../ep1.pdf` para os requisitos e o README para o estado atual.

Use múltiplos agentes especializados e Superpowers: planejamento proporcional, execução delegada, TDD para comportamentos implementados, depuração sistemática, revisão independente e verificação antes de concluir. Leia as skills antes de aplicá-las; informe limitações reais. O coordenador integra; um único agente edita cada arquivo e correções retornam ao autor. Paralelize somente tarefas independentes.

## Fluxo Superpowers

Recuperado das instruções do commit `58835cb` e adaptado à base `748077f`. Consulte o [plano vigente](docs/superpowers/plans/2026-10-10-contexto-atual.md) antes de iniciar novas implementações.

- Consulte `using-superpowers` ao iniciar e leia as demais skills antes de aplicá-las.
- Use `writing-plans` antes de implementar trabalho com várias etapas; registre novos planos em `docs/superpowers/plans/`.
- Execute planos com `subagent-driven-development`; use `dispatching-parallel-agents` para tarefas independentes. Respeite as dependências entre modelos e consumidores, com até três especialistas simultâneos mais o coordenador.
- Use `test-driven-development` para comportamentos implementados e `systematic-debugging` diante de falhas.
- Faça revisão independente com `requesting-code-review`; avalie os achados com `receiving-code-review` e encaminhe correções ao editor responsável.
- Use `verification-before-completion` antes de concluir ou publicar. Não execute formatação global enquanto especialistas estiverem editando.
- Use `brainstorming` para decisões abertas, sem pedir novamente confirmação de Rust, multiagentes ou Superpowers, já definidos pelo usuário.

O [índice Superpowers](docs/superpowers/README.md) separa o plano vigente do acervo recuperado. O plano original e os registros anteriores descrevem a base antes da simplificação; o README e o HISTORICO.md definem o estado atual. Não reative funcionalidades antigas por estarem citadas nos registros. As skills são fornecidas pelo ambiente: os documentos recuperados não instalam o plugin. Informe caso alguma skill ou ferramenta esteja indisponível.

## Responsabilidades e verificação

Responsáveis: execução (`main.rs`, testes de execução); entrada/modelos (carregador, modelos, erros e respectivos testes); núcleo (tabela, filas, política, interpretador, simulador e testes); saída (registro, estatísticas e testes); documentação (README e HISTORICO.md). Coordenador mantém Cargo, lib.rs e este arquivo; revisão independente é somente leitura.

Preserve os 12 arquivos oficiais byte a byte. A tabela possui os BCPs; filas guardam somente ProcessId. Sem unsafe, threads, tempo real ou ownership compartilhado desnecessário. O núcleo continua reservado com ErroEp::NaoImplementado específico; não implementar simulador completo sem pedido nem produzir logs ou medições fictícios. Sem CLI configurável, lote automático ou CSV.

Após cessarem edições, execute cargo check --all-targets, cargo test, cargo fmt --check e Clippy quando disponível. Confira erro explícito da execução enquanto o núcleo estiver pendente, ausência de resultados fictícios e preservação das entradas. Informe aprovados, falhas e ignorados; não confunda testes de contratos pendentes com simulação implementada.

Mantenha HISTORICO.md atualizado na mesma entrega de toda mudança de código, escopo, decisão ou pendência. Confira o estado atual e os próximos passos, marque somente implementações verificadas e acrescente um marco breve ao histórico quando houver mudança relevante. Registre apenas verificações realmente executadas e referências de commits existentes; não invente datas, resultados ou publicação. Ideias devem ser identificadas como propostas. Essa manutenção acompanha o desenvolvimento e não é uma atualização automática em segundo plano.
