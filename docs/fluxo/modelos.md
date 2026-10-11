# Modelos e erros

Especialista responsável por `src/modelos.rs`, `src/erros.rs` e `tests/modelos.rs`.
Consultadas as seções 4 e 6 do documento fonte e Task 2 do plano.
Aplicadas as skills `test-driven-development`, sua referência `writing-good-tests.md`
e `verification-before-completion`.

## Decisões

Tipos e campos públicos seguem a seção 6. `Programa` possui um segmento `Box<[Instrucao]>`;
`Bcp::novo` recebe propriedade do programa, zera o contexto e copia prioridade para créditos.
BCPs e programas não derivam `Clone`, evitando oferecer clonagem incidental de contextos.
IDs e enums pequenos derivam os traits contratados. Nenhuma execução foi implementada.
`ErroEp::Io` mantém o erro original e o expõe em `source`; erros de domínio preservam
seu contexto no `Display` e não inventam causas.

## Evidências de TDD

Primeira tentativa teve import incorreto da biblioteca (`escalonador`); corrigido
para o nome Cargo `ep1_escalonador` antes de observar RED.
`cargo test --test modelos`: compilou e executou quatro testes, todos falhando
nos corpos provisórios pendentes de inicialização, conversão ou formatação.
Esses corpos foram então implementados e removidos os `todo!` provisórios.

Verificação isolada real: compilação dos dois módulos com `rustc --edition 2021`
em biblioteca temporária e execução do próprio `tests/modelos.rs`: **4 aprovados,
0 falhas**. A compilação isolada foi necessária porque `src/lib.rs` passou a declarar
`tabela_processos`, `filas` e `politica` antes de seus arquivos existirem.
`cargo test` nessa etapa retornou 101 com E0583 para esses três módulos;
não representa falha dos testes de modelos. Integração final pertence ao coordenador.

Os testes capturam inicialização incorreta (inclusive prioridade zero e máxima),
compartilhamento de contexto e perda de informação/causa nos erros. Não testam
frameworks nem texto-fonte. Nenhuma dependência externa, Git ou formatação global usada.

Após os arquivos de filas/tabela/política aparecerem, verificação Cargo final desta
entrega: `cargo test --test modelos` **4 aprovados, 0 falhas**, exit 0.
`cargo test` **6 aprovados, 0 falhas** (4 modelos + 2 política presentes naquele
instante), exit 0. A falha E0583 anterior foi transitória e está resolvida.
