# Histórico e acompanhamento do EP1

Última conferência: **2026-10-11**, base Git `7ced42f`, branch
`codex/etapa3-transicoes-es`. Estado atual: terceira etapa de despacho, transições
e E/S em um passo integrado implementada e verificada, colocada em stage pelo
usuário; revisão pré-commit e commit autorizados se o conjunto estiver correto.
Essa conferência foi aprovada sem achados; push e merge não foram solicitados. Revisão independente do núcleo em conformidade e qualidade
aprovada sem achados; revisões documental e final do conjunto aprovadas sem achados.
A segunda etapa foi registrada em `8d4d172`, integrada e publicada na `main`;
o enunciado `ep1.pdf` foi incluído como contexto em `7336c37`. O laço completo,
estatísticas, logs reais e integração de `main` continuam pendentes.

## Objetivo e escopo

Implementar o escalonador do EP1 de Sistemas Operacionais de **2026**, conforme
[enunciado na raiz](ep1.pdf), em Rust stable, edição 2021, Cargo e somente a biblioteca padrão.
A base atual preserva os 12 arquivos oficiais em `programas/`. A primeira etapa
autorizada executa instruções e quantum; a segunda implementa filas, política de
créditos e operações de espera de E/S; a terceira integra despacho, transições e
ociosidade em um passo. O laço completo, o restante do núcleo,
experimentos e relatório PDF continuam pendentes.

## Estado atual por módulo

| Módulo | Implementado | Pendente |
| --- | --- | --- |
| `modelos`, `erros` | Instruções, BCP, IDs, estados, resultados e erros | Ajustes somente se exigidos pelo núcleo |
| `carregador` | Leitura e validação de programas, prioridades e quantum | Integração com a simulação |
| `tabela_processos` | Posse dos BCPs, inserção sem sobrescrever PID, consulta e remoção | Uso pelo laço de simulação |
| `filas`, `politica` | Carga, seleção, reordenação, reinserções, bloqueio, avanço de esperas, desconto e redistribuição de créditos, consumidos pelo passo integrado | Uso pelo laço completo |
| `interpretador` | Atribuições, COM, E/S, SAIDA, validação e avanço único do PC | Integração com o laço de simulação |
| `simulador` | Quantum limitado e passo integrado: despacho, transições, E/S, tick ocioso, conclusão vazia e BCP terminado por valor | Laço completo de simulação |
| `estatisticas` | Contratos de coleta e médias | Contadores e cálculo D/P e I/D |
| `registro` | Escrita de eventos e resumo fornecido, com erros de escrita propagados | Eventos e valores reais produzidos pelo núcleo |
| `main` | Recusa argumentos, valida `programas/` no diretório corrente | Integrar simulação, log e resumo reais |

As partes restantes do núcleo retornam `ErroEp::NaoImplementado` específico.
A execução atual falha explicitamente após validar a entrada e não cria logs. Testes desses contratos
verificam a recusa; não demonstram que a simulação funciona.

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
- Manter o escopo essencial: sem CLI configurável, lote automático, CSV, threads,
  tempo real, `unsafe`, resultados experimentais inventados ou relatório fictício.
- Usar o fluxo Superpowers detalhado no AGENTS. Plano e registros recuperados
  de `58835cb` são históricos; o plano `docs/superpowers/plans/2026-10-10-contexto-atual.md`
  orienta as próximas entregas. O [plano da primeira etapa](docs/superpowers/plans/2026-10-10-etapa1-interpretador-quantum.md)
  registra o recorte CPU; o [plano da segunda etapa](docs/superpowers/plans/2026-10-10-etapa2-filas-politica.md)
  registra filas, política e esperas; o [plano da terceira etapa](docs/superpowers/plans/2026-10-11-etapa3-transicoes-es.md)
  registra o passo integrado. As etapas restantes exigem novo pedido.

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
- [ ] Próximo passo proposto: implementar o laço completo consumindo os passos,
  com testes pequenos calculados à mão de execução até a conclusão.
- [ ] Coletar estatísticas e integrar logs; testar D/P e I/D em ponto flutuante,
  denominadores, E/S e SAIDA contadas, término e despachos repetidos do mesmo PID.
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
núcleo. Verificações pré-commit deste turno executadas pelo coordenador: check de todos os
alvos, testes (77 aprovados, zero falhas e zero ignorados), fmt e Clippy com
warnings negados aprovados. Os 12 arquivos oficiais, PDF, acervo, boilerplate e
módulos fora do escopo permanecem intactos; CPU e stub preservados. Execução com
código 1, stdout vazio, erro explícito e sem logs. A revisão independente
pré-commit de conformidade e qualidade foi aprovada sem achados críticos,
importantes ou menores; oito arquivos em stage conferidos e
`git diff --cached --check` aprovado. Push e merge não foram solicitados.

## Como manter

Na mesma entrega de cada mudança de código, escopo, decisão ou pendência,
conferir e atualizar o estado, os próximos passos e a data/base acima. Acrescentar
um marco breve para mudanças relevantes, com commit existente ou indicação de
trabalho ainda sem commit. Registrar só verificações realmente executadas.
A atualização é manual durante o desenvolvimento, sem processo em segundo plano.
