# Roteiro do relatório futuro

Este arquivo organiza a redação; não é um relatório experimental concluído. Não há tabela de medições, gráficos ou recomendação de quantum antes de implementar e executar o núcleo.

1. Identificação: nomes e números USP de todos os integrantes, disciplina e EP1 de 2026.
2. Objetivo e implementação: Rust stable/edição 2021, CPU fictícia, BCP, tabela proprietária, filas de IDs, carregador, interpretador e logs. Distinguir o que foi implementado e o que foi testado.
3. Convenções: desempates, FIFO no caso adverso, redistribuição incluindo bloqueados, contagem de E/S/SAIDA, espera de duas passagens posteriores e ordem de transições. Referenciar `docs/decisoes.md` e explicar ambiguidades resolvidas.
4. Ambiente e reprodução: versões reais de rustc/Cargo, sistema e comandos, origem das entradas preservadas e configuração de cada execução.
5. Verificação: casos pequenos calculados manualmente, resultados reais dos testes e limites. A leitura estática aponta 10 processos e 139 instruções oficiais; isso não comprova a ordem de escalonamento.
6. Experimentos: executar independentemente quanta 1, 3, 5, 7, 9, 11, 13, 15, 17, 19 e 21, recriando BCPs e contadores, com entradas idênticas. Guardar um log por quantum e CSV reproduzível.
7. Resultados: preencher tabela somente com dados obtidos. Incluir quantum, processos, instruções, despachos, média de trocas D/P, média de instruções I/D e aproveitamento (I/D)/quantum. Registrar denominadores válidos e ausência de contagem de ociosidade.
8. Gráficos: duas médias em função do quantum, e discussão da fração média da fatia usada. Salvar figuras em `figuras/` com eixos, unidades e fonte dos dados. Aproveitamento não é utilização real de CPU.
9. Discussão: relacionar preempção, E/S e tamanho das rajadas aos dados. Recomendar um quantum com base nos compromissos observados, sem pressupor que o maior é melhor.
10. Entrega: revisar identificação, referências, código e logs; gerar `relatorio.pdf` apenas após completar a análise. Preparar um ZIP com número USP de um integrante conforme o enunciado atual.

Comando reservado para a coleta futura, executado da raiz `ep1-escalonador/`:

```bash
cargo run --bin Escalonador -- --quanta 1 3 5 7 9 11 13 15 17 19 21 --saida resultados
```

Na base atual, esse comando retorna funcionalidade não implementada sem gerar medições. O prazo impresso de 04/10/2026 é um dado do PDF, não uma confirmação de prorrogação.
