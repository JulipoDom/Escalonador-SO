# Decisões e convenções

Fonte normativa: `../../ep1.pdf` (3/9/2026), complementada por `../../BOILERPLATE_EP1_ESCALONADOR.md`. O documento BOILERPLATE fornecido registra uma análise estática da referência Java de 2017; essa consulta histórica não foi repetida nesta execução e não fornece resultados experimentais nem substitui as regras atuais. Rust é permitido pela livre escolha de linguagem no enunciado de 2026. Edição 2021 e biblioteca padrão são decisões do projeto.

## Entrada e ownership

Selecionar nomes ASCII `[0-9]{2}.txt` em ordem alfabética, associando prioridades antes de qualquer ordenação por créditos. Configurações não são processos. Aceitar LF e CRLF sem alterar arquivos oficiais. Exigir nome, de 1 a 21 instruções, `SAIDA` obrigatória e somente no final, atribuições sem espaços junto a `=` e nenhum comando desconhecido ou linha vazia interna.

Prioridades usam `u32`: zero é aceito e negativo é rejeitado. Registradores usam `i64`; quantum usa `usize` positivo. Os limites desses tipos são escolhas de representação; números fora deles causam erro. Quantidade de processos vem da entrada.

IDs sequenciais e estáveis seguem os arquivos. A tabela possui todos os BCPs ativos, inclusive o executando; filas possuem apenas IDs. O texto tem tamanho fixo em `Box<[Instrucao]>`. Remover um terminado somente depois de salvar nome e registradores. Ordenar IDs para exibir resultados de `HashMap`.

## Núcleo reservado — seção 8

Estas são regras para implementação posterior, não funcionalidades já executadas:

- Toda instrução, inclusive E/S e SAIDA, consome uma unidade e avança PC exatamente uma vez. E/S retoma na instrução seguinte. Registradores e PC permanecem no BCP.
- Créditos começam iguais à prioridade. Escolher o maior crédito pronto e descontar um por despacho, com `saturating_sub(1)`, independentemente do quantum.
- Redistribuir apenas se todos os ativos, inclusive bloqueados, têm zero; restaurar prioridades e ordenar prontos de forma estável. Terminados não participam.
- Empate inicial segue o arquivo. No fim do quantum, reinserir antes de créditos iguais. No retorno de E/S, inserir depois de iguais, preservando ordem de retorno; essa última regra resolve uma ambiguidade do PDF atual.
- Se prontos têm zero e algum bloqueado tem crédito, usar FIFO e reinserir no final; essa regra prevalece sobre a reinserção anterior. Se todas as prioridades são zero, manter FIFO sem laço de redistribuição.
- E/S define espera dois e entra no fim dos bloqueados. Decrementar a espera apenas em dois surtos posteriores de outro processo, ainda que do mesmo PID entre si ou que terminem antes do quantum. Nunca descontar no surto que iniciou o bloqueio.
- Sem prontos, avançar espera lógica dos bloqueados até liberar alguém; não executar instruções nem gastar créditos ou contar despachos nesse avanço.

Ao final do surto: registrar a parada; reinserir, bloquear ou remover o atual; atualizar somente os bloqueados anteriores; liberar elegíveis em FIFO; verificar redistribuição antes do próximo despacho. Cada ativo ocupa exatamente uma posição lógica, com no máximo um executando; filas não duplicam IDs, prioridade original é preservada, créditos e espera não ficam negativos e a tabela vazia encerra a execução.

## Logs e médias — seção 9

Escrever UTF-8 em `log{quantum:02}.txt` com escritor dedicado e flush explícito, propagando erros. O formatador implementado recebe eventos e resumos; não calcula resultados de uma CPU ainda ausente.

```text
Carregando TESTE-1
Executando TESTE-1
E/S iniciada em TESTE-1
Interrompendo TESTE-1 após 5 instruções
TESTE-1 terminado. X=2. Y=10
MEDIA DE TROCAS: <valor calculado>
MEDIA DE INSTRUCOES: <valor calculado>
QUANTUM: <quantum usado>
```

As linhas ilustram formatos isolados, não um trace produzido. `Carregando` segue a fila inicial. Interrupção ocorre por quantum ou E/S e usa a contagem apenas do último surto, com singular para uma instrução. SAIDA gera término sem interrupção textual redundante.

Para P processos iniciais, D despachos concluídos e I instruções executadas: média de trocas = D/P e média de instruções = I/D, convertendo para ponto flutuante antes da divisão e validando denominadores. Cada surto conta em D, inclusive término e despacho consecutivo do mesmo PID. E/S e SAIDA contam em I; ociosidade não conta em D ou I. Coleta e cálculo permanecem pendentes.

## CLI — seção 11

Caminhos e modos estão descritos no README. `args_os` preserva caminhos não UTF-8. Ajuda e validação independem do núcleo. Simulação individual e lote devem falhar sem publicar resultados enquanto o núcleo estiver reservado. Não gravar caminhos absolutos de compilação como padrão de execução.

## Experimentos — seção 13

Reservar quanta `1,3,5,7,9,11,13,15,17,19,21`. Recarregar BCPs e zerar contadores por execução, mantendo entradas. Futuro CSV:

```text
quantum,total_processos,total_instrucoes,total_despachos,media_trocas,media_instrucoes,aproveitamento
```

`aproveitamento = media_instrucoes / quantum` é fração média da fatia usada; não mede utilização real de CPU. Recomendar quantum somente após obter dados. Logs, CSV, gráficos e PDF experimentais não são produzidos pelo boilerplate.

## Diferenças da referência histórica

Usar `programas/`, quantidade dinâmica e créditos no BCP. A tabela independente inclui o executando. Bloqueados seguem FIFO, redistribuição considera todos os ativos e E/S não é relida. A espera recém-criada não diminui no mesmo ciclo. Logs usam escritor por execução, sem redirecionamento global do terminal. Não importar exigência de Java ou regras de entrega de 2017.
