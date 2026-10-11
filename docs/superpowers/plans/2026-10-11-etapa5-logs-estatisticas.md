# Quinta etapa — estatísticas e logs reais

> **For agentic workers:** usar `superpowers:subagent-driven-development`, TDD e revisão independente por tarefa.

**Objetivo:** coletar contagens e escrever eventos reais na API `simular`, sem integrar o binário principal.
**Arquitetura:** `Estatisticas` acumula um despacho concluído e suas instruções por `ResultadoQuantum`; `simular` prepara tabela/filas, consome `executar_passo`, registra os eventos e devolve `ResultadoSimulacao`. Preservar `executar_laco`, CPU e política. O escritor existente já publica e propaga falhas de flush.
**Tecnologia:** Rust stable, edição 2021, Cargo, somente std.
**Especificação:** `ep1.pdf`, seções 1.2–1.4; README e plano vigente. Implementação autorizada pelo usuário em 2026-10-11, com branch nova e contextos atualizados.

## Restrições globais

- Preservar byte a byte as 12 entradas oficiais e o PDF.
- Tabela possui BCPs, filas somente ProcessId; sem unsafe, threads, tempo real, CLI configurável, lote automático ou CSV.
- D = surtos concluídos, inclusive SAIDA e repetições do PID; I inclui E/S e SAIDA, exclui ociosidade. Médias D/P e I/D em f64.
- Carregando segue ordem inicial de prontos; Executando precede o surto; E/S precede Interrompendo, que usa somente instruções do último surto. SAIDA escreve Terminado com X/Y reais, sem interrupção adicional.
- Quantum zero, entrada vazia (denominadores indefinidos), duplicidade ou estado inicial incompatível devem falhar antes de eventos; erro de CPU/escrita aborta sem resumo de sucesso. Eventos já escritos podem permanecer.
- `main` e seus testes continuam recusando a integração principal. Commit e merge locais autorizados posteriormente pelo usuário, condicionados à última revisão; push não solicitado.

## Foco de revisão

Ordem de carga e empates; contagem de surtos curtos/repetidos; ociosidade com todos bloqueados; erros após progresso sem resumo; validação antes da saída e overflow sem contagem parcial.

## Tarefa 1 — saída: contadores e médias

Editor exclusivo: `src/estatisticas.rs`, `tests/registro_estatisticas.rs`; `src/registro.rs` somente se necessário para comportamento autorizado.

Interfaces: manter `registrar(&mut self, &ResultadoQuantum) -> Result<(), ErroEp>` e `calcular_resumo(usize, usize, usize, usize) -> Result<Resumo, ErroEp>`; acrescentar `Estatisticas::resumo(&self, quantum: usize, processos: usize) -> Result<Resumo, ErroEp>` como consumidor dos contadores privados.

- [x] Escrever testes de médias fracionárias (q=3, P=2, I=15, D=6 => 3 e 2.5), P/D zero e quantum zero; registro de motivos diferentes e mesmo PID, contagem zero rejeitada, overflow rejeitado sem mutação.
- [x] Executar RED compilável substituindo apenas o contrato pendente; depois implementar contadores com soma verificada e validação de denominadores.
- [x] Executar GREEN do alvo registro_estatisticas; preservar formatos UTF-8, singular/plural e falhas de escrita existentes. Testes privados de overflow se necessários.
- [x] Revisão independente de conformidade e qualidade antes da tarefa consumidora.

## Tarefa 2 — núcleo: integração de eventos e estatísticas

Editor exclusivo: `src/simulador.rs`, `tests/simulador.rs`, novo `tests/logs_simulacao.rs`.

Consome APIs da tarefa 1 e `executar_passo`; produz a assinatura existente `simular(Vec<Bcp>, usize, &mut RegistroLog) -> Result<ResultadoSimulacao, ErroEp>`. Preservar assinaturas de passo, quantum e laço. Caso necessário, um gancho privado dentro do passo registra Executando no despacho real antes da CPU, sem duplicar a seleção/política.

