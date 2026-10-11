# EP1 — Escalonador

Base do EP1 de Sistemas Operacionais de 2026 em Rust stable, edição 2021, com Cargo e somente a biblioteca padrão. `programas/` preserva os 12 arquivos oficiais: dez processos, `prioridades.txt` e `quantum.txt`.

Estado, decisões e próximos passos: [Histórico do projeto](HISTORICO.md).

Fluxo de desenvolvimento com Superpowers: [instruções](AGENTS.md), [contexto vigente](docs/superpowers/plans/2026-10-10-contexto-atual.md), [plano da primeira etapa](docs/superpowers/plans/2026-10-10-etapa1-interpretador-quantum.md), [plano da segunda etapa](docs/superpowers/plans/2026-10-10-etapa2-filas-politica.md), [plano da terceira etapa](docs/superpowers/plans/2026-10-11-etapa3-transicoes-es.md), [plano da quarta etapa](docs/superpowers/plans/2026-10-11-etapa4-laco-simulacao.md), [plano da quinta etapa](docs/superpowers/plans/2026-10-11-etapa5-logs-estatisticas.md) e [índice e acervo](docs/superpowers/README.md).

Execute na raiz deste projeto, onde estão `programas/` e o [enunciado de 2026](ep1.pdf):

```bash
cargo run --bin Escalonador
cargo check --all-targets
cargo test
cargo fmt --check
```

Opcionalmente, compile com `cargo build --release --bin Escalonador` e execute `./target/release/Escalonador` no mesmo diretório. A entrada vem exclusivamente de `programas/`, incluindo prioridades e quantum.

Modelos, carregamento e validação das entradas, tabela de processos e escrita dos eventos fornecidos estão implementados. A primeira etapa implementa `executar_instrucao` e `executar_quantum` sobre um BCP: atribuições, COM, E/S e SAIDA avançam o PC uma vez; E/S e SAIDA interrompem e contam na execução, inclusive na fronteira do quantum. A retomada começa na instrução seguinte, preservando registradores e os demais campos do BCP.

PC fora do texto retorna `EstadoInvalido` sem mutação da instrução inválida. Quantum zero retorna `Entrada` antes de consultar o PC e sem modificar o BCP; um erro após instruções válidas preserva o progresso já executado. Estado, créditos e espera não são alterados pelo interpretador ou quantum.

A segunda etapa implementa carga e seleção de prontos, reordenação estável por créditos, reinserções de quantum e retorno de E/S, desconto saturado e redistribuição de créditos. Filas guardam somente IDs e validam existência, estado e duplicidade antes de mutar. A seleção não altera o estado do BCP; o passo integrado define Executando e aplica as transições posteriores.

As operações de E/S também estão implementadas: bloquear exige Executando, define Bloqueado e espera 2; avançar esperas desconta somente os IDs de um snapshot anterior validado e libera elegíveis em FIFO, definindo Pronto e espera 0. Novos bloqueados não perdem espera no próprio surto. A terceira etapa integra essas operações em `executar_passo`: valida tabela e filas antes do despacho, redistribui quando necessário, desconta um crédito, executa um quantum e aplica a parada real. Quantum recoloca Pronto; E/S bloqueia; SAIDA remove e devolve o BCP proprietário com seu contexto final. Ao fim do surto, somente os bloqueados anteriores avançam e os liberados retornam em FIFO após empates. Sem prontos, um passo avança uma unidade lógica de espera sem CPU ou alteração de créditos.

`ResultadoPasso` distingue `Surto` (resultado real e BCP terminado opcional), `Ocioso` (IDs liberados) e `Concluido` (tabela e filas vazias). Quantum zero e inconsistências estruturais falham antes de mutar. Erro da CPU após o despacho preserva o BCP Executando fora das filas, o crédito descontado e o progresso válido, sem avançar esperas. Não há recuperação automática: uma nova chamada rejeita esse estado inconsistente.

`executar_laco` completa a quarta etapa do núcleo: prepara a tabela proprietária e as filas, consome `executar_passo` até a conclusão e devolve os registradores finais reais em `HashMap<ProcessId, (i64, i64)>`. Com quantum positivo e entrada vazia, devolve mapa vazio; quantum zero é recusado antes da preparação. Surtos e ociosidade prosseguem sem limite artificial, e erros são propagados. Quinze testes novos cobrem casos calculados à mão e uma execução das entradas oficiais, incluindo registradores prévios, E/S consecutivas, prioridades zero e erros após progresso.

