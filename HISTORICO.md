# Histórico e acompanhamento do EP1

Última conferência: **2026-10-11**, base Git `434f43b`, branch `main`.
Estado atual: código e testes da quarta etapa registrados em `75941d0` e contextos
em `05bc657`, integrados localmente à `main` por fast-forward autorizado de
`94a9482`. Após autorização do usuário, a publicação em `origin/main` foi concluída
até `434f43b`. Suíte pós-merge com 92 aprovados, zero falhas e zero ignorados.
`executar_laco` consome os passos até a conclusão e devolve
os registradores finais reais por PID. Revisões independentes de conformidade e
qualidade do núcleo aprovadas sem achados; suíte com 92 aprovados, zero falhas e
zero ignorados. As revisões documental e final desta entrega foram aprovadas em conformidade e
qualidade, sem achados críticos, importantes ou menores.
A terceira etapa foi registrada em `57099c4` e integrada localmente à `main`
por fast-forward autorizado de `7ced42f`, naquele momento sem push, com 77/0/0 pós-merge.
A segunda etapa foi registrada em `8d4d172`, integrada e publicada na `main`;
o enunciado `ep1.pdf` foi incluído como contexto em `7336c37`. Estatísticas,
logs reais, `simular` com resumo/registro e integração de `main` continuam pendentes.

## Objetivo e escopo

Implementar o escalonador do EP1 de Sistemas Operacionais de **2026**, conforme
[enunciado na raiz](ep1.pdf), em Rust stable, edição 2021, Cargo e somente a biblioteca padrão.
A base atual preserva os 12 arquivos oficiais em `programas/`. A primeira etapa
autorizada executa instruções e quantum; a segunda implementa filas, política de
créditos e operações de espera de E/S; a terceira integra despacho, transições e
ociosidade em um passo; a quarta completa o laço do núcleo até a conclusão.
Estatísticas, logs reais, execução principal integrada, experimentos e relatório
PDF continuam pendentes.

## Estado atual por módulo

| Módulo | Implementado | Pendente |
| --- | --- | --- |
| `modelos`, `erros` | Instruções, BCP, IDs, estados, resultados e erros | Ajustes somente se exigidos pelo núcleo |
| `carregador` | Leitura e validação de programas, prioridades e quantum | Integração com a simulação |
| `tabela_processos` | Posse dos BCPs, inserção sem sobrescrever PID, consulta e remoção, usadas pelo laço | — |
| `filas`, `politica` | Carga, seleção, reordenação, reinserções, bloqueio, avanço de esperas, desconto e redistribuição de créditos, consumidos pelo passo e pelo laço | — |
| `interpretador` | Atribuições, COM, E/S, SAIDA, validação e avanço único do PC, consumidos pelo laço | — |
| `simulador` | Quantum, passo integrado e `executar_laco` até a conclusão, com mapa de registradores finais reais | `simular` com resumo e registro reais |
| `estatisticas` | Contratos de coleta e médias | Contadores e cálculo D/P e I/D |
| `registro` | Escrita de eventos e resumo fornecido, com erros de escrita propagados | Eventos e valores reais produzidos pelo núcleo |
| `main` | Recusa argumentos, valida `programas/` no diretório corrente | Integrar simulação, log e resumo reais |

`simular` e estatísticas retornam `ErroEp::NaoImplementado` específico. A execução
atual não chama o laço: após validar a entrada, falha com
`Não implementado: integração da simulação com estatísticas e logs` e não cria logs.
Testes desses contratos verificam a recusa; os testes de `executar_laco` comprovam
a conclusão do núcleo e os registradores finais, sem integrar resumo ou registro.

## Decisões vigentes

- A tabela possui os BCPs; filas guardam somente `ProcessId`. Empréstimos curtos
  dão acesso ao contexto, sem duplicá-lo ou compartilhar ownership sem necessidade.
