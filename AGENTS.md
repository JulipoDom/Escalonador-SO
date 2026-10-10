# Trabalho neste projeto

Este projeto usa Rust com Cargo, múltiplos agentes especializados e skills Superpowers por padrão. O agente principal coordena e integra; especialistas executam tarefas delimitadas e um agente distinto revisa o resultado. Antes de agir, consulte as skills aplicáveis e `../BOILERPLATE_EP1_ESCALONADOR.md`. Use planejamento, desenvolvimento orientado a testes quando aplicável, depuração sistemática, revisão de código e verificação antes de concluir. Paralelize somente trabalhos independentes e mantenha um editor por arquivo. Preserve os programas oficiais e o escopo autorizado: criar a base não significa implementar o núcleo reservado nem produzir resultados experimentais. Não peça novamente ao usuário para escolher entre execução individual e multiagente. Informe indisponibilidade real de ferramentas ou skills; nunca simule seu uso.

## Responsabilidade exclusiva

- Coordenador: Cargo.toml, Cargo.lock, src/lib.rs, .gitignore, AGENTS.md, plano e registro do fluxo.
- Requisitos: somente leitura das fontes de 2026 e ZIP.
- Modelos Rust: src/modelos.rs, src/erros.rs, tests/modelos.rs.
- Parsing: src/carregador.rs, tests/carregador.rs.
- Filas: src/tabela_processos.rs, src/filas.rs, src/politica.rs, tests/politica.rs.
- CPU e E/S: src/interpretador.rs, src/simulador.rs, tests/simulador.rs.
- Logs e métricas: src/registro.rs, src/estatisticas.rs, src/experimentos.rs, tests/registro_estatisticas.rs.
- CLI: src/cli.rs, src/main.rs, tests/cli.rs.
- Documentação: README.md, docs/decisoes.md, tests/fixtures/README.md, relatorio/roteiro.md.
- Revisor independente: somente leitura e achados; correções voltam ao autor.

## Fluxo e evidências

Consultar using-superpowers; writing-plans antes do código; subagent-driven-development na execução e dispatching-parallel-agents para tarefas independentes. Usar test-driven-development para comportamento implementado, systematic-debugging diante de falha, requesting-code-review/receiving-code-review e verification-before-completion. Brainstorming só para decisões abertas; Rust, arquitetura aprovada e multiagentes já foram definidos pelo usuário.

Modelos precedem consumidores; CLI integra após APIs disponíveis. Até três especialistas simultâneos mais coordenador, em ondas. Não mudar contratos compartilhados sem coordenação. Sem formatação global durante edições. Verificações finais serializadas: cargo check --all-targets, cargo test, cargo fmt --check, --help e --validar. Distinguir aprovados, ignorados e não executados.

A tabela possui BCPs, filas somente ProcessId. Sem unsafe, threads, tempo real ou ownership compartilhado desnecessário. Núcleo reservado retorna ErroEp::NaoImplementado com descrição específica, sem sucesso vazio. Não fabricar logs, CSV ou relatório experimental.

Workspace inicial sem Git: sem worktrees, commits ou scripts Superpowers dependentes de Git. Fluxo adaptado com plano e evidências em docs/fluxo; não criar repositório sem necessidade. Pedido explícito de execução e paralelismo prevalece sobre novas confirmações e serialização das skills.