O código e os testes da quarta etapa foram registrados em `75941d0` (`feat: implement complete core simulation loop`). Os contextos foram registrados em `05bc657`; ambos os commits foram integrados localmente à `main` por fast-forward autorizado de `94a9482`. A suíte pós-merge teve 92 aprovados, zero falhas e zero ignorados. Após autorização do usuário, a publicação em `origin/main` foi concluída até `434f43b`.

A quinta etapa implementa `simular` com eventos, contagens, resumo e registradores finais reais. O trabalho está na branch `codex/etapa5-logs-estatisticas`, criada de `eddae3c`, com os 11 arquivos da etapa colocados em stage pelo usuário; commit e merge local na `main` autorizados e ainda pendentes, após última revisão pré-commit aprovada em conformidade e qualidade sem achados e nova verificação do coordenador aprovada (check, suíte 111/0/0, fmt e Clippy), sem pedido de push. A suíte do autor do núcleo teve 111 aprovados, zero falhas e zero ignorados; as revisões independentes da saída e do núcleo foram aprovadas em conformidade e qualidade, sem achados. O coordenador aprovou check de todos os alvos, suíte com 111/0/0, fmt em modo check, Clippy com warnings negados e diff check. Entradas e acervo conferidos contra `eddae3c`; revisão documental e final do conjunto aprovada em conformidade e qualidade, sem achados críticos, importantes ou menores.

`simular` valida quantum positivo, entrada não vazia, PIDs únicos e estado/espera inicial antes de escrever eventos. A entrada vazia é recusada porque as médias teriam denominadores indefinidos; o contrato de mapa vazio de `executar_laco` permanece preservado. Erros da CPU ou da escrita são propagados sem resultado de sucesso; eventos reais já escritos podem permanecer, e falhas antes da conclusão não produzem resumo de sucesso. A API escreve o resumo após a conclusão e propaga a falha de publicação final antes de devolver sucesso.

A execução principal ainda não chama `simular`: valida a entrada e encerra com código 1, stdout vazio e `Não implementado: integração da simulação com estatísticas e logs`, sem criar logs. Essa mensagem antiga foi preservada porque `main` e seus testes não foram alterados; ela se refere à integração principal pendente, embora a API já implemente estatísticas e registro. Os testes de recusa da execução não comprovam a integração do binário; os testes de `simular` comprovam a saída real da API.

Convenções vigentes conforme o [enunciado de 2026](ep1.pdf):

- A tabela possui os BCPs; filas guardam somente `ProcessId`. Toda instrução, inclusive E/S e SAIDA, consome uma unidade e avança o PC uma vez; E/S retoma na seguinte.
- Créditos começam na prioridade; escolher o maior crédito pronto e descontar um por despacho, sem ficar negativo. Redistribuir as prioridades somente quando todos os ativos, inclusive bloqueados, têm zero. Terminados não participam.
- Empate inicial segue a ordem dos arquivos; ao fim do quantum, reinserir antes dos créditos iguais; no retorno de E/S, depois dos iguais, em ordem de retorno. Se prontos têm zero e algum bloqueado tem crédito, usar FIFO e reinserir ao final. Prioridades todas zero também usam FIFO.
- E/S bloqueia por dois surtos posteriores de outro processo, mesmo que sejam do mesmo PID entre si ou terminem antes do quantum. O surto que bloqueou não reduz a espera. Atualizar os bloqueados anteriores e liberar elegíveis em FIFO ao final de cada surto. Sem prontos, avançar somente a espera lógica, sem instruções, créditos ou despachos.

`simular` usa o escritor UTF-8 para registrar Carregando na ordem inicial de prontos e Executando antes de cada surto. E/S escreve EntradaSaida antes de Interrompendo; a interrupção informa somente as instruções do último surto. SAIDA escreve Terminado com X/Y reais, sem interrupção adicional. Após a conclusão, escreve as médias e o quantum. A criação de `logXX.txt` pela execução principal continua pendente. Para P processos iniciais, D despachos concluídos e I instruções executadas: média de trocas = D/P; média de instruções = I/D, com divisão em ponto flutuante e denominadores válidos. Cada surto conta em D, inclusive término e despachos consecutivos do mesmo PID; E/S e SAIDA contam em I; ociosidade não conta.

Os experimentos e o relatório PDF exigidos pelo EP continuam obrigatórios como trabalho futuro: após integrar a execução principal, executar manualmente pelo menos dez valores de quantum distribuídos uniformemente em um intervalo útil, reunir logs e médias, elaborar tabelas e gráficos e justificar a escolha de quantum com os dados. Esta base não produz resultados experimentais nem o relatório.
