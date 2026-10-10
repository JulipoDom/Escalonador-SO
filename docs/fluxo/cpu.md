# CPU simulada e E/S — Task 5

Responsável: especialista CPU/E/S. Escopo exclusivo: `src/interpretador.rs`,
`src/simulador.rs`, `tests/simulador.rs` e este registro. Consultados AGENTS,
seções 4, 6 e 8 do boilerplate e Task 5 do plano operacional; lidas as skills
TDD (incluindo writing-good-tests) e verification-before-completion.

As três APIs exatas estão disponíveis para a CLI. Todos os corpos continuam
retornando `ErroEp::NaoImplementado` com descrição específica. Não foi
executada nenhuma instrução, implementado controle E/S ou produzido resultado
experimental. Não foram necessárias interfaces adicionais.

Os contratos `///` registram PC avançando uma vez, E/S e SAIDA contando como
instruções, quantum e parada antecipada, tabela proprietária durante execução,
crédito por despacho, duas passagens posteriores para E/S, atualização apenas
dos bloqueados anteriores, tempo ocioso lógico e ordem das transições.

Testes reais verificam erro reservado e preservação de todos os campos do
BCP para as cinco instruções e quanta 0, 1 e 3. O teste integrado do escritor
verifica log vazio após simulação recusada e flush; usa diretório temporário
removido também em caso de falha de asserção. Não há mocks nem testes ignorados.
Esses testes detectam execução prematura ou publicação de sucesso fictício.

Evidências executadas:

- Antes dos corpos, `cargo test --test simulador` falhou por imports ausentes;
  isso foi uma falha de compilação por integração ainda indisponível, não uma
  fase RED comportamental. Não é alegado ciclo TDD comportamental completo.
- Após integração, `cargo test --test simulador`: 3 aprovados, 0 falhas,
  0 ignorados.
- `cargo test` na versão disponível antes de integrar CLI: 23 aprovados,
  0 falhas e 0 ignorados (incluindo os três testes CPU). A verificação global
  final permanece sob responsabilidade do coordenador após cessarem edições.
- Formatação limitada aos três arquivos Rust atribuídos, com Rust 2021.

Pendência intencional: implementar futuramente CPU, despacho, transições,
controle E/S e simulação somente após autorização para completar o núcleo.
