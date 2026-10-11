# Etapa 4 — laço completo do núcleo

> **Para agentes:** execução por tarefa com superpowers:subagent-driven-development, TDD e revisão independente.

**Goal:** executar continuamente os passos de escalonamento até todos os processos terminarem e devolver registradores finais reais.

**Architecture:** `executar_laco` possui localmente tabela e filas, inicializa os prontos e consome `executar_passo` até Concluido. O resultado é o mapa de registradores finais; não inclui resumo ou logs. A API `simular`, dependente dessas partes futuras, permanece reservada sem eventos.

**Tech Stack:** Rust stable, edição 2021, Cargo, somente std.

**Spec:** ep1.pdf, README, contexto vigente e recorte aprovado na conversa: preparar tabela/filas, executar até término, preservar registradores, propagar erros e garantir progresso com todos bloqueados. Base 94a9482; branch codex/etapa4-laco-simulacao.

## Global Constraints

- Preserve os 12 arquivos oficiais byte a byte, PDF, boilerplate e acervo histórico.
- A tabela possui BCPs ativos; filas contêm somente ProcessId. Sem unsafe, threads, tempo real ou ownership compartilhado.
- Somente std; sem CLI configurável, lote automático, CSV ou experimentos nesta entrega.
- Preservar contratos de `executar_passo` e `executar_quantum`, modelos, filas e política.
- Não implementar estatísticas, logs ou integração real da main. Não preencher Resumo com valores artificiais.
- Um editor por arquivo; correções ao autor. A entrega inicial aguardou revisão humana sem stage ou commit. Em 2026-10-11, após preparar o stage, o usuário autorizou revisão e dois commits separados: código/testes e contextos/documentação. Depois autorizou o merge local com a main, realizado por fast-forward de `94a9482` para `05bc657`. Não houve push desta etapa.

## Review Focus

- Todos bloqueados não significa conclusão; ticks lógicos continuam até haver pronto.
- Créditos e prioridades zero devem progredir usando a política já verificada.
- Quantum zero e PID duplicado devem retornar erro; entrada vazia com quantum positivo é conclusão válida do núcleo, sem médias.
- Erro depois de um término deve retornar Err, sem resultado parcial apresentado como conclusão.
- Não perder registradores finais nem reiniciar PC/registradores de contextos Pronto fornecidos.

## Task 1: laço e testes do núcleo

**Files:** editor único src/simulador.rs e novo tests/laco.rs. Preservar testes existentes.

**Interfaces:** consumir Bcp, TabelaProcessos, filas, executar_passo/ResultadoPasso existentes. Produzir `pub fn executar_laco(processos: Vec<Bcp>, quantum: usize) -> Result<HashMap<ProcessId, (i64, i64)>, ErroEp>` em simulador.rs; HashMap da std.

Contrato:
1. Quantum zero retorna Entrada antes de preparar o estado, inclusive vetor vazio.
2. Inserir todos os BCPs na tabela antes de executar; inserir rejeita PID duplicado. Inicializar FilaProntos por ordenar_inicial; FilaBloqueados vazia. Os contextos iniciais devem ser Pronto com espera zero; validação existente de executar_passo recusa outros estados. Preservar programa/PC/registradores/créditos dos contextos válidos; não redefinir créditos fora da política.
3. Repetir executar_passo. Surto com terminado Some: guardar pid->(x,y) real do BCP removido. Outros surtos e Ocioso continuam; nunca encerrar apenas porque não há prontos.
4. Concluido retorna o mapa. Essa variante valida tabela e filas vazias; não fabricar estado final independente desse contrato. Propagar todos os erros com `?`, sem recuperar automaticamente nem devolver mapa parcial como sucesso.
5. Não duplicar política/CPU/transições, não coletar contagens/médias ou traces e não escrever eventos. Não impor limite artificial de passos que aborte entradas finitas válidas.
6. Preservar comportamento e assinaturas de executar_passo/executar_quantum. `simular` mantém assinatura e recusa antes de executar/escrever; atualizar mensagem/documentação da pendência para `integração do laço com estatísticas e registro`.

