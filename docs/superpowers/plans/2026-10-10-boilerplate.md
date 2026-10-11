# Boilerplate EP1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development. Execute as tarefas delimitadas abaixo; cada especialista lê o contrato completo do seu domínio no documento fonte.

**Goal:** Criar a base funcional de entrada, CLI e logs, reservando o simulador.
**Architecture:** Tabela proprietária dos BCPs, filas de IDs; componentes independentes sobre modelos compartilhados. CLI distingue validação funcional de simulação pendente.
**Tech Stack:** Rust stable, edição 2021, Cargo, somente std.
**Spec:** ../../../../BOILERPLATE_EP1_ESCALONADOR.md e ../../../../ep1.pdf (2026).

## Global Constraints

Rust stable, edição 2021; binário Escalonador; somente biblioteca padrão. Preservar bytes dos 12 arquivos oficiais. Núcleo reservado com Err(ErroEp::NaoImplementado("descrição específica")); sem simulação ou métricas fictícias. Um editor por arquivo; agentes no mesmo workspace.

## Review Focus

Caminhos não UTF-8 (CLI); opções incompletas ou conflitantes (CLI); linhas vazias e números fora dos tipos (carregador); flush e propagação de erros (registro); modos pendentes sem arquivos de resultados (CLI/núcleo). Cada domínio deve cobrir seu comportamento com testes reais.

### Task 1: Requisitos e preparação — analista e coordenador
- [x] Ler todas as fontes; confirmar 10 processos, 58 créditos, quantum 3, 139 instruções.
- [x] Coordenador cria Cargo, diretórios, entradas preservadas, AGENTS e plano.

### Task 2: Modelos — especialista Rust
Files: src/modelos.rs, src/erros.rs, tests/modelos.rs.
Interfaces: exatamente seção 6; Bcp::novo(pid, programa, prioridade).
- [x] Escrever testes de estado inicial, independência e erros; observar RED com implementação mínima pendente.
- [x] Implementar modelos e erros; cargo test --test modelos deve passar.

### Task 3: Parsing — especialista leitura (depende Task 2)
Files: src/carregador.rs, tests/carregador.rs.
Interfaces: carregar_programas(&Path)->Result<Vec<Bcp>,ErroEp>; ler_quantum(&Path)->Result<usize,ErroEp>.
- [x] RED: oficiais 10/139/58/3; LF/CRLF; rejeições seção 7 com arquivo/linha; tipos e conjuntos menores.
- [x] Implementar e verificar cargo test --test carregador, esperado PASS.

### Task 4: Filas e tabela — especialista escalonamento (depende Task 2)
Files: src/tabela_processos.rs, src/filas.rs, src/politica.rs, tests/politica.rs.
Interfaces: TabelaProcessos possui HashMap<ProcessId,Bcp>; redistribuir_se_necessario(&mut TabelaProcessos)->Result<bool,ErroEp>.
- [x] Testar armazenamento único e consulta; implementar somente operações estruturais da tabela.
- [x] Reservar operações de política/filas com docs e erros específicos; testar contratos pendentes sem declarar regras implementadas.
- [x] cargo test --test politica, esperado PASS para contratos.

### Task 5: CPU/E/S — especialista CPU (depende Tasks 2 e 4)
Files: src/interpretador.rs, src/simulador.rs, tests/simulador.rs.
Interfaces: executar_instrucao, executar_quantum, simular conforme seção 6.
- [x] Criar stubs documentados inclusive avanço de E/S e transições; testes devem comprovar erro e ausência de mutação, sem fingir simulação.
- [x] cargo test --test simulador, esperado PASS para contratos pendentes.

### Task 6: Logs e métricas — especialista saída (depende Task 2)
Files: src/registro.rs, src/estatisticas.rs, src/experimentos.rs, tests/registro_estatisticas.rs.
Interfaces: RegistroLog::criar(&Path,usize)->Result<Self,ErroEp>; registrar(EventoLog), escrever_resumo(&Resumo), finalizar(); experimentos conforme seção 6.
- [x] RED para texto exato, singular, UTF-8, flush visível e erros de escrita.
- [x] Implementar escritor; coleta/cálculo/experimentos permanecem stubs específicos. cargo test --test registro_estatisticas, esperado PASS.

### Task 7: CLI — especialista execução (depende Tasks 3, 5 e 6)
Files: src/cli.rs, src/main.rs, tests/cli.rs.
Interfaces: executar(impl Iterator<Item=OsString>)->Result<(),ErroEp>; main ExitCode.
- [x] RED: ajuda, validação oficiais, erros de opções, caminhos, conflito validar/quanta e nenhum resultado em modos pendentes.
- [x] Implementar seção 11, testar cargo test --test cli, esperado PASS.

### Task 8: Documentação — especialista documentação (paralela, finalizar após 7)
Files: README.md, docs/decisoes.md, tests/fixtures/README.md, relatorio/roteiro.md.
- [x] Registrar uso, versões reais, convenções e pendências por módulo; sem relatório experimental.

### Task 9: Revisão independente e integração — revisor/coordenador
- [x] Revisar conformidade e qualidade de cada domínio e conjunto; encaminhar achados aos autores.
- [x] Coordenador inspeciona arquivos, formata após edições e verifica hashes ZIP.
- [x] cargo check --all-targets; cargo test; cargo fmt --check; cargo run --bin Escalonador -- --help; cargo run --bin Escalonador -- --validar. Esperado código 0.
- [x] Simulação individual e lote devem falhar sem publicar resultados. Clippy se instalado.
- [x] Registrar contagens reais de testes e comandos, pendências, limitações.

## Adaptações autorizadas

Usuário já forneceu arquitetura e ordenou execução, portanto sem nova aprovação de escolhas ou plano. Sem Git inicial: sem commits, worktrees e scripts baseados em SHA; revisão por arquivos efetivos e evidências. Paralelismo pedido explicitamente prevalece sobre serialização da skill; arquivos exclusivos evitam conflitos.
