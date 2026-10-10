# Cenários controlados

Os arquivos oficiais em `programas/` são preservados. Casos temporários são criados pelos testes que precisam deles e devem ser removidos ao final; este diretório não contém resultados de simulação.

Para entrada, verificar conjuntos menores, LF/CRLF, nome vazio, linha interna vazia, instrução desconhecida, SAIDA fora do final, limite de instruções, prioridades ausentes ou excedentes, prioridade negativa e overflow de números. Mensagens devem identificar arquivo e linha quando aplicável.

Os cenários abaixo são critérios futuros calculados manualmente; não são resultados executados pelo núcleo desta base:

| Cenário | Critério futuro |
| --- | --- |
| `X=8`, `SAIDA`, quantum 1 | 2 despachos, 2 instruções, X=8; médias 2 e 1 |
| Mesmo programa, quantum 2 | 1 despacho, 2 instruções; médias 1 e 2 |
| `E/S`, `SAIDA` | PC=1 ao bloquear; E/S conta e não é relida |
| Bloqueado e dois surtos posteriores | Espera 2, 1, 0, sem desconto no bloqueio inicial |
| Todos bloqueados | Avanço lógico sem instruções ou despachos fictícios |
| Prontos zero, bloqueado com crédito | FIFO sem redistribuição ou crédito negativo |
| Todos ativos zero | Redistribuição inclui bloqueados |
| Empates e retorno de E/S | Ordem definida em `docs/decisoes.md` |
| Dois BCPs com atribuições distintas | Contextos independentes entre quanta |

Até implementar o núcleo, testes de seus contratos verificam `NaoImplementado` e ausência de mutações indevidas. Não substituir o erro por asserções vazias nem declarar esses testes como evidência das regras de escalonamento. Casos ainda não executáveis podem ser explicitamente ignorados com justificativa, sem entrar na contagem de aprovados.
