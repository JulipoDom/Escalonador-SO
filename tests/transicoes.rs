use ep1_escalonador::{
    erros::ErroEp,
    filas::{FilaBloqueados, FilaProntos},
    modelos::{Bcp, EstadoProcesso, Instrucao, MotivoParada, ProcessId, Programa},
    simulador::{executar_passo, ResultadoPasso},
    tabela_processos::TabelaProcessos,
};

fn processo(pid: u32, prioridade: u32, instrucoes: &[Instrucao]) -> Bcp {
    Bcp::novo(
        ProcessId(pid),
        Programa {
            nome: format!("P{pid}"),
            arquivo: format!("{pid:02}.txt"),
            instrucoes: instrucoes.into(),
        },
        prioridade,
    )
}

fn estado(processos: Vec<Bcp>) -> (TabelaProcessos, FilaProntos, FilaBloqueados) {
    let mut tabela = TabelaProcessos::nova();
    for bcp in processos {
        tabela.inserir(bcp).unwrap();
    }
    let mut prontos = FilaProntos::nova();
    prontos.ordenar_inicial(&tabela).unwrap();
    (tabela, prontos, FilaBloqueados::nova())
}

// Snapshot ordenado por PID, incluindo texto e metadados, sem ordem de HashMap.
fn snapshot(
    tabela: &TabelaProcessos,
    prontos: &FilaProntos,
    bloqueados: &FilaBloqueados,
) -> String {
    let mut processos: Vec<_> = tabela.iter().collect();
    processos.sort_by_key(|(pid, _)| pid.0);
    format!("{processos:?}|{:?}|{:?}", prontos.ids(), bloqueados.ids())
}

// Detecta consulta/mutação anterior à recusa de quantum zero.
#[test]
fn quantum_zero_recusa_antes_de_validar_estado_sem_mutacao() {
    for vazio in [false, true] {
        let processos = if vazio {
            vec![]
        } else {
            vec![processo(1, 4, &[Instrucao::Saida])]
        };
        let (mut tabela, mut prontos, mut bloqueados) = estado(processos);
        if !vazio {
            tabela.get_mut(ProcessId(1)).unwrap().estado = EstadoProcesso::Executando;
        }
        let antes = snapshot(&tabela, &prontos, &bloqueados);
        assert!(matches!(
            executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 0),
            Err(ErroEp::Entrada(_))
        ));
        assert_eq!(snapshot(&tabela, &prontos, &bloqueados), antes);
    }
}

// Detecta conclusão prematura e não reconhecimento de tabela/filas vazias.
#[test]
fn estado_vazio_valido_conclui() {
    let (mut tabela, mut prontos, mut bloqueados) = estado(vec![]);
    assert!(matches!(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 1).unwrap(),
        ResultadoPasso::Concluido
    ));
}

// Detecta despachos e descontos antes da validação completa de tabela e filas.
#[test]
fn inconsistencias_estruturais_recusadas_sem_mutacao() {
    for caso in 0..8 {
        let (mut tabela, mut prontos, mut bloqueados) = estado(vec![
            processo(1, 4, &[Instrucao::Saida]),
            processo(2, 3, &[Instrucao::Saida]),
        ]);
        match caso {
            0 => {
                tabela.remover(ProcessId(2));
            }
            1 => {
                prontos.selecionar(&tabela).unwrap();
            }
            2 => {
                tabela.get_mut(ProcessId(2)).unwrap().estado = EstadoProcesso::Executando;
            }
            3 => {
                tabela.get_mut(ProcessId(2)).unwrap().espera = 1;
            }
            4 => {
                tabela.get_mut(ProcessId(2)).unwrap().pid = ProcessId(99);
            }
            5 => {
                tabela.get_mut(ProcessId(2)).unwrap().estado = EstadoProcesso::Bloqueado;
                tabela.get_mut(ProcessId(2)).unwrap().espera = 2;
            }
            6 | 7 => {
                let pid = prontos.selecionar(&tabela).unwrap().unwrap();
                tabela.get_mut(pid).unwrap().estado = EstadoProcesso::Executando;
                bloqueados.bloquear(pid, &mut tabela).unwrap();
                if caso == 6 {
                    tabela.get_mut(pid).unwrap().espera = 0;
                } else {
                    tabela.get_mut(pid).unwrap().estado = EstadoProcesso::Pronto;
                    tabela.get_mut(pid).unwrap().espera = 0;
                    prontos.reinserir_apos_es(pid, &tabela).unwrap();
                }
            }
            _ => unreachable!(),
        }
        let antes = snapshot(&tabela, &prontos, &bloqueados);
        assert!(
            matches!(
                executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 1),
                Err(ErroEp::EstadoInvalido(_))
            ),
            "caso {caso}"
        );
        assert_eq!(
            snapshot(&tabela, &prontos, &bloqueados),
            antes,
            "caso {caso}"
        );
    }
}

