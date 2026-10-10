# Parsing — Task 3

Lidos BOILERPLATE seções 6, 7 e 12, plano Task 3, AGENTS.md, test-driven-development, writing-good-tests e verification-before-completion.

## Evidência RED → GREEN

Criado skeleton público, integração do módulo solicitada ao coordenador. Testes usam arquivos temporários reais e expectativas literais; não há mocks. Primeira execução de `cargo test --test carregador`: **0 aprovados, 7 falharam**, porque carregamento/quantum ainda retornavam NaoImplementado ou não forneciam contexto. Após implementação: **7 aprovados, 0 falhas**.

Cobertura: oficiais (10 processos, 139 instruções, prioridades originais somando 58, quantum 3); ordenação de nomes antes de associação; filtro ASCII e apenas arquivos regulares; prioridade zero/u32 máximo; i64 mínimo/máximo negativos; LF/CRLF; nome vazio; ausência de instruções; instrução desconhecida/vazia; espaços em atribuições; SAIDA ausente, antecipada ou duplicada; limite 21; prioridades negativas, overflow e quantidade incorreta; quantum zero, negativo, overflow ou múltiplos valores; arquivos ausentes e UTF-8 inválido com linha; diretório/nome não UTF-8 sem panic.

Execução ampla `cargo test` após GREEN: carregador 7/7, modelos 4/4, politica 3/3; registro_estatisticas estava em RED concorrente (5 falhas: formato_dos_eventos_obedece_contrato_utf8_e_singular, cria_diretorio_e_publica_evento_antes_de_finalizar, criacao_propaga_erro_de_io, falha_de_flush_e_propagada, resumo_com_denominador_zero_nao_publica_resultados). Coordenador informado; não representa verificação final da integração.

## Decisões

BCPs seguem a ordem lexicográfica dos arquivos com PID sequencial a partir de 1; não se ordena por prioridade. FileType::is_file seleciona arquivos regulares, ignorando diretórios e links simbólicos. Nomes ASCII selecionados são UTF-8 seguros; diretório pode conter bytes não UTF-8. Registro aceita sinal opcional em decimal i64; configurações exigem dígitos ASCII sem espaços ou sinal. Um terminador LF/CRLF final é aceito; linha vazia adicional é rejeitada. Nome original é preservado, mas nome composto só por espaços é rejeitado.

Erros de leitura preservam variante Io e causa contextualizada por caminho, com número de linha para falhas durante leitura; erros de validação usam Entrada com caminho:linha. Nenhum programa oficial foi modificado. Formatação restrita aos dois arquivos Rust de responsabilidade do parsing. Nenhuma pendência de implementação neste escopo; revisão e verificação final cabem ao coordenador.
