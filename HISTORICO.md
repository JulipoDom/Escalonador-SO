# Histórico e acompanhamento do EP1

Última conferência: **2026-10-10**, fontes e README na base Git `ffc8849`.
Esta entrega acrescenta o acompanhamento documental à base indicada acima.

## Objetivo e escopo

Implementar o escalonador do EP1 de Sistemas Operacionais de **2026**, conforme
`../ep1.pdf`, em Rust stable, edição 2021, Cargo e somente a biblioteca padrão.
A base atual preserva os 12 arquivos oficiais em `programas/` e reserva o núcleo
para implementação posterior. Experimentos e relatório PDF continuam pendentes.

## Estado atual por módulo

| Módulo | Implementado | Pendente |
| --- | --- | --- |
| `modelos`, `erros` | Instruções, BCP, IDs, estados, resultados e erros | Ajustes somente se exigidos pelo núcleo |
| `carregador` | Leitura e validação de programas, prioridades e quantum | Integração com a simulação |
| `tabela_processos` | Posse dos BCPs, inserção sem sobrescrever PID, consulta e remoção | Uso pelo laço de simulação |
| `filas`, `politica` | Estruturas de IDs e contratos documentados | Ordenação, seleção, reinserção, E/S e créditos |
| `interpretador` | Contrato explícito de execução | Instruções, registradores e avanço do PC |
| `simulador` | Contratos de quantum, despacho e transições | Execução e tempo lógico |
| `estatisticas` | Contratos de coleta e médias | Contadores e cálculo D/P e I/D |
| `registro` | Escrita de eventos e resumo fornecido, com erros de escrita propagados | Eventos e valores reais produzidos pelo núcleo |
| `main` | Recusa argumentos, valida `programas/` no diretório corrente | Integrar simulação, log e resumo reais |

O núcleo reservado retorna `ErroEp::NaoImplementado`. A execução atual falha
explicitamente após validar a entrada e não cria logs. Testes desses contratos
verificam a recusa; não demonstram que a simulação funciona.

## Decisões vigentes

- A tabela possui os BCPs; filas guardam somente `ProcessId`. Empréstimos curtos
  dão acesso ao contexto, sem duplicá-lo ou compartilhar ownership sem necessidade.
- Cada instrução, inclusive E/S e SAIDA, consome uma unidade e avança o PC uma vez.
- Créditos começam na prioridade, caem uma vez por despacho sem ficar negativos
  e só são redistribuídos quando todos os ativos, inclusive bloqueados, têm zero.
- Desempates, reinserções e exceções FIFO seguem as convenções do README e do EP.
- E/S espera dois surtos posteriores; o surto que bloqueia não reduz a própria espera.
  Ociosidade avança somente a espera lógica, sem instruções, créditos ou despachos.
- Manter o escopo essencial: sem CLI configurável, lote automático, CSV, threads,
  tempo real, `unsafe`, resultados experimentais inventados ou relatório fictício.

## Sequência proposta de implementação e verificação

- [x] Preparar modelos, entrada validada, tabela e escritor de log.
- [ ] Implementar interpretador e quantum; testar atribuições, COM, PC único,
  retomada após E/S, SAIDA e parada antes do limite do quantum.
- [ ] Implementar filas e política; testar empates, reinserções, desconto saturado,
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
| 2026-10-10 | Entrega documental sobre `ffc8849` | Criação deste acompanhamento, link no README e regra de manutenção no AGENTS. |

Verificação desta entrega: `cargo check --all-targets`, `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings` e `git diff --check` aprovados;
`cargo test` com 28 aprovados, zero falhas e zero ignorados.

## Como manter

Na mesma entrega de cada mudança de código, escopo, decisão ou pendência,
conferir e atualizar o estado, os próximos passos e a data/base acima. Acrescentar
um marco breve para mudanças relevantes, com commit existente ou indicação de
trabalho ainda sem commit. Registrar só verificações realmente executadas.
A atualização é manual durante o desenvolvimento, sem processo em segundo plano.
