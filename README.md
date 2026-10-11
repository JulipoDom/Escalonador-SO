# EP1 — Escalonador

Base em Rust stable, edição 2021, para o EP1 de Sistemas Operacionais de 2026. Usa somente a biblioteca padrão. A entrada e a escrita de eventos têm implementação; o núcleo de simulação permanece reservado. Esta base não contém resultados experimentais.

## Ambiente e execução

Toolchain observado em 10/10/2026:

```text
rustc 1.99.0 (b940084d7 2026-09-28)
cargo 1.99.0 (5f94df478 2026-08-27)
stable-x86_64-unknown-linux-gnu
```

Instale Rust stable com Cargo para desenvolver. Execute a partir da subpasta do projeto:

```bash
cd ep1-escalonador
cargo run --bin Escalonador -- --help
cargo run --bin Escalonador -- --validar
cargo run --bin Escalonador -- --validar --programas programas
cargo check --all-targets
cargo test
cargo fmt --check
cargo build --release --bin Escalonador
```

`--validar` mostra nomes, prioridades, número de instruções e quantum, sem simular. Há dez processos oficiais, prioridades somando 58, quantum padrão 3 e 139 instruções. Esses dados descrevem a entrada, não uma execução da CPU.

Os comandos abaixo expressam as interfaces reservadas. Enquanto faltar o núcleo, falham com `ErroEp::NaoImplementado`, código diferente de zero e sem publicar logs ou CSV:

```bash
cargo run --bin Escalonador
cargo run --bin Escalonador -- --programas programas --saida resultados/logs
cargo run --bin Escalonador -- --quanta 1 3 5 7 9 11 13 15 17 19 21 --saida resultados
./target/release/Escalonador --programas programas --saida resultados/logs
```

O binário compilado dispensa Cargo no destino, respeitada a compatibilidade do sistema. No Windows, o caminho é `target\release\Escalonador.exe`.

Sem `--programas`, a busca tenta `programas/` ao lado do executável e depois no diretório atual. Se não encontrar, informe `--programas CAMINHO`. Caminhos explícitos relativos usam o diretório atual. Na execução individual, o destino padrão é a pasta pai do diretório de programas; `--saida` indica a pasta do log. No lote, `--saida` indica a raiz de `logs/` e `tabelas/`; o padrão é `resultados/` na raiz identificada. `--quanta` substitui o quantum somente no lote, sem editar a entrada. `--validar` e `--quanta` são incompatíveis. Opções desconhecidas, valores inválidos e argumentos ausentes são erros. Repetir uma opção de caminho ou a lista de quanta com valores diferentes também é erro. No lote, os quanta informados dispensam a leitura de `quantum.txt`.

## Estrutura e estado por módulo

| Caminho | Implementação nesta base | Trabalho posterior |
| --- | --- | --- |
| `src/modelos.rs`, `src/erros.rs` | BCP, programa tipado, resultados e erros | Usar os tipos no núcleo |
| `src/carregador.rs` | Leitura e validação dos programas, prioridades e quantum | Nenhuma execução de instruções pertence ao carregador |
| `src/tabela_processos.rs` | Armazenamento, inserção, consulta, empréstimo mutável e remoção | Coordenar os ativos durante a simulação |
| `src/filas.rs`, `src/politica.rs` | Estruturas e contratos explícitos | Seleção, reinserção, desempates e créditos |
| `src/interpretador.rs`, `src/simulador.rs` | Assinaturas documentadas e erros de funcionalidade pendente | CPU, PC, quantum, E/S, transições e laço principal |
| `src/registro.rs` | Formatação e escrita UTF-8 de eventos, resumo recebido, criação de diretórios e flush | Integrar eventos e dados calculados pelo núcleo |
| `src/estatisticas.rs` | Contratos reservados | Contagem e cálculo das médias |
| `src/experimentos.rs` | Contrato reservado | Execuções independentes e CSV |
| `src/cli.rs`, `src/main.rs` | Ajuda, argumentos, validação, caminhos e propagação de erros | Conectar simulação e lote completos |
| `tests/` | Verificações dos componentes e dos erros pendentes | Cenários de comportamento do núcleo |

A tabela é a única proprietária dos BCPs ativos. Filas armazenam somente `ProcessId`; o texto usa `Box<[Instrucao]>`. Não há threads ou relógio real. Rotinas reservadas retornam erros específicos em vez de simular sucesso.

`programas/` preserva os dez programas e os dois arquivos de configuração oficiais. `resultados/logs/`, `resultados/tabelas/` e `relatorio/figuras/` reservam destinos. `docs/decisoes.md` registra regras; `tests/fixtures/README.md` descreve cenários; `relatorio/roteiro.md` orienta o relatório futuro. `AGENTS.md` e `docs/superpowers/plans/` documentam a organização do desenvolvimento.

## Continuidade e verificação

Implemente, nesta ordem sugerida: interpretador; filas e política; E/S e simulador; estatísticas e integração dos logs; experimentos; relatório. As convenções detalhadas estão em [docs/decisoes.md](docs/decisoes.md).

Os comandos de verificação acima são instruções reproduzíveis, não uma declaração de aprovação. As evidências da integração ficam em `docs/fluxo/`. Testes de contratos pendentes comprovam o erro e a ausência de sucesso fictício; não comprovam escalonamento. Se Clippy estiver instalado, use também `cargo clippy --all-targets -- -D warnings`.
