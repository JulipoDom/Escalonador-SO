# Sexta etapa — execução principal integrada

> **For agentic workers:** usar `superpowers:subagent-driven-development`, TDD e revisão independente.

**Objetivo:** substituir a recusa de `main` pela execução completa usando as APIs existentes.
**Arquitetura:** `main` rejeita argumentos, carrega e valida `programas/` e quantum; somente depois cria `RegistroLog` em `.` e chama `simular(processos, quantum, &mut registro)`. A API já publica eventos, resumo e flush final. Não duplicar núcleo nem estatísticas.
**Tecnologia:** Rust stable, edição 2021, Cargo e somente std.
**Especificação:** `ep1.pdf`, seções 1.2–1.4, README e pedido de integração da execução autorizado em 2026-10-11.
**Base:** `5ca6f6c`; branch `codex/etapa6-execucao-integrada`. Baseline `cargo test`: 111 aprovados, zero falhas/ignorados.

## Restrições globais

- Preservar as 12 entradas oficiais, PDF e acervo byte a byte; sem alterações nas APIs de núcleo, carregamento, modelos, estatísticas ou escritor.
- Sem CLI configurável, lote automático, CSV, threads, tempo real ou unsafe.
- Criar `logXX.txt` no diretório corrente, com quantum de `programas/quantum.txt`; substituir somente o log desse quantum após entrada válida (contrato existente do escritor).
- stdout e stderr vazios em sucesso; exit0 apenas depois de simulação e publicação final aprovadas. Erros retornam exit1 e diagnóstico em stderr.
- Rejeição de argumentos e entradas inválidas antecedem criação/truncamento do log. Erro de escrita não pode retornar sucesso. Logs parciais reais podem permanecer após erro de execução/escrita.
- Experimentos de dez quantums e relatório continuam pendentes; não há autorização de commit, merge ou push desta nova etapa.

## Foco de revisão

Caminhos relativos ao cwd, quantum realmente lido, validação antes de truncar log existente, retomada após E/S e propagação de falha de criação/publicação até exit1.

## Tarefa 1 — execução (um editor)

Arquivos: `src/main.rs` e `tests/execucao.rs`. Interfaces consumidas: `carregar_programas(&Path) -> Result<Vec<Bcp>, ErroEp>`, `ler_quantum(&Path) -> Result<usize, ErroEp>`, `RegistroLog::criar(&Path, usize) -> Result<RegistroLog, ErroEp>`, `simular(Vec<Bcp>, usize, &mut RegistroLog) -> Result<ResultadoSimulacao, ErroEp>`.

- [x] Substituir teste da recusa por execução real com PEQUENO, X=8, SAIDA e q3: exit0, streams vazios e log03 exatamente Carregando/Executando/Terminado X=8 Y=0, médias1 e2, quantum3.
- [x] Acrescentar testes calculados à mão: q1 produz dois despachos, I2/D2, log01 e nenhum log03; COM,E/S,X=7,SAIDA com q2 produz E/S+Interrompendo2, retomada/Terminado, D2 I4; novo log substitui anterior e preserva log de outro quantum.
- [x] Validar erro de criação (log03.txt diretório) e escrita (/dev/full no Linux) retorna exit1/Erro de E/S. Entrada inválida ou argumentos devem preservar log já existente e não gerar resultados novos; programas/prioridades/quantum inválidos continuam com contexto.
- [x] Executar RED compilável antes da implementação mínima. Implementar somente integração de main, executar GREEN e suíte completa; formatar apenas arquivos próprios.
- [x] Revisão independente de conformidade e qualidade antes dos contextos finais.

## Tarefa 2 — contextos e verificação

Editor documental: README, HISTORICO, índice Superpowers e plano vigente. Coordenador: AGENTS e este plano.

- [x] Atualizar estado atual e próximos passos; registrar entrega sem commit sobre base existente, preservar marcos antigos. Explicar execução em cwd e substituição de log do quantum escolhido.
- [x] Após cessarem edições executar `cargo check --all-targets`, `cargo test`, `cargo fmt --check`, Clippy com warnings negados e `git diff --check`.
- [x] Executar binário com cópias byte a byte dos programas oficiais em diretório temporário, confirmar exit0, log real com dez términos e resumo; comparar entradas/PDF/acervo com `5ca6f6c`. Não gerar logs na raiz nem medições experimentais adicionais.
- [x] Revisão final independente de código/contextos; entregar branch sem stage/commit/merge/push.

## Evidências desta entrega

TDD do autor: RED compilável de 5 aprovados/6 falhas/0 ignorados no alvo execução; GREEN 11/0/0 e suíte 117/0/0. Revisão independente de código aprovada em conformidade e qualidade sem achados. Coordenador: check de todos os alvos, suíte 117/0/0 (2 lib e 115 integração), fmt em modo check, Clippy com warnings negados e diff check aprovados. Execução com cópias das entradas oficiais em diretório temporário: exit0, stdout/stderr vazios, quantum3, dez cargas/términos, 59 despachos e 139 instruções; médias reais 5.9 e 2.3559322033898304.

Entradas e PDF/acervo conferidos byte a byte com `5ca6f6c`; 31 snapshots (25 arquivos protegidos, 5 APIs e boilerplate) preservados. Nenhum log criado na raiz do projeto. A simulação oficial usada como verificação não substitui os experimentos com dez quantums ou o relatório futuro. Revisão final independente do conjunto e documentação aprovada em conformidade e qualidade, sem achados críticos, importantes ou menores; sem stage, commit, merge ou push.
