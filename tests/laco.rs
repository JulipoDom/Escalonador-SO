use ep1_escalonador::{
    erros::ErroEp,
    modelos::{Bcp, Instrucao, ProcessId, Programa},
    simulador::executar_laco,
};
use std::collections::HashMap;

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

#[test]
fn vazio_conclui_com_mapa_vazio() {
    assert_eq!(executar_laco(vec![], 1).unwrap(), HashMap::new());
}

#[test]
fn quantum_zero_recusa_antes_de_preparar_inclusive_vazio() {
    for processos in [vec![], vec![processo(1, 1, &[Instrucao::Saida])]] {
        assert!(matches!(
            executar_laco(processos, 0),
            Err(ErroEp::Entrada(_))
        ));
    }
}

#[test]
fn um_processo_entrega_registradores_reais_com_quantum_curto_e_longo() {
    use Instrucao::*;
    for quantum in [1, 4, usize::MAX] {
        assert_eq!(
            executar_laco(
                vec![processo(1, 2, &[AtribuirX(8), Com, AtribuirY(-4), Saida])],
                quantum
            )
            .unwrap(),
            HashMap::from([(ProcessId(1), (8, -4))])
        );
    }
}

#[test]
fn diferentes_processos_e_prioridades_preservam_todos_os_resultados() {
    use Instrucao::*;
    assert_eq!(
        executar_laco(
            vec![
                processo(3, 1, &[AtribuirY(7), Saida]),
                processo(1, 4, &[AtribuirX(-8), Com, Saida]),
                processo(2, 2, &[AtribuirX(2), AtribuirY(-3), Saida]),
            ],
            2
        )
        .unwrap(),
        HashMap::from([
            (ProcessId(1), (-8, 0)),
            (ProcessId(2), (2, -3)),
            (ProcessId(3), (0, 7))
        ])
    );
}

#[test]
fn contexto_pronto_retoma_pc_e_registradores_sem_reinicializar() {
    use Instrucao::*;
    let mut bcp = processo(1, 3, &[AtribuirX(99), AtribuirY(3), Saida]);
    bcp.pc = 1;
    bcp.x = -12;
    bcp.y = 88;
    bcp.creditos = 0;
    assert_eq!(
        executar_laco(vec![bcp], 1).unwrap(),
        HashMap::from([(ProcessId(1), (-12, 3))])
    );
}

#[test]
fn todos_bloqueados_continuam_ate_concluir_com_es_repetida() {
    use Instrucao::*;
    for quantum in [1, 3] {
        assert_eq!(
            executar_laco(
                vec![
                    processo(
                        1,
                        3,
                        &[
                            EntradaSaida,
                            AtribuirX(9),
                            EntradaSaida,
                            AtribuirY(4),
                            Saida
                        ]
                    ),
                    processo(
                        2,
                        2,
                        &[
                            EntradaSaida,
                            AtribuirY(-2),
                            EntradaSaida,
                            AtribuirX(7),
                            Saida
                        ]
                    ),
                ],
                quantum
            )
            .unwrap(),
            HashMap::from([(ProcessId(1), (9, 4)), (ProcessId(2), (7, -2))])
        );
    }
}

#[test]
fn unico_processo_retorna_de_es_repetida() {
    use Instrucao::*;
    for quantum in [1, 3] {
        assert_eq!(
            executar_laco(
                vec![processo(
                    1,
                    1,
                    &[EntradaSaida, EntradaSaida, AtribuirX(6), Saida]
                )],
                quantum
            )
            .unwrap(),
            HashMap::from([(ProcessId(1), (6, 0))])
        );
    }
}

#[test]
fn prioridades_zero_concluem_com_quantum_e_es() {
    use Instrucao::*;
    assert_eq!(
        executar_laco(
            vec![
                processo(1, 0, &[Com, EntradaSaida, AtribuirX(5), Saida]),
                processo(2, 0, &[EntradaSaida, AtribuirY(-9), Saida]),
            ],
            1
        )
        .unwrap(),
        HashMap::from([(ProcessId(1), (5, 0)), (ProcessId(2), (0, -9))])
    );
}

