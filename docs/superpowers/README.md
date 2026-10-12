# Superpowers no contexto atual

O [contexto vigente](plans/2026-10-10-contexto-atual.md) registra o estado e as próximas ações. As seis etapas estão implementadas; os experimentos e o relatório existem desde `cb6d5f6`, e as correções estão publicadas em `884d7f5`. O pedido vigente autoriza integrar a branch `codex/correcoes-relatorio-ep1` à `main` após validação. A nova rodada aprovou testes, reexecuções e revisões independentes do núcleo e do relatório. Evidências e limites estão no [HISTORICO](../../HISTORICO.md): o PDF existente foi inspecionado; sua geração não foi reexecutada por ausência de ReportLab, e a compilação TEX permanece não verificada.

## Planos das etapas implementadas

| Etapa | Plano | Commit |
| --- | --- | --- |
| Interpretador e quantum | [Primeira etapa](plans/2026-10-10-etapa1-interpretador-quantum.md) | `7b92a6a` |
| Filas, política de créditos e espera de E/S | [Segunda etapa](plans/2026-10-10-etapa2-filas-politica.md) | `8d4d172` |
| Despacho, transições e E/S em um passo | [Terceira etapa](plans/2026-10-11-etapa3-transicoes-es.md) | `57099c4` |
| Laço completo do núcleo | [Quarta etapa](plans/2026-10-11-etapa4-laco-simulacao.md) | `75941d0` |
| Estatísticas e logs reais em `simular` | [Quinta etapa](plans/2026-10-11-etapa5-logs-estatisticas.md) | `e61ce88` |
| Execução principal integrada | [Sexta etapa](plans/2026-10-11-etapa6-execucao-integrada.md) | `bf093f1` |

O [enunciado de 2026](../../ep1.pdf), incluído em `7336c37`, prevalece sobre documentos anteriores. O escalonador resolve entradas e log pelo diretório corrente e continua em Rust com somente `std`. O gerador Python resolve arquivos pela raiz do próprio script, valida os logs existentes e usa ReportLab para produzir PDF; também gera TEX do mesmo conteúdo. A geração deste relatório exige a identidade semântica da carga oficial original e os 15 quantums previstos; cargas divergentes são recusadas antes das saídas, embora as funções de validação/coleta permaneçam genéricas. Instruções de geração, dependência e testes estão no [README](../../README.md).

Consulte [AGENTS](../../AGENTS.md) para responsabilidades, delegação, TDD, revisão e verificação. O HISTORICO preserva as evidências das entregas anteriores, sem tratá-las como resultados da correção atual. Os próximos passos são conferência do material e preparação do ZIP; prazo e situação de submissão permanecem a confirmar.

## Acervo histórico

O [plano do boilerplate](plans/2026-10-10-boilerplate.md) e os registros em `docs/fluxo/`, incluindo o [registro de progresso](../fluxo/progresso.md), são documentos históricos da base anterior à simplificação do commit `ffc8849`. Foram recuperados do commit `58835cb` e preservados sem alterações para consulta.

Menções nesses documentos a CLI configurável, `--help`, `--validar`, execução em lote, CSV e contagens não descrevem capacidades atuais do projeto.

O [índice de docs/fluxo](../fluxo/README.md) reforça essa distinção. Os arquivos históricos permanecem byte a byte como em `58835cb`; os novos índices e o contexto vigente são documentação posterior.
