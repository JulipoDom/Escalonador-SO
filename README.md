# EP1 — Escalonador

Escalonador do EP1 de Sistemas Operacionais de 2026 em Rust stable, edição 2021, com Cargo e somente a biblioteca padrão. Os 12 arquivos oficiais de `programas/` são preservados: dez processos, `prioridades.txt` e `quantum.txt`. O [enunciado](ep1.pdf) define os requisitos.

A execução integrada foi registrada em `bf093f1`; os 15 experimentos, suas duas coleções de logs e o relatório foram incluídos em `cb6d5f6`. As correções da revisão estão implementadas e verificadas localmente na branch `codex/correcoes-relatorio-ep1`, sobre `cb6d5f6`. As correções foram registradas em `884d7f5` e publicadas somente em `origin/codex/correcoes-relatorio-ep1`. O push retornou código 0; não houve merge, e `main` local/remota permaneceu em `cb6d5f6`. Este registro documental acompanha a publicação das correções na mesma branch; identifica o commit `884d7f5`, sem atribuir um hash ao próprio registro. A revisão independente final do conjunto foi aprovada em conformidade e qualidade, sem achados pendentes. O [histórico](HISTORICO.md) distingue as verificações anteriores das verificações desta correção.

## Executar o escalonador

Execute na raiz do projeto, onde está `programas/`:

```bash
cargo run --bin Escalonador
```

Para compilar a versão otimizada, use `cargo build --release --bin Escalonador` e execute `./target/release/Escalonador`. O binário resolve `programas/` e `logXX.txt` pelo diretório corrente. Primeiro valida programas, prioridades e quantum; depois publica o log correspondente ao quantum lido. Logs de outros quantums são preservados. Em sucesso, encerra com código 0 e stdout/stderr vazios após publicar os eventos e o resumo reais; falhas retornam código 1 com diagnóstico em stderr. Argumentos são recusados antes da criação do log. Não há CLI configurável, lote automático ou CSV.

A correção da publicação substitui o nome de destino por um arquivo novo, preservando o alvo de links simbólicos e hardlinks preexistentes. Um destino que seja diretório causa erro. Cada evento é publicado durante a execução; eventos reais parciais podem permanecer após falha de CPU ou escrita, sem um retorno de sucesso.

## Modelo e convenções

- A tabela possui os BCPs; filas guardam somente `ProcessId`. O BCP mantém PC, estado, prioridade, créditos, registradores, programa e espera de E/S.
- Toda instrução, inclusive E/S e SAIDA, consome uma unidade e avança o PC uma vez. E/S e SAIDA prevalecem sobre o limite do quantum; a retomada começa na instrução seguinte.
- Créditos começam na prioridade e caem um por despacho sem ficar negativos. A redistribuição ocorre somente quando todos os ativos, inclusive bloqueados, têm zero.
- Empate inicial segue a ordem dos arquivos. Ao fim do quantum, o processo volta antes dos créditos iguais; no retorno de E/S, depois dos iguais. Se prontos têm zero e algum bloqueado tem crédito, usar FIFO; prioridades todas zero também usam FIFO.
- E/S bloqueia por dois surtos posteriores, inclusive surtos curtos e repetições de um mesmo outro PID. O surto que bloqueou não reduz a própria espera. Liberação segue FIFO; sem prontos, somente a espera lógica avança.
- `executar_laco` consome os passos até a conclusão e devolve os registradores finais reais. Entrada vazia com quantum positivo devolve mapa vazio. `simular` recusa entrada vazia pelos denominadores indefinidos e integra eventos e estatísticas ao núcleo.
- PC inválido e quantum zero são recusados; erros posteriores preservam o progresso válido. Erro da CPU após despacho preserva o BCP Executando fora das filas, sem recuperação automática.

`simular` registra Carregando na ordem inicial de prontos, Executando antes de cada surto, E/S antes de Interrompendo e Terminado com X/Y finais reais. A interrupção informa apenas as instruções do último surto. Para P processos iniciais, D despachos concluídos e I instruções executadas, as médias são D/P e I/D. D inclui término e despachos consecutivos do mesmo PID; I inclui E/S e SAIDA; ociosidade não conta. A contagem de trocas definida no EP não equivale necessariamente à troca física de contexto.