- [x] Stub compilável da API nova; testes primeiro, RED comportamental real; implementar mínimo por grupos e observar GREEN. Não contar import/compilação acidental como RED.
- [x] Vazio com quantum positivo -> mapa vazio; quantum zero -> Entrada, inclusive vazio. Um processo [X=8, COM, Y=-4, SAIDA], quantum1 e4 -> mapa {(1):(8,-4)}; quantum usizeMAX com parada curta não aloca proporcionalmente ao quantum.
- [x] Processos diferentes/prioridades diferentes preservam todos os resultados. Contexto Pronto com PC1, X prévio e programa [X=99,Y=3,SAIDA] mantém X prévio, não reinicia a CPU.
- [x] Todos bloqueados: A3 [ES,X=9,ES,Y=4,SAIDA], B2 [ES,Y=-2,ES,X=7,SAIDA] -> A(9,4), B(7,-2); testar E/S repetida, único processo que bloqueia e quantum1/3. Prioridades todas zero e pronto zero com bloqueado positivo também concluem.
- [x] Redistribuição repetida: prioridade1, programas com vários COM antes de atribuição/SAIDA, quantum1; todos os resultados são entregues. Misturar processo curto/SAIDA e E/S antes/na fronteira do quantum.
- [x] PID duplicado -> EstadoInvalido; contextos iniciais Executando/Bloqueado ou Pronto com espera positiva -> EstadoInvalido; PC inválido ou texto sem SAIDA -> EstadoInvalido, inclusive depois de outro processo terminar. Não expor setters/test hooks ou resultados parciais para esses testes.
- [x] Preservar teste de recusa de simular e ausência de eventos; adicionar um teste com entradas oficiais pelo carregador e quantum oficial3. Calcular valores esperados lendo os arquivos, registrando-os literalmente no teste; não derivar expectativa do interpretador do produto. Essa execução única de teste não é experimento/lote.
- [x] Rodar testes focados/suíte e formatar somente arquivos próprios; reportar RED/GREEN reais, casos e limitações em task-1-report.md.

## Task 2: diagnóstico da execução ainda pendente

**Files:** editor único src/main.rs; tests/execucao.rs somente se necessário, preservando comportamentos existentes.

- [x] Atualizar somente mensagem/comentário da recusa para `integração da simulação com estatísticas e logs`. Não executar o novo laço pela main, não criar RegistroLog ou mudar código1/validação/CLI.
- [x] Rodar cargo test --test execucao; conferir execução oficial código1, stdout vazio, erro específico e nenhum log. Mudança textual de diagnóstico não exige teste redundante que espelhe a string.

## Task 3: documentação e contextos

**Files:** editor único README.md, HISTORICO.md, docs/superpowers/README.md, docs/superpowers/plans/2026-10-10-contexto-atual.md. Coordenador mantém AGENTS e este plano.

- [x] Registrar laço completo do núcleo implementado e mapa de registradores, diferenciando `executar_laco` de `simular` com resumo/registro ainda reservado e main pendente.
- [x] Próximo recorte: estatísticas e logs reais, depois main integrada, depois experimentos/relatório. Etapa3 commit `57099c4` integrada localmente à main; código/testes da etapa4 registrado em `75941d0` e contextos em `05bc657`, desenvolvidos na branch `codex/etapa4-laco-simulacao` e integrados localmente à main.
- [x] Atualizar histórico com marco de 2026-10-11, decisões e evidências reais fornecidas pelo coordenador; não antecipar hashes/revisões ou resultados. Preservar acervo.
- [x] Conferir links e diff check; relatório task-3-report.md.

## Task 4: revisão e verificação final

- [x] Revisão independente do núcleo, diagnóstico e documentação/conjunto; achados ao autor responsável.
- [x] Após cessarem edições: cargo check --all-targets, cargo test, cargo fmt --check, cargo clippy --all-targets -- -D warnings e git diff --check; registrar aprovados/falhas/ignorados.
- [x] Conferir main código1/erro atualizado/sem logs, preservação dos 12 oficiais/PDF/acervo/boilerplate, ausência de estatísticas fictícias e contratos externos intactos.
- [x] Registrar revisões aprovadas e entregar inicialmente sem commit para revisão humana.
- [x] Após autorização de dois commits, revisar o stage e registrar código/testes em `75941d0`; revisão independente pré-commit sem achados de código/contratos, com 34 testes focados aprovados. Coordenador repetiu check, suíte (92/0/0), fmt, Clippy e conferências do diff, entradas, boilerplate e recusa da execução. Contextos/documentação registrados separadamente em `05bc657` após revisão independente aprovada.
- [x] Após autorização posterior do usuário, integrar localmente à main por fast-forward de `94a9482` para `05bc657`; suíte pós-merge com 92 aprovados, zero falhas e zero ignorados. Sem push.