- Cada instrução, inclusive E/S e SAIDA, consome uma unidade e avança o PC uma vez.
  E/S e SAIDA prevalecem sobre Quantum na fronteira; a retomada usa a instrução seguinte.
- PC inválido retorna `EstadoInvalido` sem mutação da instrução inválida. Quantum zero
  retorna `Entrada` antes de ler PC, sem modificar o BCP. Erro depois de progresso válido
  preserva PC e registradores já atualizados; identidade, programa, estado, prioridade,
  créditos e espera não são alterados pela CPU.
- Créditos começam na prioridade, caem uma vez por despacho sem ficar negativos
  e só são redistribuídos quando todos os ativos, inclusive bloqueados, têm zero.
- Desempates, reinserções e exceções FIFO seguem as convenções do README e do EP.
  Seleção não altera estado: o passo integrado define Executando e recoloca Pronto
  antes de reinserir. Filas validam existência, estado e duplicidade antes de modificar.
- E/S espera dois surtos posteriores; o surto que bloqueia não reduz a própria espera.
  As operações implementadas validam o snapshot anterior integralmente, descontam
  somente seus IDs e liberam elegíveis na ordem FIFO da fila. O passo integrado
  aplica isso ao fim dos surtos; em ociosidade, avança uma unidade lógica,
  sem instruções, alteração de créditos ou despachos.
- `ResultadoPasso` distingue Surto com resultado real e BCP terminado opcional,
  Ocioso com IDs liberados e Concluido para estado vazio válido. SAIDA remove o
  BCP e devolve seu nome, programa, PC, registradores e crédito final por valor.
- Quantum zero e inconsistências estruturais são recusados sem mutação. Erro de
  CPU após despacho preserva Executando fora das filas, crédito descontado e
  progresso válido; não avança esperas nem aplica transição. Sem recuperação
  automática, a chamada seguinte rejeita esse estado antes de mutar.
- `executar_laco` prepara tabela e filas e consome o passo até `Concluido`, sem
  limite artificial. Captura os registradores dos BCPs terminados por PID; entrada
  vazia com quantum positivo devolve mapa vazio. Quantum zero é recusado antes
  da preparação; erros são propagados, inclusive após progresso válido. Não coleta
  eventos ou estatísticas. `simular` permanece reservado antes de executar ou escrever.
- Manter o escopo essencial: sem CLI configurável, lote automático, CSV, threads,
  tempo real, `unsafe`, resultados experimentais inventados ou relatório fictício.
- Usar o fluxo Superpowers detalhado no AGENTS. Plano e registros recuperados
  de `58835cb` são históricos; o plano `docs/superpowers/plans/2026-10-10-contexto-atual.md`
  orienta as próximas entregas. O [plano da primeira etapa](docs/superpowers/plans/2026-10-10-etapa1-interpretador-quantum.md)
  registra o recorte CPU; o [plano da segunda etapa](docs/superpowers/plans/2026-10-10-etapa2-filas-politica.md)
  registra filas, política e esperas; o [plano da terceira etapa](docs/superpowers/plans/2026-10-11-etapa3-transicoes-es.md)
  registra o passo integrado; o [plano da quarta etapa](docs/superpowers/plans/2026-10-11-etapa4-laco-simulacao.md)
  registra o laço completo. As etapas restantes exigem novo pedido.

## Sequência proposta de implementação e verificação

- [x] Preparar modelos, entrada validada, tabela e escritor de log.
- [x] Implementar interpretador e quantum; testes de atribuições, COM, PC único,
  retomada após E/S, SAIDA, fronteiras, quantum zero e erros com progresso válido
  aprovados pelo autor; revisão independente de conformidade e qualidade sem achados.
- [x] Implementar filas, política e operações de espera de E/S; testes de empates,
  reinserções, desconto saturado, bloqueados com crédito, redistribuição, prioridades
  zero, snapshots, liberação FIFO e erros sem mutação aprovados; revisão independente
  de conformidade e qualidade sem achados.