- [x] Escrever RED com log completo calculado à mão: um processo COM, E/S, X=7, SAIDA com q=2 => D=2, I=4, médias 2 e 2, evento E/S/Interrompendo seguido de Terminado X=7/Y=0; ociosidade sem eventos extras.
- [x] Cobrir carga por crédito/arquivo/PID, retomada e registradores, quantum repetido, E/S na fronteira, todos bloqueados, prioridades zero/redistribuição, vazio/quantum zero/IDs duplicados/estados/espera inválidos sem saída, erro CPU após surto válido sem médias e erro de escrita via /dev/full no Linux.
- [x] Implementar usando núcleo existente, registrar resumo apenas depois da conclusão e confirmar flush final. Atualizar contrato pendente de simular somente quando integração real o substituir.
- [x] GREEN dos alvos simulador/logs_simulacao/laco/transicoes; revisão independente de conformidade e qualidade.

## Tarefa 3 — contextos e verificação

Documentação: README, HISTORICO e índice/plano vigente. Coordenador: AGENTS e este plano.

- [x] Registrar quinta etapa implementada somente após evidência; próximo passo é integração de main e depois experimentos/relatório.
- [x] Após cessarem edições executar check de todos os alvos, suíte, fmt --check, Clippy com warnings negados e diff --check; registrar aprovados/falhas/ignorados reais.
- [x] Verificar execução principal com código 1, stdout vazio, erro explícito e ausência de logs; comparar entradas/PDF/acervo com base eddae3c.
- [x] Revisão final independente do conjunto e documentação; entregar branch inicialmente sem commit/merge/push; autorização posterior registrada abaixo.

## Evidência desta entrega

Branch `codex/etapa5-logs-estatisticas`, base `eddae3c`, sem commit, stage, merge ou push. Saída: RED compilável em coleta/cálculo/validação e overflow, GREEN 15 testes do recorte. Núcleo: RED compilável nos dez testes novos e contrato atualizado; GREEN 57 testes do recorte. Revisões independentes da saída e do núcleo aprovadas em conformidade e qualidade, sem achados.

Coordenador, após cessarem edições: `cargo check --all-targets`, `cargo test` (111 aprovados, zero falhas/ignorados; 2 lib e 109 integração), `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` e `git diff --check` aprovados. Execução principal com código 1, stdout vazio, diagnóstico explícito de integração pendente e zero logs antes/depois. Os 25 arquivos protegidos (12 entradas, PDF e 12 documentos do acervo) conferem byte a byte com `eddae3c`. Revisão final independente do conjunto e documentação aprovada em conformidade e qualidade, sem achados críticos, importantes ou menores. A evidência de I/O via /dev/full cobre carga inicial; propagação tardia e flush final foram conferidos por inspeção.

## Autorização de integração

Após colocar os 11 arquivos da etapa em stage, o usuário autorizou commit e merge local com `main`, condicionados a uma última revisão. Esta autorização substitui a condição anterior de entrega sem commit/merge e não amplia o escopo. Push não foi solicitado. Verificação pré-commit do coordenador: check de todos os alvos, suíte 111/0/0, fmt e Clippy aprovados; 25 arquivos protegidos conferidos contra `eddae3c`; execução principal com código 1, stdout vazio e zero logs antes/depois. Revisão independente pré-commit aprovada em conformidade e qualidade, sem achados críticos, importantes ou menores. Código e testes em stage coincidem com o checkout verificado; metadados documentais atualizados para registrar o parecer. Commit `e61ce88` criado com os 11 arquivos da etapa; após autorização, integrado localmente à `main` por fast-forward de `eddae3c`. Suíte pós-merge na `main`: 111 aprovados, zero falhas e zero ignorados. Sem push; integração da execução principal continua pendente.
