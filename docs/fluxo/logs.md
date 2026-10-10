# Logs e métricas — tarefa 6

Skills consultadas: test-driven-development, sua referência writing-good-tests.md e verification-before-completion. using-superpowers foi consultada e dispensa agentes delegados. Plano seguido: docs/superpowers/plans/2026-10-10-boilerplate.md, Task 6. Fontes de contrato: seções 4, 6, 9 e 13 do documento de boilerplate.

RegistroLog possui BufWriter<File>, cria diretórios, usa log{quantum:02}.txt e escreve UTF-8 sem redirecionar stdout. EventoLog tipado implementa Display com os cinco eventos obrigatórios; a contagem um usa “instrução”. registrar, escrever_resumo e finalizar fazem flush e propagam ErroEp::Io. O escritor apenas formata o Resumo recebido e rejeita total_processos ou total_despachos zero antes de escrever. A sequência de eventos e a contagem do último surto pertencem ao futuro simulador.

Estatisticas::registrar exige contexto mutável local, sem estado global. A representação dos contadores permanece reservada até implementar a coleta. calcular_resumo e executar_experimentos retornam NaoImplementado específico; nenhum resultado experimental é calculado ou publicado.

Evidência TDD: após skeleton compilável, cargo test --test registro_estatisticas apresentou 5 falhas e 1 aprovação: eventos vazios, criação pendente, ausência de escrita/flush e validação pendente. Após implementação, o mesmo comando apresentou 6 aprovações, nenhuma falha ou ignorado. Os testes usam diretórios temporários com limpeza por Drop, verificam bytes antes da finalização e exercitam falha real de flush via /dev/full no Linux. Não foram criados logs ou CSV no workspace.

Primeira tentativa de cargo test global foi bloqueada por E0432 em tests/simulador.rs: executar_instrucao, executar_quantum e simular ainda ausentes nos módulos em edição concorrente; também apareceu unused_parens no teste do simulador. O coordenador foi avisado para verificar novamente após integração. Uma tentativa inicial local usou a raiz superior sem Cargo.toml e foi corrigida para o diretório do pacote antes da evidência RED acima.

Pendências: contadores e cálculo de médias; simulações independentes, CSV e análise experimental; integração de eventos com o núcleo. Formatação realizada somente nos quatro arquivos Rust deste domínio.