fn surto(passo: ResultadoPasso, pid: u32, quantidade: usize, motivo: MotivoParada) -> Option<Bcp> {
    let ResultadoPasso::Surto {
        resultado,
        terminado,
    } = passo
    else {
        panic!("esperado surto")
    };
    assert_eq!(resultado.pid, ProcessId(pid));
    assert_eq!(resultado.instrucoes_executadas, quantidade);
    assert_eq!(resultado.motivo, motivo);
    terminado
}

// Detecta estado errado na reinserção, desconto por instrução ou empate invertido.
#[test]
fn quantum_desconta_um_credito_e_reinsere_antes_de_iguais() {
    let (mut tabela, mut prontos, mut bloqueados) = estado(vec![
        processo(
            1,
            4,
            &[Instrucao::AtribuirX(8), Instrucao::Com, Instrucao::Saida],
        ),
        processo(2, 3, &[Instrucao::Saida]),
    ]);
    assert!(surto(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 2).unwrap(),
        1,
        2,
        MotivoParada::Quantum
    )
    .is_none());
    let a = tabela.get(ProcessId(1)).unwrap();
    assert_eq!(
        (a.pc, a.x, a.y, a.creditos, a.espera, a.estado),
        (2, 8, 0, 3, 0, EstadoProcesso::Pronto)
    );
    assert_eq!(prontos.ids(), &[ProcessId(1), ProcessId(2)]);
}

// Detecta perda do BCP removido, nome/contexto alterados ou SAIDA sem desconto.
#[test]
fn saida_remove_e_devolve_contexto_final_real() {
    let (mut tabela, mut prontos, mut bloqueados) = estado(vec![
        processo(
            1,
            4,
            &[
                Instrucao::AtribuirX(-8),
                Instrucao::AtribuirY(17),
                Instrucao::Saida,
            ],
        ),
        processo(2, 3, &[Instrucao::Saida]),
    ]);
    let terminado = surto(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 8).unwrap(),
        1,
        3,
        MotivoParada::Saida,
    )
    .unwrap();
    assert_eq!(
        (terminado.pc, terminado.x, terminado.y, terminado.creditos),
        (3, -8, 17, 3)
    );
    assert_eq!(terminado.programa.nome, "P1");
    assert_eq!(terminado.programa.arquivo, "01.txt");
    assert_eq!(terminado.prioridade, 4);
    assert_eq!(terminado.pid, ProcessId(1));
    assert_eq!(
        &*terminado.programa.instrucoes,
        &[
            Instrucao::AtribuirX(-8),
            Instrucao::AtribuirY(17),
            Instrucao::Saida
        ]
    );
    assert!(tabela.get(ProcessId(1)).is_none());
    assert_eq!(prontos.ids(), &[ProcessId(2)]);
    assert!(bloqueados.ids().is_empty());
}

// Detecta seleção feita antes de redistribuir e instabilidade nos novos empates.
#[test]
fn redistribuicao_antes_do_despacho_reordena_estavelmente() {
    let (mut tabela, mut prontos, mut bloqueados) = estado(vec![
        processo(1, 1, &[Instrucao::Com, Instrucao::Saida]),
        processo(2, 4, &[Instrucao::Com, Instrucao::Saida]),
        processo(3, 4, &[Instrucao::Com, Instrucao::Saida]),
    ]);
    for pid in [1, 2, 3] {
        tabela.get_mut(ProcessId(pid)).unwrap().creditos = 0;
    }
    // Ordenação zero usa o arquivo: 1,2,3. Redistribuição traz 2,3 antes de 1.
    prontos.ordenar_inicial(&tabela).unwrap();
    assert!(surto(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 1).unwrap(),
        2,
        1,
        MotivoParada::Quantum
    )
    .is_none());
    assert_eq!(prontos.ids(), &[ProcessId(3), ProcessId(2), ProcessId(1)]);
    assert_eq!(tabela.get(ProcessId(1)).unwrap().creditos, 1);
    assert_eq!(tabela.get(ProcessId(2)).unwrap().creditos, 3);
    assert_eq!(tabela.get(ProcessId(3)).unwrap().creditos, 4);
}

