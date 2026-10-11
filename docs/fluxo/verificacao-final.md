# Verificação integrada — 10/10/2026

Executada pelo coordenador após a correção da CLI e o encerramento das edições de código. Rust stable 1.99.0, Cargo 1.99.0, Linux x86_64, edição 2021.

| Comando | Código | Resultado |
|---|---:|---|
| `cargo check --all-targets` | 0 | aprovado |
| `cargo test` | 0 | aprovado |
| `cargo fmt --check` | 0 | aprovado |
| `cargo clippy --all-targets -- -D warnings` | 0 | aprovado |
| `cargo run --bin Escalonador -- --help` | 0 | aprovado |
| `cargo run --bin Escalonador -- --validar` | 0 | aprovado |
| `cargo build --release --bin Escalonador` | 0 | aprovado |

Testes executados: **32 aprovados, 0 falhas, 0 ignorados**. Carregador 7; CLI 9; modelos 4; tabela/contratos de política 3; logs/contratos de métricas 6; contratos CPU/simulador 3. Testes de contratos pendentes não demonstram execução do escalonamento. Nenhuma verificação solicitada ficou sem execução. Windows não foi testado; simulação e experimentos completos não foram executados porque continuam reservados.

Validação confirmou 10 processos, 139 instruções e quantum 3. Soma de prioridades 58 verificada por testes. Os 12 arquivos foram comparados byte a byte com o ZIP, sem diferenças.

Simulação individual e lote foram executados para verificar a recusa: ambos código 1, mensagem Não implementado e nenhum arquivo de resultados. Diretórios resultados/ contêm apenas .gitkeep; relatorio/ contém roteiro e reserva de figuras. Nenhum log de simulação, CSV, medição ou PDF experimental foi produzido.

Revisão independente identificou panic na saída stdout da CLI quando o dispositivo falha. Autor reproduziu RED em /dev/full, corrigiu escrita fallível/flush, e os dois testes agora verificam status 1 e ErroEp::Io. Clippy também identificou construção default de unit struct em teste; autor corrigiu sem alterar comportamento.

Fluxo real: nove especialidades em ondas, arquivos exclusivos, Superpowers consultadas pelos responsáveis. Revisor atingiu limite temporário de uso; retomada posterior permitiu revisão integrada. Workspace sem Git: sem worktree/commits/scripts de revisão por SHA; plano, evidências e revisão dos arquivos efetivos foram preservados. CPU tem somente stubs e seu RED foi de imports ausentes, não TDD comportamental; limitação explicitada no relato.

## Saída real dos comandos

### cargo check --all-targets

```text
Checking ep1-escalonador v0.1.0 (/home/julipodom/Work/EP-SO/ep1-escalonador)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.82s
```

### cargo test

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/ep1_escalonador-e95b92d28cd095a5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/Escalonador-dc1c7f7f1e1541e0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/carregador.rs (target/debug/deps/carregador-e3a8c09d927a13fb)

running 7 tests
test diretorio_com_nome_nao_utf8_nao_causa_panic ... ok
test erros_de_leitura_incluem_caminho_e_linha_quando_disponivel ... ok
test quantum_exige_um_inteiro_positivo_representavel ... ok
test oficiais_preservam_ordem_prioridades_e_contagens ... ok
test seleciona_apenas_arquivos_ascii_e_ordena_antes_de_associar ... ok
test rejeita_prioridades_invalidas_ou_quantidade_diferente ... ok
test rejeita_programas_invalidos_com_arquivo_e_linha ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/cli.rs (target/debug/deps/cli-12c82f4ca2d8daae)

running 9 tests
test ajuda_nao_precisa_entradas ... ok
test ajuda_propaga_falha_de_stdout_sem_panic ... ok
test preserva_caminhos_nao_utf8 ... ok
test validacao_propaga_falha_de_stdout_sem_panic ... ok
test valida_oficiais_e_total ... ok
test fallback_cwd_e_ausencia_com_orientacao ... ok
test modos_pendentes_nao_criam_resultados_nem_alteram_quantum ... ok
test entradas_ao_lado_do_executavel_precedem_cwd ... ok
test rejeita_argumentos_invalidos ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/modelos.rs (target/debug/deps/modelos-b9c5fa59bac91874)

