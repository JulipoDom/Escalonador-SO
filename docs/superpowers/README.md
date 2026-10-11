# Superpowers no contexto atual

O [contexto vigente](plans/2026-10-10-contexto-atual.md) organiza o método de trabalho e as próximas etapas propostas. O [plano da primeira etapa](plans/2026-10-10-etapa1-interpretador-quantum.md) registra o recorte autorizado de interpretador e quantum, implementado com TDD e revisão independente. A primeira etapa foi integrada à `main` no commit `7b92a6a`. O [plano da segunda etapa](plans/2026-10-10-etapa2-filas-politica.md) registra filas, política de créditos e operações de espera de E/S, implementadas no commit `8d4d172`, integrado e publicado na `main`. O [plano da terceira etapa](plans/2026-10-11-etapa3-transicoes-es.md) registra o passo integrado de despacho, transições e E/S, registrado no commit `57099c4` e integrado localmente à `main` por fast-forward autorizado de `7ced42f`, sem push; suíte pós-merge com 77 aprovados, zero falhas e zero ignorados. A revisão independente do núcleo foi aprovada sem achados; revisões documental e final aprovadas sem achados. O laço completo é o próximo passo proposto; estatísticas, logs reais e integração de `main` continuam pendentes e exigem nova autorização.

O [enunciado de 2026](../../ep1.pdf) está na raiz do projeto, versionado no commit `7336c37`. A execução integrada continua recusando a simulação pendente e não produz logs ou medições.

Consulte o [AGENTS.md](../../AGENTS.md) para as instruções atuais de desenvolvimento e o [HISTORICO.md](../../HISTORICO.md) para o estado, as decisões e as pendências. Essas são as fontes atuais; prevalecem sobre o acervo abaixo.

## Acervo histórico

O [plano do boilerplate](plans/2026-10-10-boilerplate.md) e os registros em `docs/fluxo/`, incluindo o [registro de progresso](../fluxo/progresso.md), são documentos históricos da base anterior à simplificação do commit `ffc8849`. Foram recuperados do commit `58835cb` e preservados sem alterações para consulta.

Menções nesses documentos a CLI configurável, `--help`, `--validar`, execução em lote, CSV e contagens não descrevem capacidades atuais do projeto.

O [índice de docs/fluxo](../fluxo/README.md) reforça essa distinção. Os arquivos históricos permanecem byte a byte como em `58835cb`; os novos índices e o plano vigente são documentação posterior.