// Detecta redistribuição infinita ou reinserção na frente com prioridades zero.
#[test]
fn prioridades_todas_zero_progridem_em_fifo() {
    let (mut tabela, mut prontos, mut bloqueados) = estado(vec![
        processo(1, 0, &[Instrucao::Com, Instrucao::Saida]),
        processo(2, 0, &[Instrucao::Com, Instrucao::Saida]),
    ]);
    surto(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 1).unwrap(),
        1,
        1,
        MotivoParada::Quantum,
    );
    assert_eq!(prontos.ids(), &[ProcessId(2), ProcessId(1)]);
    surto(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 1).unwrap(),
        2,
        1,
        MotivoParada::Quantum,
    );
    assert_eq!(prontos.ids(), &[ProcessId(1), ProcessId(2)]);
    assert_eq!(tabela.get(ProcessId(1)).unwrap().creditos, 0);
    assert_eq!(tabela.get(ProcessId(2)).unwrap().creditos, 0);
}

fn bloquear(
    tabela: &mut TabelaProcessos,
    prontos: &mut FilaProntos,
    bloqueados: &mut FilaBloqueados,
    espera: u8,
) -> ProcessId {
    let pid = prontos.selecionar(tabela).unwrap().unwrap();
    tabela.get_mut(pid).unwrap().estado = EstadoProcesso::Executando;
    bloqueados.bloquear(pid, tabela).unwrap();
    tabela.get_mut(pid).unwrap().espera = espera;
    pid
}

// Detecta rollback do progresso válido, liberação de E/S ou nova execução após aborto.
#[test]
fn erro_cpu_preserva_despacho_abortado_e_esperas_anteriores() {
    let (mut tabela, mut prontos, mut bloqueados) = estado(vec![
        processo(1, 4, &[Instrucao::AtribuirX(8), Instrucao::AtribuirY(-9)]),
        processo(2, 8, &[Instrucao::Saida]),
    ]);
    bloquear(&mut tabela, &mut prontos, &mut bloqueados, 1);
    assert!(matches!(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 4),
        Err(ErroEp::EstadoInvalido(_))
    ));
    let a = tabela.get(ProcessId(1)).unwrap();
    assert_eq!(
        (a.pc, a.x, a.y, a.creditos, a.estado),
        (2, 8, -9, 3, EstadoProcesso::Executando)
    );
    assert!(prontos.ids().is_empty());
    assert_eq!(
        bloqueados.ids().iter().copied().collect::<Vec<_>>(),
        vec![ProcessId(2)]
    );
    assert_eq!(tabela.get(ProcessId(2)).unwrap().espera, 1);
    let antes = snapshot(&tabela, &prontos, &bloqueados);
    assert!(matches!(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 1),
        Err(ErroEp::EstadoInvalido(_))
    ));
    assert_eq!(snapshot(&tabela, &prontos, &bloqueados), antes);
}

// Detecta validação de PC como recusa anterior ao despacho ou conclusão fictícia.
#[test]
fn pc_inicial_invalido_aborta_apos_descontar_credito() {
    let (mut tabela, mut prontos, mut bloqueados) =
        estado(vec![processo(1, 4, &[Instrucao::Saida])]);
    tabela.get_mut(ProcessId(1)).unwrap().pc = usize::MAX;
    assert!(matches!(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 1),
        Err(ErroEp::EstadoInvalido(_))
    ));
    let a = tabela.get(ProcessId(1)).unwrap();
    assert_eq!(
        (a.pc, a.creditos, a.estado),
        (usize::MAX, 3, EstadoProcesso::Executando)
    );
    assert_eq!(tabela.len(), 1);
    assert!(prontos.ids().is_empty());
}