running 4 tests
test alterar_um_contexto_preserva_o_outro ... ok
test contexto_novo_preserva_programa_e_inicia_pronto ... ok
test erro_io_preserva_causa_e_mensagem ... ok
test erros_de_dominio_exibem_contexto_sem_causa_artificial ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/politica.rs (target/debug/deps/politica-c7741554c0378ff0)

running 3 tests
test contratos_pendentes_retornam_erro_sem_alterar_filas_ou_contexto ... ok
test pid_duplicado_nao_substitui_contexto_original ... ok
test tabela_preserva_contextos_e_remove_apenas_o_pid_solicitado ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/registro_estatisticas.rs (target/debug/deps/registro_estatisticas-48f5de17eea4ba14)

running 6 tests
test criacao_propaga_erro_de_io ... ok
test formato_dos_eventos_obedece_contrato_utf8_e_singular ... ok
test falha_de_flush_e_propagada ... ok
test cria_diretorio_e_publica_evento_antes_de_finalizar ... ok
test metricas_e_lote_permanecem_pendentes_sem_criar_destino ... ok
test resumo_com_denominador_zero_nao_publica_resultados ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/simulador.rs (target/debug/deps/simulador-5f6a73154ecb458b)

running 3 tests
test instrucao_pendente_nao_executa_nem_modifica_contexto ... ok
test quantum_pendente_nao_consome_instrucoes_ou_creditos ... ok
test simulacao_pendente_nao_produz_eventos_ou_resumo ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ep1_escalonador

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### cargo fmt --check

```text

```

### cargo clippy --all-targets -- -D warnings

```text
Checking ep1-escalonador v0.1.0 (/home/julipodom/Work/EP-SO/ep1-escalonador)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.49s
```

### cargo run --bin Escalonador -- --help

```text
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/Escalonador --help`
Uso: Escalonador [--validar] [--programas CAMINHO] [--saida CAMINHO]
     Escalonador --quanta N ... [--programas CAMINHO] [--saida CAMINHO]
     Escalonador --help

--validar mostra nome, prioridade, instruções e quantum, sem simular.
--programas seleciona entradas; padrão: programas/ ao lado do executável, depois no diretório atual.
--saida seleciona pasta do log individual logXX.txt (padrão: pai de programas/).
--quanta seleciona inteiros positivos para lote, sem alterar quantum.txt.
No lote, --saida é a raiz de logs/ e tabelas/ (padrão: pai de programas/resultados/).
Simulação e lote ainda não implementados; não publicam resultados.
```

### cargo run --bin Escalonador -- --validar

```text
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/Escalonador --validar`
TESTE-1: prioridade 1, instruções 10
TESTE-2: prioridade 3, instruções 21
TESTE-3: prioridade 2, instruções 14
TESTE-4: prioridade 10, instruções 12
TESTE-5: prioridade 4, instruções 11
TESTE-6: prioridade 6, instruções 15
TESTE-7: prioridade 12, instruções 14
TESTE-8: prioridade 8, instruções 21
TESTE-9: prioridade 8, instruções 15
TESTE-10: prioridade 4, instruções 6
quantum: 3
Total: 10 processos, 139 instruções
```

### cargo build --release --bin Escalonador

```text
Compiling ep1-escalonador v0.1.0 (/home/julipodom/Work/EP-SO/ep1-escalonador)
    Finished `release` profile [optimized] target(s) in 0.52s
```


Revisão escopada final: revisor confirmou ADDRESSED e reproduziu ambas as rotas stdout em /dev/full com status1, sem panic, nenhuma quebra nova. Binário release também executado: --validar --programas programas código0; simulação individual --saida resultados/logs código1; lote onze quanta --saida resultados código1. Comparação final dos 12 arquivos com ZIP e ausência de logs/CSV experimentais confirmadas.
