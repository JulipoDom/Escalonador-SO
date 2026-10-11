# Primeira etapa: interpretador e quantum — plano de implementação

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development para executar as tarefas. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** executar instruções e um quantum sobre um BCP, mantendo simulação e escalonamento pendentes.
**Architecture:** conservar as assinaturas existentes; o interpretador atualiza somente PC/X/Y e devolve motivo de parada. O quantum acumula instruções até o limite ou E/S/SAIDA. Estado, espera e créditos são responsabilidade do laço futuro.
**Tech Stack:** Rust stable, edição 2021, Cargo e somente std.
**Spec:** contratos de `src/interpretador.rs`, `src/simulador.rs`, [plano vigente](2026-10-10-contexto-atual.md) e recorte apresentado no chat e autorizado pelo usuário em 2026-10-10.

## Global Constraints

- Rust stable, edição 2021, Cargo e somente std.
- Preserve os 12 arquivos oficiais byte a byte.
- A tabela possui os BCPs; filas guardam somente ProcessId.
- Sem unsafe, threads, tempo real ou ownership compartilhado desnecessário.
- Sem CLI configurável, lote automático ou CSV.
- `simular`, filas, política, estatísticas e `main` continuam pendentes; execução encerra com erro explícito e sem logs.
- Não alterar modelos, assinaturas públicas ou registros históricos recuperados de `58835cb`.
- Um editor por arquivo. Coordenador integra; autor corrige achados. A implementação inicial não autorizava commit ou publicação automática. Após sua conclusão, o usuário autorizou stage das alterações da etapa, exceto `BOILERPLATE_EP1_ESCALONADOR.md`, nova revisão e commit após nova verificação.

## Review Focus

- PC igual ao tamanho, maior que ele ou segmento vazio: EstadoInvalido antes de alteração da instrução.
- Quantum zero: Entrada sem mutação do BCP, mesmo com PC inválido.
- E/S ou SAIDA exatamente no limite: seu motivo prevalece sobre Quantum.
- Retomada após E/S: continuar na instrução seguinte sem repetição ou perda de registradores.
- Erro depois de instruções válidas: propagação sem panic nem rollback implícito do progresso já executado.

## Task 1: CPU e quantum com TDD

**Files:** modificar `src/interpretador.rs`, `src/simulador.rs` e `tests/simulador.rs`. Autor exclusivo: especialista núcleo.
**Interfaces:** consome `Bcp`, `Instrucao`, `MotivoParada`, `ResultadoQuantum`, `ErroEp` existentes. Produz `executar_instrucao(&mut Bcp) -> Result<Option<MotivoParada>, ErroEp>` e `executar_quantum(&mut Bcp, usize) -> Result<ResultadoQuantum, ErroEp>`.

- [x] Escrever e executar testes de instrução antes do código: atribuir X=-5 preserva Y=12; atribuir Y=99 preserva X=-8; COM mantém ambos; cada chamada avança PC uma vez; E/S retorna Some(Es), SAIDA Some(Saida), demais None. Estado, créditos, espera, identidade e programa preservados. PC fora do texto retorna EstadoInvalido sem mutação. Usar valores esperados literais e testes reais, sem mocks.
- [x] Executar `cargo test --test simulador` e observar falhas comportamentais contra NaoImplementado; substituir somente teste de recusa do interpretador. Registrar comandos e falhas reais.
- [x] Implementar interpretação com acesso validado à instrução e avanço único; executar testes e `cargo test` para conferir suite inteira. Teste de recusa do quantum permanece até a segunda parte.
- [x] Escrever testes de quantum antes do código e substituir sua recusa: [X=8, COM, Y=10, SAIDA], quantum 2 resulta PC=2, X=8, Y=0, 2 instruções, Quantum; próxima chamada resulta PC=4, Y=10, 2 instruções, Saida.
- [x] Testar parada antecipada e fronteira com [COM, E/S, X=9, SAIDA], quantum 2 e 4: primeiro resultado 2 instruções, Es, PC=2; retomada executa X=9 e SAIDA. [COM, SAIDA] termina em 2 instruções tanto com quantum 2 quanto 4. E/S ou SAIDA isolada conta 1. Quantum 1 impede execução da segunda instrução.
- [x] Testar quantum zero sem mutação, PC inicialmente inválido/segmento vazio, créditos/estado/espera preservados, propagação do erro ao esgotar programa sem SAIDA (PC preserva progresso válido) e quantum usize::MAX com SAIDA imediata sem reserva proporcional ao quantum.
- [x] Executar os novos testes e observar RED real. Implementar validação e laço mínimo; E/S/SAIDA retornam antes de classificar limite como Quantum. Não descontar crédito, não bloquear ou remover BCP.
- [x] Executar `cargo test`, formatar somente arquivos próprios e registrar GREEN real e auto-revisão. Preservar teste de simulação recusada e execução sem logs.
- [x] Revisão independente de conformidade e qualidade; corrigir achados com autor e verificar novamente.

Evidências da Task 1: RED de instrução com quatro falhas comportamentais; GREEN do interpretador antes de iniciar quantum; RED de quantum com nove falhas comportamentais; GREEN final do autor com 14 testes no alvo `simulador` e 39 na suíte, zero falhas e zero ignorados. Revisão independente de conformidade e qualidade aprovada sem achados.

## Task 2: Contexto e histórico

**Files:** `README.md`, `HISTORICO.md`, `docs/superpowers/README.md`, `docs/superpowers/plans/2026-10-10-contexto-atual.md`, este plano. Autor exclusivo: documentação. AGENTS fica com coordenador.
**Interfaces:** consome código e relatório aprovado da Task 1; produz estado correto e próximos passos.

- [x] Atualizar estado do interpretador/quantum, primeiro próximo passo filas/política e explicitar que execução completa permanece pendente. Corrigir referência ao PDF da raiz nos documentos vigentes; manter acervo histórico byte a byte.
- [x] Vincular este plano pelo índice vigente e marcar somente comportamento verificado. Registrar marco sobre base `4210a1c`, branch e implementação nesta entrega em HISTORICO, com TDD real e revisão. Não inventar resultados experimentais.
- [x] Após cessarem edições, coordenador executa `cargo check --all-targets`, `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, comparação byte a byte das entradas, execução com código 1 sem logs e `git diff --check`. Autor registra os resultados reais fornecidos.
- [x] Revisão final independente do conjunto, com conformidade e qualidade; autor corrige achados e coordenador verifica alterações afetadas.

Verificações finais executadas pelo coordenador: check, test (39 aprovados, zero falhas e zero ignorados; 14 no alvo `simulador`), fmt, Clippy e diff check aprovados. Os 12 arquivos oficiais conferem byte a byte com HEAD e assinaturas iniciais; acervo de 12 documentos e PDF/boilerplate locais intactos. Executável com código 1, stdout vazio, erro explícito de laço/despacho/transições pendentes e nenhum log antes/depois. Revisões independentes da documentação e do conjunto aprovadas em conformidade e qualidade, sem achados críticos, importantes ou menores; nenhuma correção de CPU necessária. Implementação da etapa concluída; filas e política permanecem como próximo passo proposto.

A nova revisão antes do commit foi aprovada em conformidade e qualidade, sem achados. Coordenador executou check, test (39/0/0), fmt, Clippy e conferência do diff em stage: aprovados. Stage com nove arquivos da etapa, sem PDF/boilerplate; entradas, acervo e documentos locais intactos. Execução com código 1, stdout vazio, pendência explícita e sem logs. Entrega apta para o commit autorizado, sem antecipar hash ou afirmar commit criado.