// Detecta desconto do próprio surto e contagem por PID distinto em vez de surto.
#[test]
fn es_espera_dois_surtos_do_mesmo_pid_e_retoma_apos_instrucao_es() {
    let (mut tabela, mut prontos, mut bloqueados) = estado(vec![
        processo(
            1,
            6,
            &[
                Instrucao::EntradaSaida,
                Instrucao::AtribuirX(11),
                Instrucao::Saida,
            ],
        ),
        processo(
            2,
            4,
            &[
                Instrucao::Com,
                Instrucao::Com,
                Instrucao::Com,
                Instrucao::Saida,
            ],
        ),
    ]);
    assert!(surto(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 1).unwrap(),
        1,
        1,
        MotivoParada::Es
    )
    .is_none());
    let a = tabela.get(ProcessId(1)).unwrap();
    assert_eq!(
        (a.pc, a.creditos, a.espera, a.estado),
        (1, 5, 2, EstadoProcesso::Bloqueado)
    );
    assert_eq!(prontos.ids(), &[ProcessId(2)]);
    surto(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 1).unwrap(),
        2,
        1,
        MotivoParada::Quantum,
    );
    assert_eq!(tabela.get(ProcessId(1)).unwrap().espera, 1);
    surto(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 1).unwrap(),
        2,
        1,
        MotivoParada::Quantum,
    );
    let a = tabela.get(ProcessId(1)).unwrap();
    assert_eq!(
        (a.pc, a.creditos, a.espera, a.estado),
        (1, 5, 0, EstadoProcesso::Pronto)
    );
    assert_eq!(prontos.ids(), &[ProcessId(1), ProcessId(2)]);
    assert!(bloqueados.ids().is_empty());
    let terminado = surto(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 2).unwrap(),
        1,
        2,
        MotivoParada::Saida,
    )
    .unwrap();
    assert_eq!((terminado.pc, terminado.x, terminado.creditos), (3, 11, 4));
}

// Detecta snapshot capturado após bloquear, E/S ignorada na fronteira e surto curto sem tick.
#[test]
fn es_curta_ou_na_fronteira_avanca_apenas_bloqueados_anteriores() {
    for quantum in [2, 8] {
        let (mut tabela, mut prontos, mut bloqueados) = estado(vec![
            processo(
                1,
                6,
                &[Instrucao::Com, Instrucao::EntradaSaida, Instrucao::Saida],
            ),
            processo(2, 9, &[Instrucao::Saida]),
        ]);
        bloquear(&mut tabela, &mut prontos, &mut bloqueados, 2);
        assert!(surto(
            executar_passo(&mut tabela, &mut prontos, &mut bloqueados, quantum).unwrap(),
            1,
            2,
            MotivoParada::Es
        )
        .is_none());
        assert_eq!(
            (
                tabela.get(ProcessId(1)).unwrap().pc,
                tabela.get(ProcessId(1)).unwrap().espera
            ),
            (2, 2)
        );
        assert_eq!(tabela.get(ProcessId(2)).unwrap().espera, 1);
        assert_eq!(
            bloqueados.ids().iter().copied().collect::<Vec<_>>(),
            vec![ProcessId(2), ProcessId(1)]
        );
        assert!(prontos.ids().is_empty());
    }
}

// Detecta SAIDA retornando antes do tick e retorno de E/S colocado antes de crédito igual.
#[test]
fn saida_curta_avanca_espera_e_libera_apos_empates() {
    let (mut tabela, mut prontos, mut bloqueados) = estado(vec![
        processo(1, 9, &[Instrucao::Saida]),
        processo(2, 4, &[Instrucao::Saida]),
        processo(3, 3, &[Instrucao::Saida]),
    ]);
    bloquear(&mut tabela, &mut prontos, &mut bloqueados, 1);
    tabela.get_mut(ProcessId(1)).unwrap().creditos = 3;
    assert!(surto(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 8).unwrap(),
        2,
        1,
        MotivoParada::Saida
    )
    .is_some());
    assert_eq!(prontos.ids(), &[ProcessId(3), ProcessId(1)]);
    assert_eq!(
        (
            tabela.get(ProcessId(1)).unwrap().espera,
            tabela.get(ProcessId(1)).unwrap().estado
        ),
        (0, EstadoProcesso::Pronto)
    );
    assert!(bloqueados.ids().is_empty());
}

