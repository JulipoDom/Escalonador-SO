# Etapa 3 — integração de transições e E/S

> **Para agentes:** executar por tarefa com superpowers:subagent-driven-development. Usar TDD para os comportamentos e revisão independente antes da entrega.

**Goal:** integrar CPU, créditos e filas em um passo de escalonamento, sem implementar o laço completo de `simular`.

**Architecture:** a tabela continua proprietária dos BCPs ativos e as filas guardam somente IDs. Um passo devolve o resultado de um surto, o avanço ocioso ou a conclusão do estado vazio. O BCP terminado sai da tabela e é devolvido por valor, preservando nome, registradores e contexto para o futuro consumidor.

**Tech Stack:** Rust stable, edição 2021, Cargo, somente std.

**Spec:** ep1.pdf, README, contexto vigente e recorte aprovado na conversa: despachar, reinserir/bloquear/remover, avançar bloqueados anteriores e tratar ociosidade. A autorização atual aplica esse recorte às APIs existentes. Base 7ced42f; branch codex/etapa3-transicoes-es.

## Global Constraints

- Preserve os 12 arquivos oficiais byte a byte, o PDF e o boilerplate local.
- A tabela possui os BCPs ativos; filas guardam somente ProcessId.
- Sem unsafe, threads, tempo real, CLI configurável, lote automático ou CSV.
- Não implementar o laço completo, estatísticas, logs reais ou integração com main nesta entrega; os respectivos contratos continuam NaoImplementado.
- Não alterar os contratos de CPU, modelos, filas ou política das etapas já verificadas.
- Um editor por arquivo; correções voltam ao autor. Sem stage, commit, merge ou push nesta entrega, aguardando revisão humana.

Autorização posterior: após colocar a entrega em stage, o usuário solicitou nova revisão e autorizou o commit se o conjunto estiver correto. Essa instrução substitui a restrição anterior de stage e commit; merge, push e etapas futuras não foram solicitados.

## Review Focus

- Estado inconsistente de filas/tabela deve falhar antes de qualquer mutação do passo.
- Erro da CPU depois do despacho preserva progresso e crédito descontado; não inventa parada nem avança esperas.
- E/S na fronteira e surto curto devem ser tratados pela parada real, não pelo tamanho do quantum.
- Término não pode descartar nome/registradores nem manter PID ativo.
- Ociosidade não redistribui nem desconta créditos e nunca executa CPU; um passo ocioso avança apenas uma unidade lógica.

## Task 1: passo integrado e testes

**Files:** editor único de src/simulador.rs e novo tests/transicoes.rs. Preserve executar_quantum e simular (stub) em comportamento e assinatura; testes CPU existentes permanecem intactos.

**Interfaces:**
- Consumir TabelaProcessos e suas consultas/remoção, FilaProntos e FilaBloqueados públicas, politica::{redistribuir_se_necessario,descontar_credito}, executar_quantum.
- Produzir em simulador.rs `pub enum ResultadoPasso { Surto { resultado: ResultadoQuantum, terminado: Option<Bcp> }, Ocioso { liberados: Vec<ProcessId> }, Concluido }`, com Debug.
- Produzir `pub fn executar_passo(tabela: &mut TabelaProcessos, prontos: &mut FilaProntos, bloqueados: &mut FilaBloqueados, quantum: usize) -> Result<ResultadoPasso, ErroEp>`.

Contrato e ordem:
1. Quantum zero retorna Entrada antes de qualquer consulta/mutação, inclusive estado vazio.
2. Validar todo o estado no início: cada ID de fila existe e corresponde ao pid do BCP; filas sem duplicidade nem interseção; Pronto com espera zero pertence exatamente aos prontos, Bloqueado com espera positiva pertence exatamente aos bloqueados; cada BCP ativo está em uma das filas; nenhum Executando preexistente. Inconsistências retornam EstadoInvalido sem mutação. Não exigir limite superior artificial para espera positiva.
3. Estado vazio válido retorna Concluido. Sem prontos, executar exatamente um tick sobre snapshot dos bloqueados, reinserir liberados após empates e devolver Ocioso com IDs FIFO. Não executar CPU ou modificar créditos, mesmo que todos sejam zero.
4. Havendo prontos, redistribuir somente se necessário e reordenar estavelmente. Capturar snapshot dos bloqueados antes do surto; selecionar maior crédito, descontar exatamente um crédito e marcar Executando; executar_quantum.
5. Quantum: marcar Pronto antes de reinserir pelo contrato de quantum. E/S: bloquear com espera dois, sem descontar o próprio surto. SAIDA: remover e devolver o BCP terminado em Some, preservando PC/nome/registradores/créditos; nas outras paradas terminado=None.
6. Após a transição, avançar somente o snapshot anterior; reinserir liberados por E/S em FIFO após empates. Redistribuição ocorre antes de um próximo despacho, não em ociosidade. Devolver Surto com ResultadoQuantum real.
7. Se executar_quantum falhar, propagar erro; manter BCP Executando na tabela, fora das filas, com crédito já descontado e progresso válido preservado. Não concluir transição nem reduzir esperas. É execução abortada; recuperação automática não pertence ao recorte. Nova chamada sobre esse estado deve recusar inconsistência antes de mutação. Erros estruturais e quantum zero são diferentes: são anteriores ao despacho e não mutam estado.

