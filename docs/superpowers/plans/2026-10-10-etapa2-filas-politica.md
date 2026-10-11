# Etapa 2 — filas e política de créditos

Goal: implementar as operações das filas e a política de créditos já aprovadas na conversa, sem integrar o simulador completo.

Architecture: a tabela continua dona dos BCPs; filas contêm apenas ProcessId. Operações inválidas devem retornar EstadoInvalido antes de modificar estado. Rust 2021, somente std.

Spec: ep1.pdf, README e contratos públicos de filas.rs/politica.rs. Base: 7b92a6a. Branch: codex/etapa2-filas-politica. Sem stage, commit ou merge nesta entrega; aguardar revisão humana.

Global constraints: preservar as 12 entradas oficiais; não alterar CPU, main, estatísticas ou produzir resultados de simulação. Atualizar contextos e histórico com verificações reais.

Autorização posterior (2026-10-11): o usuário solicitou nova revisão e autorizou stage e commit se tudo estiver correto. Essa autorização substitui a restrição anterior desta entrega; não autoriza merge ou etapas futuras. O boilerplate local deve permanecer fora do rastreio, com exclusão no Git.

Review focus: desempates, FIFO de créditos zero, espera de duas unidades posteriores ao bloqueio, validação antes de mutação e distinção entre operações implementadas e integração pendente.

## Task 1: núcleo e testes

Editor único: src/filas.rs, src/politica.rs, tests/politica.rs e, se útil, tests/filas.rs. Aplicar TDD em grupos de comportamento, registrar RED e GREEN realmente observados.

- Carregar todos os BCPs Pronto em ordenar_inicial; ordenar por créditos decrescentes, arquivo crescente, PID como desempate determinístico de arquivos iguais. Selecionar maior crédito preservando FIFO nos empates. Reordenar após redistribuição estavelmente.
- Reinserção de quantum precede créditos iguais; retorno de E/S sucede iguais. Reinserir ao final quando todos os prontos, incluindo candidato, têm zero e há bloqueado com crédito positivo; também usar FIFO quando todas as prioridades ativas são zero.
- Validar existência, estado Pronto e duplicidade antes de reinserir; validar a fila inteira antes de operações que usam seus BCPs. Seleção não muda o estado do BCP; esse papel pertence ao futuro laço.
- Descontar um crédito com saturação; PID ausente é erro. Redistribuir apenas se todos os ativos têm zero e alguma prioridade é positiva; tabela vazia e prioridades todas zero retornam false. Incluir bloqueados; terminados removidos da tabela não participam.
- Bloquear um processo Executando, sem duplicidade: estado Bloqueado, espera 2, inserção FIFO. avancar_esperas valida snapshot sem duplicidades e pertencente à fila, decrementa somente seus IDs e libera elegíveis na ordem da fila, definindo Pronto e espera 0. Novos bloqueados não perdem espera no próprio surto. Chamador aplica um tick lógico em ociosidade com snapshot da fila, sem CPU/desconto.
- Testar ordens literais, casos zero, bloqueados impedindo redistribuição, erros sem mutação, duas esperas posteriores e liberação FIFO mesmo com snapshot invertido. Preservar testes da tabela.
- Rodar testes focados e formatar apenas arquivos sob sua responsabilidade. Relatar arquivos, testes, decisões e limitações ao coordenador.

## Task 2: documentação

Editor único: README.md, HISTORICO.md, docs/superpowers/README.md e 2026-10-10-contexto-atual.md. Após revisão do núcleo, atualizar estado e próximos passos. Registrar commit existente 7b92a6a como primeira etapa integrada à main; segunda etapa sem commit. Não marcar simulador, logs ou estatísticas como implementados. Após verificações globais, registrar somente resultados realmente fornecidos pelo coordenador.

## Task 3: revisão independente e verificação

Revisor somente leitura verifica requisitos, implementação e testes. Correções retornam ao autor. Coordenador executa cargo check --all-targets, cargo test, cargo fmt --check, cargo clippy --all-targets -- -D warnings, execução explícita ainda pendente, comparação byte a byte das entradas e ausência de resultados fictícios. Documentação registra resultados; revisão final confirma escopo e coerência. Entrega sem commit.
