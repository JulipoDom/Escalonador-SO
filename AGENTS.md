# Desenvolvimento

Rust stable, edição 2021, Cargo e somente std. O pedido atual é manter apenas a base necessária ao EP1 de 2026; ele substitui as opções e ferramentas extras do boilerplate original. Consulte `../ep1.pdf` para os requisitos e o README para o estado atual.

Use múltiplos agentes especializados e Superpowers: planejamento proporcional, execução delegada, TDD para comportamentos implementados, depuração sistemática, revisão independente e verificação antes de concluir. Leia as skills antes de aplicá-las; informe limitações reais. O coordenador integra; um único agente edita cada arquivo e correções retornam ao autor. Paralelize somente tarefas independentes.

Responsáveis: execução (`main.rs`, testes de execução); entrada/modelos (carregador, modelos, erros e respectivos testes); núcleo (tabela, filas, política, interpretador, simulador e testes); saída (registro, estatísticas e testes); documentação (README). Coordenador mantém Cargo, lib.rs e este arquivo; revisão independente é somente leitura.

Preserve os 12 arquivos oficiais byte a byte. A tabela possui os BCPs; filas guardam somente ProcessId. Sem unsafe, threads, tempo real ou ownership compartilhado desnecessário. O núcleo continua reservado com ErroEp::NaoImplementado específico; não implementar simulador completo sem pedido nem produzir logs ou medições fictícios. Sem CLI configurável, lote automático ou CSV.

Após cessarem edições, execute cargo check --all-targets, cargo test, cargo fmt --check e Clippy quando disponível. Confira erro explícito da execução enquanto o núcleo estiver pendente, ausência de resultados fictícios e preservação das entradas. Informe aprovados, falhas e ignorados; não confunda testes de contratos pendentes com simulação implementada.
