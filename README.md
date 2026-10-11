# EP1 — Escalonador

Base do EP1 de Sistemas Operacionais de 2026 em Rust stable, edição 2021, com Cargo e somente a biblioteca padrão. `programas/` preserva os 12 arquivos oficiais: dez processos, `prioridades.txt` e `quantum.txt`.

Estado, decisões e próximos passos: [Histórico do projeto](HISTORICO.md).

Fluxo de desenvolvimento com Superpowers: [instruções](AGENTS.md), [plano vigente](docs/superpowers/plans/2026-10-10-contexto-atual.md) e [registros históricos](docs/superpowers/README.md).

Execute na raiz deste projeto (`ep1-escalonador/`), onde está `programas/`:

```bash
cargo run --bin Escalonador
cargo check --all-targets
cargo test
cargo fmt --check
```

Opcionalmente, compile com `cargo build --release --bin Escalonador` e execute `./target/release/Escalonador` no mesmo diretório. A entrada vem exclusivamente de `programas/`, incluindo prioridades e quantum.

Modelos, carregamento e validação das entradas, tabela de processos e escrita dos eventos de log estão implementados. Interpretador, política de escalonamento, controle de E/S, simulação e cálculo das estatísticas permanecem pendentes. A execução valida a entrada e retorna `ErroEp::NaoImplementado`, com código diferente de zero e sem criar logs. Os testes de contratos pendentes verificam essa recusa, sem comprovar o funcionamento do núcleo.

Convenções para implementar o núcleo conforme `../ep1.pdf` de 2026:

- A tabela possui os BCPs; filas guardam somente `ProcessId`. Toda instrução, inclusive E/S e SAIDA, consome uma unidade e avança o PC uma vez; E/S retoma na seguinte.
- Créditos começam na prioridade; escolher o maior crédito pronto e descontar um por despacho, sem ficar negativo. Redistribuir as prioridades somente quando todos os ativos, inclusive bloqueados, têm zero. Terminados não participam.
- Empate inicial segue a ordem dos arquivos; ao fim do quantum, reinserir antes dos créditos iguais; no retorno de E/S, depois dos iguais, em ordem de retorno. Se prontos têm zero e algum bloqueado tem crédito, usar FIFO e reinserir ao final. Prioridades todas zero também usam FIFO.
- E/S bloqueia por dois surtos posteriores de outro processo, mesmo que sejam do mesmo PID entre si ou terminem antes do quantum. O surto que bloqueou não reduz a espera. Atualizar os bloqueados anteriores e liberar elegíveis em FIFO ao final de cada surto. Sem prontos, avançar somente a espera lógica, sem instruções, créditos ou despachos.

Os logs futuros serão UTF-8 em `logXX.txt`, com eventos de carga, execução, E/S, interrupção e término, seguidos das médias e do quantum. Para P processos iniciais, D despachos concluídos e I instruções executadas: média de trocas = D/P; média de instruções = I/D, com divisão em ponto flutuante e denominadores válidos. Cada surto conta em D, inclusive término e despachos consecutivos do mesmo PID; E/S e SAIDA contam em I; ociosidade não conta.

Os experimentos e o relatório PDF exigidos pelo EP continuam obrigatórios como trabalho futuro: após implementar o núcleo, executar manualmente pelo menos dez valores de quantum distribuídos uniformemente em um intervalo útil, reunir logs e médias, elaborar tabelas e gráficos e justificar a escolha de quantum com os dados. Esta base não produz resultados experimentais nem o relatório.