- [x] Escrever testes compiláveis com stubs mínimos da nova API; executar e observar RED de comportamento ausente, não mero erro de import/compilação.
- [x] Implementar por grupos e observar GREEN, registrando comandos e falhas reais.
- [x] Testar surto Quantum de A prioridade4 e B3: após desconto A3 reinserido antes de B3, PC avançado, estado Pronto; SAIDA remove A e devolve contexto final.
- [x] Testar A prioridade6 bloqueando, B4 executando dois surtos consecutivos do mesmo PID: esperas A2,1,0, retorno com crédito5 à frente e retomada após E/S. Testar surtos curtos por SAIDA e por E/S e E/S na fronteira.
- [x] Testar snapshot anterior: o novo bloqueado fica com espera2 enquanto o anterior diminui; término de outro processo também diminui espera. Liberados com espera inicial1 na fila A,B saem FIFO e entram após prontos de crédito igual. Fixtures de bloqueados devem ser montadas via bloquear e ajuste explícito de espera para este cenário.
- [x] Testar sequência manual de todos bloqueados: A3 e B2 bloqueiam; tick ocioso libera A e mantém B esperando, sem alteração de PC/registradores/créditos. Chamadas posteriores removem terminados e Concluido só é retornado com tabela/filas vazias. Não adicionar laço automático ao produto.
- [x] Testar redistribuição antes do despacho considerando bloqueados; prontos zero e bloqueado positivo rotacionam FIFO; prioridades todas zero progridem; empates após redistribuição são estáveis.
- [x] Testar quantum zero sem mutação; IDs ausentes, BCP ausente de fila, estado Executando, espera incompatível, pid alterado e filas em estado incorreto sem mutação. Comparar snapshots determinísticos por PID, sem depender da ordem de HashMap.
- [x] Testar erro de CPU após instruções válidas: crédito descontado, PC/registradores preservados, Executando fora da fila, espera anterior intacta; nova chamada recusa estado abortado sem mais mutação. Testar PC inicialmente inválido como despacho abortado, sem inventar conclusão.
- [x] Formatar somente arquivos próprios, rodar testes focados e suíte, auto revisar. Não coletar estatísticas nem escrever eventos.

## Task 2: documentação e contextos

**Files:** editor único README.md, HISTORICO.md, docs/superpowers/README.md, docs/superpowers/plans/2026-10-10-contexto-atual.md. Coordenador mantém AGENTS e este plano.

- [x] Após aprovação do núcleo, atualizar estado: passo integrado verificado, laço completo próximo, logs/estatísticas/main pendentes. Explicar resultado, contexto terminado e erro abortado sem afirmar recuperação automática.
- [x] Registrar branch/base reais, TDD e revisões realmente realizados, marco de terceira etapa sem commit. Preservar acervo e dados das entregas publicadas anteriores.
- [x] Registrar somente verificações globais fornecidas pelo coordenador; conferir links locais e whitespace. Não antecipar revisão final.

## Task 3: revisão e verificação de entrega

- [x] Revisão independente Task1 (spec e qualidade), correções ao autor; revisão documental e final do conjunto somente leitura.
- [x] Após edições cessarem: cargo check --all-targets, cargo test, cargo fmt --check, cargo clippy --all-targets -- -D warnings; reportar aprovados, falhas e ignorados.
- [x] Conferir execução ainda com código1/erro explícito e sem logs, contratos simular/estatísticas ainda pendentes, 12 entradas byte a byte com base; módulos fora do recorte, acervo/PDF/boilerplate preservados.
- [x] Atualizar contextos com resultados reais e entregar sem commit para revisão humana.
