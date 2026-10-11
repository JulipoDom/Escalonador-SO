# EP1 — Escalonador

Base do EP1 de Sistemas Operacionais de 2026 em Rust stable, edição 2021, com Cargo e somente a biblioteca padrão. `programas/` preserva os 12 arquivos oficiais: dez processos, `prioridades.txt` e `quantum.txt`.

Estado, decisões e próximos passos: [Histórico do projeto](HISTORICO.md).

Fluxo de desenvolvimento com Superpowers: [instruções](AGENTS.md), [contexto vigente](docs/superpowers/plans/2026-10-10-contexto-atual.md), [plano da primeira etapa](docs/superpowers/plans/2026-10-10-etapa1-interpretador-quantum.md) e [índice e acervo](docs/superpowers/README.md).

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

Filas e política são o primeiro próximo passo proposto. Controle de E/S, laço de simulação, estatísticas e integração da execução permanecem pendentes com `ErroEp::NaoImplementado` específico. A execução valida a entrada e encerra com código 1, sem criar logs. Os testes dos contratos pendentes verificam essa recusa; a execução de quantum isolada não comprova a simulação completa.

Convenções para as etapas restantes conforme o [enunciado de 2026](ep1.pdf):

- A tabela possui os BCPs; filas guardam somente `ProcessId`. Toda instrução, inclusive E/S e SAIDA, consome uma unidade e avança o PC uma vez; E/S retoma na seguinte.
- Créditos começam na prioridade; escolher o maior crédito pronto e descontar um por despacho, sem ficar negativo. Redistribuir as prioridades somente quando todos os ativos, inclusive bloqueados, têm zero. Terminados não participam.
- Empate inicial segue a ordem dos arquivos; ao fim do quantum, reinserir antes dos créditos iguais; no retorno de E/S, depois dos iguais, em ordem de retorno. Se prontos têm zero e algum bloqueado tem crédito, usar FIFO e reinserir ao final. Prioridades todas zero também usam FIFO.
- E/S bloqueia por dois surtos posteriores de outro processo, mesmo que sejam do mesmo PID entre si ou terminem antes do quantum. O surto que bloqueou não reduz a espera. Atualizar os bloqueados anteriores e liberar elegíveis em FIFO ao final de cada surto. Sem prontos, avançar somente a espera lógica, sem instruções, créditos ou despachos.

Os logs futuros serão UTF-8 em `logXX.txt`, com eventos de carga, execução, E/S, interrupção e término, seguidos das médias e do quantum. Para P processos iniciais, D despachos concluídos e I instruções executadas: média de trocas = D/P; média de instruções = I/D, com divisão em ponto flutuante e denominadores válidos. Cada surto conta em D, inclusive término e despachos consecutivos do mesmo PID; E/S e SAIDA contam em I; ociosidade não conta.

Os experimentos e o relatório PDF exigidos pelo EP continuam obrigatórios como trabalho futuro: após implementar o núcleo, executar manualmente pelo menos dez valores de quantum distribuídos uniformemente em um intervalo útil, reunir logs e médias, elaborar tabelas e gráficos e justificar a escolha de quantum com os dados. Esta base não produz resultados experimentais nem o relatório.