- [x] Integrar despacho, transições e E/S em um passo; testes de dois surtos
  posteriores, surtos curtos, repetição do PID, liberações FIFO, estados inválidos,
  aborto da CPU, término com contexto e tick ocioso aprovados; revisão independente
  de conformidade e qualidade do núcleo sem achados.
- [x] Implementar o laço completo consumindo os passos até a conclusão e devolver
  registradores finais reais; 15 testes novos, incluindo as entradas oficiais,
  aprovados e revisão independente de conformidade e qualidade sem achados.
- [ ] Próximo passo proposto: coletar estatísticas e integrar logs reais; testar D/P
  e I/D em ponto flutuante, denominadores, E/S e SAIDA contadas, término e
  despachos repetidos do mesmo PID.
- [ ] Integrar `main` somente depois das estatísticas e logs reais, substituindo
  a recusa pela execução com eventos e resumo reais.
- [ ] Executar manualmente ao menos dez quantums uniformemente distribuídos em
  intervalo útil; reunir logs, médias, tabelas e gráficos para justificar o quantum no PDF.

Ideia dentro do escopo: usar programas pequenos com resultado calculado à mão
antes dos arquivos oficiais, para localizar erros de ordem e contagem. É uma
proposta de validação, não uma funcionalidade adicional prometida.

## Marcos

| Data | Referência | Marco |
| --- | --- | --- |
| 2026-10-10 | `189b01d` | Base inicial do projeto. |
| 2026-10-10 | `58835cb` | Revisão da propagação de erros de saída e registro da verificação da base. |
| 2026-10-10 | `ffc8849` | Simplificação para a base essencial do EP1; núcleo permanece reservado. |
| 2026-10-10 | `748077f` | Criação deste acompanhamento, link no README e regra de manutenção no AGENTS. |
| 2026-10-10 | Entrega sobre `748077f` | Recuperação do acervo Superpowers de `58835cb`, fluxo detalhado no AGENTS e plano adaptado ao contexto atual. |
| 2026-10-10 | `7b92a6a` | Primeira etapa de interpretador e quantum com TDD real e revisão independente, integrada à `main`; restante do núcleo pendente. |
| 2026-10-10 | Entrega sem commit sobre `7b92a6a` | Segunda etapa na branch `codex/etapa2-filas-politica`: filas, política de créditos e operações de espera de E/S com TDD real e revisão independente; integração ao laço pendente. |
| 2026-10-11 | Trabalho sobre `7b92a6a` | Revisão pré-commit, stage e commit da segunda etapa autorizados na branch atual, condicionados à conferência do conjunto. Boilerplate local excluído do rastreio Git e protegido por regra em `.gitignore`; verificações de código e revisão independente pré-commit aprovadas. |
| 2026-10-11 | `8d4d172` | Segunda etapa registrada após revisão e conferência do stage, integrada localmente à `main` por fast-forward autorizado, sem push; suíte pós-merge aprovada com 59/0/0. |
| 2026-10-11 | Entrega sobre `888a1f8` | Inclusão do enunciado `ep1.pdf` de 2026 como contexto do projeto, preservado byte a byte, e publicação dos commits locais em `origin/main` autorizadas pelo usuário. |
| 2026-10-11 | `7336c37` | Enunciado `ep1.pdf` incluído como contexto do projeto; publicação autorizada dos commits locais em `origin/main` concluída. |
| 2026-10-11 | Entrega sem commit sobre `7ced42f` | Terceira etapa na branch `codex/etapa3-transicoes-es`: passo integrado de despacho, transições e E/S com TDD real, revisão independente do núcleo e 77/0/0; revisão final aprovada sem achados; laço completo pendente. |
| 2026-10-11 | Trabalho sobre `7ced42f` | Usuário colocou a terceira etapa em stage e autorizou revisão pré-commit e commit se o conjunto estiver correto; conferência aprovada sem achados, sem pedido de push ou merge. |
| 2026-10-11 | `57099c4` | Terceira etapa registrada e integrada localmente à `main` por fast-forward autorizado de `7ced42f`, sem push; suíte pós-merge aprovada com 77/0/0. |
| 2026-10-11 | Entrega sem commit sobre `94a9482` | Quarta etapa na branch `codex/etapa4-laco-simulacao`: laço completo do núcleo, mapa de registradores finais reais, TDD e revisão independente do núcleo sem achados; 92/0/0. Estatísticas, logs reais e execução principal integrada pendentes; revisões documental e final aprovadas sem achados. |
| 2026-10-11 | `75941d0` | Código e testes do laço completo registrados após autorização de dois commits separados e revisão independente pré-commit sem achados; naquele momento, contextos em preparação para o segundo commit, sem merge ou push. |
| 2026-10-11 | `05bc657` | Contextos da quarta etapa registrados em commit separado; após autorização do usuário, ambos os commits integrados localmente à `main` por fast-forward de `94a9482`, sem push. Suíte pós-merge aprovada com 92/0/0. |
| 2026-10-11 | `434f43b` | Após autorização do usuário, publicação dos cinco commits locais em `origin/main` concluída, avançando a remota de `7ced42f` para `434f43b`, incluindo as etapas três e quatro. |

