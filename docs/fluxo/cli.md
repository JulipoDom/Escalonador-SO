# CLI

Escopo exclusivo: `src/cli.rs`, `src/main.rs`, `tests/cli.rs` e este registro. Consultados AGENTS, boilerplate seções 4, 6 e 11, plano Task 7, test-driven-development, writing-good-tests e verification-before-completion.

O executável usa args_os e ExitCode; erros saem em stderr. Parsing manual preserva caminhos não UTF-8, rejeita opções desconhecidas, caminhos ausentes, quanta vazios/não positivos e modos incompatíveis. Repetições idênticas são idempotentes; repetições com valores diferentes são rejeitadas.

A descoberta busca programas ao lado do executável antes do diretório atual. Caminho explícito prevalece. O pai das entradas fornece a raiz padrão; lote usa resultados/ e --saida prevalece. Lote chama executar_experimentos sem alterar quantum.txt. A execução individual retorna NaoImplementado antes de abrir RegistroLog: chamar simular com um escritor já aberto criaria um arquivo vazio enquanto o núcleo continua reservado. Nenhum modo pendente cria diretórios ou publica resultados.

Evidência TDD real: skeleton de CLI retornava NaoImplementado; cargo test --test cli resultou em 6 falhas comportamentais (ajuda, oficiais, argumentos inválidos, descoberta, caminhos não UTF-8 e precedência de executável) e 1 aprovação da ausência de resultados. Após implementação: 7/7 aprovados, zero ignorados. Testes usam CARGO_BIN_EXE_Escalonador e subprocessos reais com diretórios temporários, sem mocks. Validam stdout, stderr, status, total oficial 139, quantum intacto e ausência de resultados.

Verificação final do domínio: cargo test --test cli passou 7/7; cargo test passou 30 testes de integração, zero falhas e zero ignorados. rustfmt executado apenas nos três arquivos Rust de responsabilidade CLI. Formatação e verificações globais ficam com o coordenador após cessarem todas as edições.

## Correção após revisão: stdout indisponível

Consultadas receiving-code-review e systematic-debugging. Reprodução por dois subprocessos reais Linux com stdout em /dev/full: --help e --validar encerravam com status 101 e panic em std::io::stdio (failed printing to stdout, ENOSPC). Causa confirmada: print!/println! tratavam erro de saída por panic em vez de propagar o erro de E/S como os demais componentes.

RED: dois testes de regressão falharam pelo status 101 contra expectativa 1. Correção limitada à CLI: stdout.lock, write!/writeln! fallíveis com ? e flush explícito; ErroEp::Io chega ao main existente, que escreve stderr e retorna ExitCode::FAILURE. GREEN: cargo test --test cli 9/9 e cargo test 32/32, zero falhas e zero ignorados. Os testes Linux verificam status 1, stderr contendo Erro de E/S e ausência de panicked. Nenhuma API ou módulo de outro domínio foi alterado; rustfmt somente em src/cli.rs e tests/cli.rs.
