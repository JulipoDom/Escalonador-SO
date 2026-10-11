# Histórico e acompanhamento do EP1

Última conferência: **2026-10-10**, base Git `4210a1c`.
Trabalho atual: primeira etapa de interpretador e quantum na branch
`codex/etapa1-interpretador-quantum`, implementada nesta entrega. Implementação e revisão
da etapa CPU concluídas; verificações finais e revisão independente do conjunto aprovadas.

## Objetivo e escopo

Implementar o escalonador do EP1 de Sistemas Operacionais de **2026**, conforme
[enunciado na raiz](ep1.pdf), em Rust stable, edição 2021, Cargo e somente a biblioteca padrão.
A base atual preserva os 12 arquivos oficiais em `programas/`. A primeira etapa
autorizada executa instruções e quantum; o restante do núcleo, experimentos e
relatório PDF continuam pendentes.

## Estado atual por módulo

| Módulo | Implementado | Pendente |
| --- | --- | --- |
| `modelos`, `erros` | Instruções, BCP, IDs, estados, resultados e erros | Ajustes somente se exigidos pelo núcleo |
| `carregador` | Leitura e validação de programas, prioridades e quantum | Integração com a simulação |
| `tabela_processos` | Posse dos BCPs, inserção sem sobrescrever PID, consulta e remoção | Uso pelo laço de simulação |
| `filas`, `politica` | Estruturas de IDs e contratos documentados | Ordenação, seleção, reinserção, E/S e créditos |
| `interpretador` | Atribuições, COM, E/S, SAIDA, validação e avanço único do PC | Integração com o laço de simulação |
| `simulador` | Quantum limitado, contagem, paradas e propagação de erros | Laço de simulação, despacho, transições e tempo lógico |
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
- E/S espera dois surtos posteriores; o surto que bloqueia não reduz a própria espera.
  Ociosidade avança somente a espera lógica, sem instruções, créditos ou despachos.
- Manter o escopo essencial: sem CLI configurável, lote automático, CSV, threads,
  tempo real, `unsafe`, resultados experimentais inventados ou relatório fictício.
- Usar o fluxo Superpowers detalhado no AGENTS. Plano e registros recuperados
  de `58835cb` são históricos; o plano `docs/superpowers/plans/2026-10-10-contexto-atual.md`
  orienta as próximas entregas. O [plano da primeira etapa](docs/superpowers/plans/2026-10-10-etapa1-interpretador-quantum.md)
  registra o recorte autorizado; as etapas restantes exigem novo pedido.

## Sequência proposta de implementação e verificação

- [x] Preparar modelos, entrada validada, tabela e escritor de log.
- [x] Implementar interpretador e quantum; testes de atribuições, COM, PC único,
  retomada após E/S, SAIDA, fronteiras, quantum zero e erros com progresso válido
  aprovados pelo autor; revisão independente de conformidade e qualidade sem achados.
- [ ] Primeiro próximo passo proposto: implementar filas e política; testar empates, reinserções, desconto saturado,
  bloqueados com crédito, redistribuição de todos os ativos e prioridades zero.
- [ ] Implementar transições e E/S; testar dois surtos posteriores, surtos curtos,
  surtos consecutivos do mesmo PID, liberações FIFO e ausência de IDs duplicados.
- [ ] Integrar simulação; testar término com tabela vazia e ociosidade sem consumo
  de instruções, créditos ou despachos.
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
| 2026-10-10 | Entrega sobre a base `4210a1c` | Primeira etapa na branch `codex/etapa1-interpretador-quantum`: interpretador e quantum com TDD real e revisão independente; contexto atualizado, restante do núcleo pendente. |

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

Verificação final desta entrega executada pelo coordenador: `cargo check --all-targets`,
`cargo test`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` e
`git diff --check` aprovados. Testes: 39 aprovados, zero falhas e zero ignorados
(14 no alvo `simulador`, incluindo o contrato de recusa); alvos unitários e doc-tests
sem testes. Os 12 arquivos oficiais conferem byte a byte com HEAD e com as assinaturas
iniciais; os 12 documentos do acervo (`docs/fluxo/` e plano antigo), PDF e boilerplate
locais permanecem intactos. Execução: código 1, stdout vazio e stderr
`Não implementado: laço de simulação, despacho e transições de processos`, sem logs
antes ou depois. Revisões independentes da documentação e do conjunto aprovadas em conformidade e
qualidade, sem achados críticos, importantes ou menores. Nenhuma correção de CPU
necessária; filas e política continuam como próximo passo proposto.

Após concluir a implementação, o usuário autorizou preparar o stage das alterações
da etapa, excluindo `BOILERPLATE_EP1_ESCALONADOR.md`, revisar novamente e criar o
commit após nova verificação. Essa autorização substitui a condição anterior de
trabalho sem commit automático; não amplia o escopo do núcleo. Os resultados da
nova revisão e verificação foram conferidos pelo coordenador: revisão antes do
commit aprovada em conformidade e qualidade, sem achados; check, testes (39/0/0),
fmt, Clippy e conferência do diff em stage aprovados. Nove arquivos da etapa em
stage, sem PDF ou boilerplate; entradas, acervo e documentos locais intactos.
Execução com código 1, stdout vazio, pendência explícita e sem logs. Entrega apta
para o commit autorizado; nenhum hash é antecipado.

## Como manter

Na mesma entrega de cada mudança de código, escopo, decisão ou pendência,
conferir e atualizar o estado, os próximos passos e a data/base acima. Acrescentar
um marco breve para mudanças relevantes, com commit existente ou indicação de
trabalho ainda sem commit. Registrar só verificações realmente executadas.
A atualização é manual durante o desenvolvimento, sem processo em segundo plano.
