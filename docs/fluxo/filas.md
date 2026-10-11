# Evidências — tabela e filas

Escopo exclusivo: `src/tabela_processos.rs`, `src/filas.rs`, `src/politica.rs`, `tests/politica.rs` e este registro. Lidas seções 4, 6 e 8 do documento de contratos, Task 4 do plano, AGENTS e skills test-driven-development (incluindo writing-good-tests) e verification-before-completion.

A tabela é proprietária de `HashMap<ProcessId, Bcp>`. Inserção usa Entry para rejeitar duplicação sem substituir o contexto original. Consulta, empréstimo mutável curto, remoção, iteração, tamanho e consulta de vazio são operações estruturais implementadas. A iteração não promete ordem. `is_empty` e `vazia` são equivalentes.

`FilaProntos` contém `Vec<ProcessId>`; `FilaBloqueados` contém `VecDeque<ProcessId>`, ambos privados e observáveis por empréstimo imutável. Construtores criam filas vazias. Ordenação, seleção, reinserções distintas, bloqueio, avanço de esperas, desconto e redistribuição estão documentados e retornam erros NaoImplementado específicos sem mutação. Nenhuma política de escalonamento foi implementada.

## Verificação executada

- RED: `cargo test --test politica` compilou e falhou nos dois testes da tabela, na inserção, com NaoImplementado("armazenamento estrutural").
- GREEN: mesmo comando após implementação estrutural aprovou os dois testes.
- Contratos pendentes: terceiro teste verifica erro de cada stub e ausência de alterações na tabela e nas filas; não afirma implementação das regras reservadas.
- `cargo test --test politica`: 3 aprovados, 0 falhas, 0 ignorados.
- `cargo test`: 7 aprovados (4 modelos e 3 política), 0 falhas e 0 ignorados no conjunto disponível durante esta tarefa; módulos de outras ondas ainda serão integrados pelo coordenador.
- Formatação limitada aos quatro arquivos Rust atribuídos, sem rustfmt global.

Pendências intencionais: todas as regras da política de créditos, organização de filas, controle de espera e escalonamento descritas na seção 8. Revisão e verificações finais após integração cabem ao coordenador.