#[test]
fn pronto_sem_credito_continua_enquanto_bloqueado_tem_credito() {
    use Instrucao::*;
    let mut pronto = processo(2, 0, &[Com, Com, AtribuirY(8), Saida]);
    pronto.creditos = 0;
    assert_eq!(
        executar_laco(
            vec![processo(1, 5, &[EntradaSaida, AtribuirX(4), Saida]), pronto,],
            1
        )
        .unwrap(),
        HashMap::from([(ProcessId(1), (4, 0)), (ProcessId(2), (0, 8))])
    );
}

#[test]
fn redistribuicoes_repetidas_entregam_todos_os_registradores() {
    use Instrucao::*;
    assert_eq!(
        executar_laco(
            vec![
                processo(1, 1, &[Com, Com, Com, Com, AtribuirX(11), Saida]),
                processo(2, 1, &[Com, Com, Com, Com, Com, AtribuirY(22), Saida]),
            ],
            1
        )
        .unwrap(),
        HashMap::from([(ProcessId(1), (11, 0)), (ProcessId(2), (0, 22))])
    );
}

#[test]
fn saida_curta_e_es_antes_e_na_fronteira_do_quantum_concluem() {
    use Instrucao::*;
    assert_eq!(
        executar_laco(
            vec![
                processo(1, 5, &[Saida]),
                processo(2, 3, &[EntradaSaida, AtribuirX(7), Saida]),
                processo(3, 2, &[Com, Com, EntradaSaida, AtribuirY(6), Saida]),
            ],
            3
        )
        .unwrap(),
        HashMap::from([
            (ProcessId(1), (0, 0)),
            (ProcessId(2), (7, 0)),
            (ProcessId(3), (0, 6))
        ])
    );
}

#[test]
fn pid_duplicado_e_rejeitado_antes_da_execucao() {
    assert!(matches!(
        executar_laco(
            vec![
                processo(1, 1, &[Instrucao::Saida]),
                processo(1, 2, &[Instrucao::Saida])
            ],
            1
        ),
        Err(ErroEp::EstadoInvalido(_))
    ));
}

#[test]
fn estados_iniciais_incompativeis_e_espera_em_pronto_sao_rejeitados() {
    use ep1_escalonador::modelos::EstadoProcesso;
    for estado in [
        EstadoProcesso::Executando,
        EstadoProcesso::Bloqueado,
        EstadoProcesso::Pronto,
    ] {
        let mut bcp = processo(1, 1, &[Instrucao::Saida]);
        bcp.estado = estado;
        bcp.espera = if estado == EstadoProcesso::Executando {
            0
        } else {
            1
        };
        assert!(matches!(
            executar_laco(vec![bcp], 1),
            Err(ErroEp::EstadoInvalido(_))
        ));
    }
}

#[test]
fn pc_invalido_e_programa_sem_saida_propagam_erro_inclusive_apos_termino() {
    use Instrucao::*;
    for apos_termino in [false, true] {
        for sem_saida in [false, true] {
            let mut invalido = processo(2, 1, if sem_saida { &[AtribuirX(9)] } else { &[Saida] });
            if !sem_saida {
                invalido.pc = 1;
            }
            let mut processos = vec![invalido];
            if apos_termino {
                processos.push(processo(1, 5, &[AtribuirY(4), Saida]));
            }
            assert!(matches!(
                executar_laco(processos, 2),
                Err(ErroEp::EstadoInvalido(_))
            ));
        }
    }
}

#[test]
fn entradas_oficiais_concluem_com_valores_conferidos_nos_arquivos() {
    use ep1_escalonador::carregador::{carregar_programas, ler_quantum};
    let diretorio = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("programas");
    let processos = carregar_programas(&diretorio).unwrap();
    let quantum = ler_quantum(&diretorio).unwrap();
    assert_eq!(quantum, 3);
    // Últimas atribuições lidas diretamente em 01.txt..10.txt. COM e E/S
    // preservam os registradores; 09 e 10 não contêm atribuições.
    let esperado = HashMap::from([
        (ProcessId(1), (2, 10)),
        (ProcessId(2), (9, 1)),
        (ProcessId(3), (4, 10)),
        (ProcessId(4), (91, 5)),
        (ProcessId(5), (8, 5)),
        (ProcessId(6), (8, 5)),
        (ProcessId(7), (12, 3)),
        (ProcessId(8), (1, 3)),
        (ProcessId(9), (0, 0)),
        (ProcessId(10), (0, 0)),
    ]);
    assert_eq!(executar_laco(processos, quantum).unwrap(), esperado);
}