// Detecta liberação em ordem da tabela ou inserção reversa nos empates.
#[test]
fn liberacoes_simultaneas_seguem_fifo_apos_pronto_empatado() {
    let (mut tabela, mut prontos, mut bloqueados) = estado(vec![
        processo(1, 8, &[Instrucao::Saida]),
        processo(2, 7, &[Instrucao::Saida]),
        processo(3, 4, &[Instrucao::Com, Instrucao::Saida]),
    ]);
    assert_eq!(
        bloquear(&mut tabela, &mut prontos, &mut bloqueados, 1),
        ProcessId(1)
    );
    assert_eq!(
        bloquear(&mut tabela, &mut prontos, &mut bloqueados, 1),
        ProcessId(2)
    );
    tabela.get_mut(ProcessId(1)).unwrap().creditos = 3;
    tabela.get_mut(ProcessId(2)).unwrap().creditos = 3;
    surto(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 1).unwrap(),
        3,
        1,
        MotivoParada::Quantum,
    );
    assert_eq!(prontos.ids(), &[ProcessId(3), ProcessId(1), ProcessId(2)]);
    assert!(bloqueados.ids().is_empty());
}

// Detecta CPU, desconto ou redistribuição durante ociosidade; espera positiva não tem teto artificial.
#[test]
fn ociosidade_avanca_um_tick_sem_cpu_ou_creditos_mesmo_todos_zero() {
    let (mut tabela, mut prontos, mut bloqueados) = estado(vec![processo(
        1,
        5,
        &[Instrucao::AtribuirX(90), Instrucao::Saida],
    )]);
    bloquear(&mut tabela, &mut prontos, &mut bloqueados, 200);
    let a = tabela.get_mut(ProcessId(1)).unwrap();
    a.creditos = 0;
    a.x = -8;
    a.y = 17;
    let ResultadoPasso::Ocioso { liberados } =
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 9).unwrap()
    else {
        panic!("esperado tick ocioso")
    };
    assert!(liberados.is_empty());
    let a = tabela.get(ProcessId(1)).unwrap();
    assert_eq!(
        (a.pc, a.x, a.y, a.creditos, a.espera, a.estado),
        (0, -8, 17, 0, 199, EstadoProcesso::Bloqueado)
    );
    assert!(prontos.ids().is_empty());
}

// Detecta mais de um tick por chamada e conclusão enquanto existem contextos ativos.
#[test]
fn sequencia_manual_todos_bloqueados_ate_conclusao_real() {
    let (mut tabela, mut prontos, mut bloqueados) = estado(vec![
        processo(1, 3, &[Instrucao::EntradaSaida, Instrucao::Saida]),
        processo(2, 2, &[Instrucao::EntradaSaida, Instrucao::Saida]),
    ]);
    surto(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 1).unwrap(),
        1,
        1,
        MotivoParada::Es,
    );
    surto(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 1).unwrap(),
        2,
        1,
        MotivoParada::Es,
    );
    assert_eq!(
        (
            tabela.get(ProcessId(1)).unwrap().espera,
            tabela.get(ProcessId(2)).unwrap().espera
        ),
        (1, 2)
    );
    let ResultadoPasso::Ocioso { liberados } =
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 8).unwrap()
    else {
        panic!("esperado tick ocioso")
    };
    assert_eq!(liberados, vec![ProcessId(1)]);
    assert_eq!(prontos.ids(), &[ProcessId(1)]);
    let a = tabela.get(ProcessId(1)).unwrap();
    let b = tabela.get(ProcessId(2)).unwrap();
    assert_eq!((a.pc, a.x, a.y, a.creditos, a.espera), (1, 0, 0, 2, 0));
    assert_eq!((b.pc, b.x, b.y, b.creditos, b.espera), (1, 0, 0, 1, 1));
    surto(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 8).unwrap(),
        1,
        1,
        MotivoParada::Saida,
    )
    .unwrap();
    assert_eq!(tabela.len(), 1);
    assert_eq!(prontos.ids(), &[ProcessId(2)]);
    surto(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 8).unwrap(),
        2,
        1,
        MotivoParada::Saida,
    )
    .unwrap();
    assert!(tabela.is_empty());
    assert!(prontos.ids().is_empty());
    assert!(bloqueados.ids().is_empty());
    assert!(matches!(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 1).unwrap(),
        ResultadoPasso::Concluido
    ));
}