Verificação da entrega `748077f`: `cargo check --all-targets`, `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings` e `git diff --check` aprovados;
`cargo test` com 28 aprovados, zero falhas e zero ignorados.

Verificação da adaptação Superpowers: check, test (28 aprovados, zero falhas e
zero ignorados), fmt e Clippy novamente aprovados. Execução com entradas oficiais
retornou código 1 e pendência explícita, sem logs; os 12 arquivos conferem com o ZIP.
Os 11 documentos recuperados conferem byte a byte com `58835cb`; links novos
válidos e `git diff --check` aprovado. Código e entradas permanecem inalterados.

Verificação da primeira etapa pelo autor do núcleo: RED real com quatro falhas
comportamentais de instrução e, depois do GREEN do interpretador, nove falhas
comportamentais de quantum. GREEN final: `cargo test --test simulador` com 14
aprovados e `cargo test` com 39 testes de integração aprovados, zero falhas e zero
ignorados; alvos unitários e doc-tests sem testes. Formatação dos três arquivos
próprios e conferência de whitespace nesses arquivos aprovadas. Revisão independente
de conformidade e qualidade da Task 1 aprovada sem achados. São evidências do recorte
CPU e dos contratos restantes, não resultados experimentais nem simulação implementada.

Verificação final da primeira etapa executada pelo coordenador: `cargo check --all-targets`,
`cargo test`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` e
`git diff --check` aprovados. Testes: 39 aprovados, zero falhas e zero ignorados
(14 no alvo `simulador`, incluindo o contrato de recusa); alvos unitários e doc-tests
sem testes. Os 12 arquivos oficiais conferem byte a byte com HEAD e com as assinaturas
iniciais; os 12 documentos do acervo (`docs/fluxo/` e plano antigo), PDF e boilerplate
locais permanecem intactos. Execução: código 1, stdout vazio e stderr
`Não implementado: laço de simulação, despacho e transições de processos`, sem logs
antes ou depois. Revisões independentes da documentação e do conjunto aprovadas em conformidade e
qualidade, sem achados críticos, importantes ou menores. Nenhuma correção de CPU
necessária; naquele momento, filas e política eram o próximo passo proposto.

Após concluir a implementação, o usuário autorizou preparar o stage das alterações
da etapa, excluindo `BOILERPLATE_EP1_ESCALONADOR.md`, revisar novamente e criar o
commit após nova verificação. Essa autorização substitui a condição anterior de
trabalho sem commit automático; não amplia o escopo do núcleo. Os resultados da
nova revisão e verificação foram conferidos pelo coordenador: revisão antes do
commit aprovada em conformidade e qualidade, sem achados; check, testes (39/0/0),
fmt, Clippy e conferência do diff em stage aprovados. Nove arquivos da etapa em
stage, sem PDF ou boilerplate; entradas, acervo e documentos locais intactos.
Execução com código 1, stdout vazio, pendência explícita e sem logs. O commit autorizado foi criado como `7b92a6a` e integrado à `main`.

## Verificação da segunda etapa

O autor do núcleo registrou RED comportamental real em grupos de política, carga e
seleção, reinserções, bloqueio e avanço de esperas. GREEN final dos alvos `filas` e
`politica`: 23 aprovados, zero falhas e zero ignorados; suíte completa: 59/0/0.
Clippy focado detectou `unnecessary_map_or`; após depuração e correção, passou sem
warnings. Revisão independente de conformidade e qualidade do núcleo aprovada sem achados.

Verificações executadas pelo coordenador após cessarem as edições do núcleo:
`cargo check --all-targets`, `cargo test`, `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings` e `git diff --check` aprovados.
Testes: 59 aprovados, zero falhas e zero ignorados; os contratos pendentes de
simulação e estatísticas continuam sendo testes de recusa. `cargo run --bin Escalonador`
encerrou com código 1 e `Não implementado: laço de simulação, despacho e transições de processos`,
sem logs. Os 12 arquivos oficiais conferem byte a byte com `7b92a6a`;
CPU, execução, estatísticas e registro permaneceram intactos. Não foram produzidos
resultados de simulação ou experimentos. Naquela verificação, a segunda etapa estava
sem stage, commit, push ou merge; a integração das transições e do laço ainda exige novo pedido.

Em 2026-10-11, o usuário autorizou revisar o conjunto, preparar o stage e criar o
commit da segunda etapa na branch atual se tudo estiver correto. Essa autorização
substitui a condição anterior de trabalho sem stage ou commit; não amplia o escopo
do núcleo. Também autorizou excluir `BOILERPLATE_EP1_ESCALONADOR.md` do rastreio Git,
preservando o arquivo local. O arquivo já não era versionado e recebeu a regra
`/BOILERPLATE_EP1_ESCALONADOR.md` em `.gitignore`; `git check-ignore` confirmou que
o arquivo está ignorado.

Verificações pré-commit executadas pelo coordenador em 2026-10-11:
`cargo check --all-targets`, `cargo test`, `cargo fmt --check` e
`cargo clippy --all-targets -- -D warnings` aprovados. Testes: 59 aprovados, zero
falhas e zero ignorados. Os 12 arquivos oficiais conferem byte a byte com HEAD.
A execução encerrou com código 1, stdout vazio, stderr com a pendência explícita
do laço de simulação e sem logs. Revisão independente pré-commit de conformidade
e qualidade aprovada, sem achados críticos, importantes ou menores. O coordenador
conferiu os 11 arquivos exatos em stage e criou o commit autorizado `8d4d172` na
branch `codex/etapa2-filas-politica`.

Após autorização do usuário, o coordenador integrou a segunda etapa localmente
à `main` por fast-forward de `7b92a6a` para `8d4d172`, sem push. A suíte executada
após o merge teve 59 aprovados, zero falhas e zero ignorados. Não houve nova
implementação; transições e laço de simulação permanecem pendentes.

Na entrega sobre `888a1f8`, o usuário autorizou versionar `ep1.pdf`, o enunciado
de 2026 usado como contexto do projeto, preservando seu conteúdo byte a byte.
Também autorizou enviar os commits locais para `origin/main` após essa inclusão.
Esse PDF é o enunciado existente; o relatório do trabalho continua pendente.
O enunciado foi incluído no commit `7336c37` (`docs: add EP1 specification as
project context`). O envio autorizado para `origin/main` retornou código 0 e
avançou a `main` remota de `4210a1c` para `7336c37`.

## Verificação da terceira etapa

O autor do núcleo executou TDD em três grupos compiláveis: RED de 3, 6 e 9 falhas
comportamentais por implementação ausente, seguidos de GREEN com 3, 9 e 18 testes
no alvo `transicoes`, respectivamente. Os 18 testes calculados à mão cobrem
validação anterior ao despacho, redistribuição, empates e FIFO, Quantum, E/S,
SAIDA, surtos curtos e consecutivos do mesmo PID, snapshots, ociosidade, conclusão
vazia e erros da CPU com progresso preservado. Revisão independente do núcleo
em conformidade e qualidade aprovada sem achados. `executar_quantum` e o stub
`simular` foram preservados; nenhum consumidor de logs ou estatísticas foi integrado.

Verificações repetidas pelo coordenador após as edições documentais:
`cargo check --all-targets`, `cargo test`, `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings` e `git diff --check` aprovados.
Testes: 77 aprovados, zero falhas e zero ignorados, incluindo os 18 novos testes
de transições. Os contratos pendentes continuam comprovando recusa, não simulação
completa. A execução encerrou com código 1, stdout vazio e erro explícito de laço
pendente, sem logs. Os 12 arquivos oficiais, PDF, acervo e boilerplate local
permanecem byte a byte como na base `7ced42f`; modelos, filas, política, execução,
registro, estatísticas e Cargo permanecem intactos. Naquela entrega, não houve
stage, commit, push ou merge; revisões documental e final do conjunto aprovadas
em conformidade e qualidade, sem achados críticos, importantes ou menores.

Após colocar a terceira etapa em stage, o usuário autorizou revisar o conjunto e
criar o commit na branch atual se tudo estiver correto. Essa autorização substitui
a condição anterior de trabalho sem stage ou commit; não amplia o escopo do
núcleo. Verificações pré-commit daquela entrega executadas pelo coordenador: check de todos os
alvos, testes (77 aprovados, zero falhas e zero ignorados), fmt e Clippy com
warnings negados aprovados. Os 12 arquivos oficiais, PDF, acervo, boilerplate e
módulos fora do escopo permanecem intactos; CPU e stub preservados. Execução com
código 1, stdout vazio, erro explícito e sem logs. A revisão independente
pré-commit de conformidade e qualidade foi aprovada sem achados críticos,
importantes ou menores; oito arquivos em stage conferidos e
`git diff --cached --check` aprovado. Naquele momento, push e merge não haviam
sido solicitados.

O commit autorizado da terceira etapa foi criado como `57099c4`
(`feat: integrate process transitions and I/O scheduling steps`). Após autorização
do usuário, o coordenador integrou `codex/etapa3-transicoes-es` localmente à `main`
por fast-forward de `7ced42f` para `57099c4`, sem push. `cargo test` pós-merge teve
77 aprovados, zero falhas e zero ignorados. Naquele momento, o laço completo
continuava pendente.

## Verificação da quarta etapa

O autor do núcleo registrou RED compilável de 2 falhas comportamentais para
entrada vazia e quantum zero, seguido de GREEN com 2 aprovados. Em seguida,
registrou 13 novas falhas comportamentais antes de implementar o laço; GREEN do
alvo `laco` com 15 aprovados, zero falhas e zero ignorados. Suíte completa com
92/0/0, repetida após formatar somente os arquivos próprios. As expectativas
foram calculadas à mão, incluindo os registradores finais dos dez arquivos
oficiais. Cobertura de retomadas, registradores prévios, E/S consecutivas,
prioridades zero, redistribuição, fronteiras, estados inválidos, duplicidade e
erros depois de progresso. Revisões independentes de conformidade e qualidade
do núcleo aprovadas sem achados. `executar_passo` e `executar_quantum` preservados;
`simular` continua recusando a integração com estatísticas e registro.

Verificações executadas pelo coordenador após cessarem as edições de código
e repetidas após as edições documentais:
`cargo check --all-targets`, `cargo test`, `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings` e `git diff --check` aprovados.
Testes: 92 aprovados, zero falhas e zero ignorados, incluindo os 15 novos do laço.
A execução manual de `cargo run --quiet --bin Escalonador` retornou código 1,
stdout vazio e stderr `Não implementado: integração da simulação com estatísticas e logs`,
sem logs. `main` recebeu somente diagnóstico e comentário atualizados, sem
consumir o laço; seus testes anteriores permanecem intactos. Testes de recusa
não comprovam integração de saída; testes do laço comprovam execução do núcleo
até a conclusão, sem medidas ou eventos fictícios.

Os 12 arquivos oficiais, PDF, acervo e boilerplate local permanecem byte a byte
como na base `94a9482`. Modelos, filas, política, estatísticas, registro e Cargo
permanecem intactos. Na entrega anterior ao pedido de commit, não houve stage,
commit, merge ou push. As revisões
documental e final do conjunto foram aprovadas em conformidade e qualidade, sem
achados críticos, importantes ou menores. Os 35 links locais e as dez expectativas
oficiais foram conferidos pelo revisor. Próximo recorte proposto:
estatísticas e logs reais, depois execução principal integrada e, por fim,
experimentos e relatório.

Após colocar as alterações em stage, o usuário autorizou revisão e registro em
dois commits: código/testes separados dos contextos. Essa autorização substitui
a condição anterior de trabalho sem stage ou commit e não amplia o escopo do
núcleo. Verificações frescas do coordenador antes do commit: `cargo check --all-targets`,
`cargo test` (92 aprovados, zero falhas e zero ignorados), `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings` e conferências do diff aprovados.
As 12 entradas conferem byte a byte com `94a9482`; boilerplate local preservado,
ignorado e fora do rastreio Git. Execução com código 1, stdout vazio, diagnóstico
específico de integração pendente e sem novo log.

A revisão independente pré-commit de código e contratos foi aprovada sem achados;
o revisor executou 34 testes nos alvos `laco`, `simulador` e `execucao`, todos
aprovados, conferiu os resultados finais dos dez programas oficiais e o stage.
O coordenador criou o commit `75941d0` (`feat: implement complete core simulation loop`)
com exatamente `src/main.rs`, `src/simulador.rs` e `tests/laco.rs`. Naquele momento,
os contextos estavam em preparação para o segundo commit autorizado, sem hash
antecipado e sem merge ou push.

Os contextos foram registrados em `05bc657` (`docs: update context and history for core simulation loop`).
Após autorização do usuário, o coordenador integrou os commits `75941d0` e
`05bc657` localmente à `main` por fast-forward de `94a9482` para `05bc657`, sem
push. A suíte pós-merge executada pelo coordenador teve 92 aprovados, zero falhas
e zero ignorados. Estatísticas, logs reais e execução principal integrada
continuam pendentes; a integração Git não amplia esse escopo.

Após autorização do usuário para finalizar com push, o envio para `origin/main`
retornou código 0 e avançou a remota de `7ced42f` para `434f43b`, publicando os
cinco commits locais, incluindo as etapas três e quatro. A publicação não amplia
o escopo implementado; estatísticas, logs reais e execução principal integrada
continuam pendentes. Não houve nova verificação de código neste registro.

## Como manter

Na mesma entrega de cada mudança de código, escopo, decisão ou pendência,
conferir e atualizar o estado, os próximos passos e a data/base acima. Acrescentar
um marco breve para mudanças relevantes, com commit existente ou indicação de
trabalho ainda sem commit. Registrar só verificações realmente executadas.
A atualização é manual durante o desenvolvimento, sem processo em segundo plano.