## Experimentos e relatório

Há logs para os quantums `1,2,3,4,5,6,7,8,9,10,12,15,20,25,30` na raiz e em `experimentos/`. A série principal `1…10` é uniforme; os demais pontos exploram a saturação. A carga oficial soma 139 instruções e 33 segmentos terminados por E/S ou SAIDA; o maior segmento tem 21 instruções. A partir de q21, são necessários 33 despachos e a média máxima desta carga é 139/33.

O conteúdo e os dados do relatório têm uma fonte única em `gerar_relatorio.py`: o gerador lê e valida programas, eventos e resumos das duas coleções de logs antes de produzir `relatorio_ep1.pdf` e `relatorio_ep1.tex`. Ele reconstrói PC, instruções e registradores dos eventos; não executa novos experimentos nem roda o binário Rust. As entradas e as saídas são resolvidas pela raiz do próprio script, inclusive quando invocado de outro diretório.

A geração deste relatório é específica da carga oficial original: exige os mesmos dez arquivos, nomes, prioridades e instruções, os 15 quantums previstos e uma recomendação compatível com q7. Uma divergência é recusada antes de renderizar ou substituir PDF/TEX. As funções de carregamento, validação e coleta permanecem genéricas para testes; gerar um relatório de outra carga exige adaptar seu conteúdo e recorte. A leitura UTF-8 usa as mesmas regras de linhas LF/CRLF do carregador Rust. `quantum.txt` precisa conter um valor positivo; q3 é o valor da entrada original fornecida, não uma exigência de configuração atual.

O escalonador Rust continua usando somente `std`. A geração do PDF usa Python e ReportLab, cuja dependência está em `requirements-relatorio.txt`. Para preparar o ambiente Python e gerar o relatório:

```bash
python3 -m pip install -r requirements-relatorio.txt
python3 gerar_relatorio.py
```

O TEX é gerado do mesmo conteúdo do PDF e pode ser aberto no editor LaTeX para edição e preview. Regenerar pelo script substitui ambos os arquivos; para manter alterações, edite o conteúdo no gerador. O PDF produzido pelo script é o artefato de entrega.

q7 permanece como recomendação prática para esta carga: é o menor quantum testado que alcança pelo menos 85% da média máxima de instruções por despacho. O critério é escolhido pelos autores, sem alegar ótimo universal, tempo de resposta aceitável, throughput ou custo real não medidos. O relatório distingue a razão `(I/D)/q` de utilização real da CPU.

## Verificar e preparar a entrega

```bash
cargo check --all-targets
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
python3 -m unittest discover -s tests -p 'test_relatorio.py'
```

O coordenador verificou a correção com 127 testes Rust e 24 testes Python aprovados, zero falhas e zero ignorados; check, fmt, Clippy, build e conferência de whitespace também aprovados. As 15 reexecuções oficiais conferem byte a byte com ambas as coleções de logs; 54 arquivos protegidos foram preservados. O PDF final tem sete páginas A4, todas renderizadas e inspecionadas. O TEX foi aberto no editor, mas sua compilação não foi verificada: o compilador integrado não conseguiu obter o bundle Tectonic necessário. O PDF ReportLab é o artefato visual verificado. Os detalhes ficam no [histórico](HISTORICO.md). Faltam conferir o material e preparar o ZIP com código, logs e PDF conforme o enunciado. O prazo escrito no enunciado é 04/10/2026; a situação administrativa e eventual submissão devem ser confirmadas com a disciplina. Não há submissão registrada nesta entrega.

Fluxo de desenvolvimento: [AGENTS](AGENTS.md), [contexto vigente](docs/superpowers/plans/2026-10-10-contexto-atual.md), [plano das correções](docs/superpowers/plans/2026-10-11-correcoes-revisao-relatorio.md) e [índice Superpowers](docs/superpowers/README.md).