// Detecta redistribuição esquecendo bloqueados, ou feita ao final do surto em vez do próximo despacho.
#[test]
fn redistribuicao_restaura_tambem_bloqueados_antes_do_despacho() {
    let (mut tabela, mut prontos, mut bloqueados) = estado(vec![
        processo(1, 2, &[Instrucao::Com, Instrucao::Saida]),
        processo(2, 7, &[Instrucao::Saida]),
    ]);
    bloquear(&mut tabela, &mut prontos, &mut bloqueados, 2);
    tabela.get_mut(ProcessId(1)).unwrap().creditos = 0;
    tabela.get_mut(ProcessId(2)).unwrap().creditos = 0;
    surto(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 1).unwrap(),
        1,
        1,
        MotivoParada::Quantum,
    );
    assert_eq!(tabela.get(ProcessId(1)).unwrap().creditos, 1);
    assert_eq!(
        (
            tabela.get(ProcessId(2)).unwrap().creditos,
            tabela.get(ProcessId(2)).unwrap().espera
        ),
        (7, 1)
    );
}

// Detecta redistribuição prematura e reinserção de pronto zero antes de outros prontos zero.
#[test]
fn bloqueado_positivo_impede_redistribuicao_e_prontos_zero_rotacionam_fifo() {
    let (mut tabela, mut prontos, mut bloqueados) = estado(vec![
        processo(1, 3, &[Instrucao::Com, Instrucao::Saida]),
        processo(2, 2, &[Instrucao::Com, Instrucao::Saida]),
        processo(3, 8, &[Instrucao::Saida]),
    ]);
    bloquear(&mut tabela, &mut prontos, &mut bloqueados, 2);
    tabela.get_mut(ProcessId(1)).unwrap().creditos = 0;
    tabela.get_mut(ProcessId(2)).unwrap().creditos = 0;
    surto(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 1).unwrap(),
        1,
        1,
        MotivoParada::Quantum,
    );
    assert_eq!(prontos.ids(), &[ProcessId(2), ProcessId(1)]);
    assert_eq!(tabela.get(ProcessId(1)).unwrap().creditos, 0);
    assert_eq!(tabela.get(ProcessId(2)).unwrap().creditos, 0);
    assert_eq!(
        (
            tabela.get(ProcessId(3)).unwrap().creditos,
            tabela.get(ProcessId(3)).unwrap().espera
        ),
        (8, 1)
    );
}

// Detecta redistribuição realizada ao liberar o último bloqueado com créditos zero.
#[test]
fn ociosidade_libera_fifo_sem_redistribuir_ate_o_proximo_despacho() {
    let (mut tabela, mut prontos, mut bloqueados) = estado(vec![
        processo(1, 4, &[Instrucao::Com, Instrucao::Saida]),
        processo(2, 3, &[Instrucao::Saida]),
    ]);
    bloquear(&mut tabela, &mut prontos, &mut bloqueados, 1);
    bloquear(&mut tabela, &mut prontos, &mut bloqueados, 1);
    tabela.get_mut(ProcessId(1)).unwrap().creditos = 0;
    tabela.get_mut(ProcessId(2)).unwrap().creditos = 0;
    let ResultadoPasso::Ocioso { liberados } =
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 1).unwrap()
    else {
        panic!("esperado tick ocioso")
    };
    assert_eq!(liberados, vec![ProcessId(1), ProcessId(2)]);
    assert_eq!(prontos.ids(), &[ProcessId(1), ProcessId(2)]);
    assert_eq!(tabela.get(ProcessId(1)).unwrap().creditos, 0);
    assert_eq!(tabela.get(ProcessId(2)).unwrap().creditos, 0);
    surto(
        executar_passo(&mut tabela, &mut prontos, &mut bloqueados, 1).unwrap(),
        1,
        1,
        MotivoParada::Quantum,
    );
    assert_eq!(tabela.get(ProcessId(1)).unwrap().creditos, 3);
    assert_eq!(tabela.get(ProcessId(2)).unwrap().creditos, 3);
}
