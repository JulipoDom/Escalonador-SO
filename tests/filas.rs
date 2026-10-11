use ep1_escalonador::{
    erros::ErroEp,
    filas::{FilaBloqueados, FilaProntos},
    modelos::{Bcp, EstadoProcesso, Instrucao, ProcessId, Programa},
    tabela_processos::TabelaProcessos,
};

fn tabela(dados: &[(u32, u32, &str)]) -> TabelaProcessos {
    let mut tabela = TabelaProcessos::nova();
    for &(pid, prioridade, arquivo) in dados {
        tabela
            .inserir(Bcp::novo(
                ProcessId(pid),
                Programa {
                    nome: format!("P{pid}"),
                    arquivo: arquivo.into(),
                    instrucoes: vec![Instrucao::Saida].into_boxed_slice(),
                },
                prioridade,
            ))
            .unwrap();
    }
    tabela
}
fn ids(fila: &FilaProntos) -> Vec<u32> {
    fila.ids().iter().map(|pid| pid.0).collect()
}
fn invalido<T>(resultado: Result<T, ErroEp>) {
    assert!(matches!(resultado, Err(ErroEp::EstadoInvalido(_))));
}

#[test]
fn carga_inicial_ordena_creditos_arquivo_e_pid_e_exclui_nao_prontos() {
    let mut tabela = tabela(&[
        (4, 3, "b"),
        (1, 3, "a"),
        (2, 3, "a"),
        (3, 8, "c"),
        (5, 9, "d"),
        (6, 10, "e"),
    ]);
    tabela.get_mut(ProcessId(5)).unwrap().estado = EstadoProcesso::Bloqueado;
    tabela.get_mut(ProcessId(6)).unwrap().estado = EstadoProcesso::Executando;
    let mut fila = FilaProntos::nova();
    fila.ordenar_inicial(&tabela).unwrap();
    assert_eq!(ids(&fila), [3, 1, 2, 4]);
    fila.ordenar_inicial(&tabela).unwrap();
    assert_eq!(ids(&fila), [3, 1, 2, 4]);
    assert_eq!(fila.selecionar(&tabela).unwrap(), Some(ProcessId(3)));
    assert_eq!(
        tabela.get(ProcessId(3)).unwrap().estado,
        EstadoProcesso::Pronto
    );
}

#[test]
fn selecao_consulta_creditos_atuais_e_preserva_fifo_dos_empates() {
    let mut tabela = tabela(&[(1, 2, "a"), (2, 2, "b"), (3, 1, "c")]);
    let mut fila = FilaProntos::nova();
    fila.ordenar_inicial(&tabela).unwrap();
    tabela.get_mut(ProcessId(3)).unwrap().creditos = 5;
    assert_eq!(fila.selecionar(&tabela).unwrap(), Some(ProcessId(3)));
    assert_eq!(fila.selecionar(&tabela).unwrap(), Some(ProcessId(1)));
    assert_eq!(fila.selecionar(&tabela).unwrap(), Some(ProcessId(2)));
    assert_eq!(fila.selecionar(&tabela).unwrap(), None);
}

#[test]
fn reordenacao_apos_redistribuicao_preserva_empates_existentes() {
    let mut tabela = tabela(&[(1, 2, "a"), (2, 3, "b"), (3, 1, "c")]);
    let mut fila = FilaProntos::nova();
    fila.ordenar_inicial(&tabela).unwrap();
    assert_eq!(ids(&fila), [2, 1, 3]);
    tabela.get_mut(ProcessId(1)).unwrap().creditos = 7;
    tabela.get_mut(ProcessId(2)).unwrap().creditos = 7;
    tabela.get_mut(ProcessId(3)).unwrap().creditos = 9;
    fila.reordenar_apos_redistribuicao(&tabela).unwrap();
    assert_eq!(ids(&fila), [3, 2, 1]);
}

#[test]
fn filas_prontas_invalidas_nao_sofrem_mutacao_parcial() {
    let mut tabela = tabela(&[(1, 3, "a"), (2, 1, "b")]);
    let mut fila = FilaProntos::nova();
    fila.ordenar_inicial(&tabela).unwrap();
    tabela.remover(ProcessId(2));
    invalido(fila.selecionar(&tabela));
    assert_eq!(ids(&fila), [1, 2]);
    invalido(fila.reordenar_apos_redistribuicao(&tabela));
    assert_eq!(ids(&fila), [1, 2]);
    tabela
        .inserir(Bcp::novo(
            ProcessId(2),
            Programa {
                nome: "P2".into(),
                arquivo: "b".into(),
                instrucoes: Box::new([]),
            },
            1,
        ))
        .unwrap();
    tabela.get_mut(ProcessId(2)).unwrap().estado = EstadoProcesso::Bloqueado;
    invalido(fila.selecionar(&tabela));
    assert_eq!(ids(&fila), [1, 2]);
}

#[test]
fn fila_inicial_vazia_permite_selecao_e_reordenacao() {
    let tabela = TabelaProcessos::nova();
    let mut fila = FilaProntos::nova();
    fila.ordenar_inicial(&tabela).unwrap();
    fila.reordenar_apos_redistribuicao(&tabela).unwrap();
    assert_eq!(fila.selecionar(&tabela).unwrap(), None);
}

#[test]
fn quantum_precede_empates_e_es_retorna_depois_deles() {
    let tabela = tabela(&[(1, 3, "a"), (2, 3, "b"), (3, 3, "c"), (4, 1, "d")]);
    let mut fila = FilaProntos::nova();
    fila.ordenar_inicial(&tabela).unwrap();
    assert_eq!(fila.selecionar(&tabela).unwrap(), Some(ProcessId(1)));
    fila.reinserir_apos_quantum(ProcessId(1), &tabela).unwrap();
    assert_eq!(ids(&fila), [1, 2, 3, 4]);
    fila.selecionar(&tabela).unwrap();
    fila.reinserir_apos_es(ProcessId(1), &tabela).unwrap();
    assert_eq!(ids(&fila), [2, 3, 1, 4]);
    fila.selecionar(&tabela).unwrap();
    fila.reinserir_apos_es(ProcessId(2), &tabela).unwrap();
    assert_eq!(ids(&fila), [3, 1, 2, 4]);
}

#[test]
fn reinsercao_encontra_posicao_por_credito_atual() {
    let mut tabela = tabela(&[(1, 8, "a"), (2, 5, "b"), (3, 2, "c")]);
    let mut fila = FilaProntos::nova();
    fila.ordenar_inicial(&tabela).unwrap();
    fila.selecionar(&tabela).unwrap();
    tabela.get_mut(ProcessId(1)).unwrap().creditos = 3;
    fila.reinserir_apos_quantum(ProcessId(1), &tabela).unwrap();
    assert_eq!(ids(&fila), [2, 1, 3]);
}

#[test]
fn prontos_zero_com_bloqueado_positivo_rotacionam_fifo() {
    let mut tabela = tabela(&[(1, 3, "a"), (2, 2, "b"), (3, 7, "c")]);
    tabela.get_mut(ProcessId(1)).unwrap().creditos = 0;
    tabela.get_mut(ProcessId(2)).unwrap().creditos = 0;
    tabela.get_mut(ProcessId(3)).unwrap().estado = EstadoProcesso::Bloqueado;
    let mut fila = FilaProntos::nova();
    fila.ordenar_inicial(&tabela).unwrap();
    fila.selecionar(&tabela).unwrap();
    fila.reinserir_apos_quantum(ProcessId(1), &tabela).unwrap();
    assert_eq!(ids(&fila), [2, 1]);
    assert_eq!(fila.selecionar(&tabela).unwrap(), Some(ProcessId(2)));
    fila.reinserir_apos_quantum(ProcessId(2), &tabela).unwrap();
    assert_eq!(ids(&fila), [1, 2]);
}

#[test]
fn todas_prioridades_zero_rotacionam_fifo_sem_bloqueados() {
    let tabela = tabela(&[(1, 0, "a"), (2, 0, "b")]);
    let mut fila = FilaProntos::nova();
    fila.ordenar_inicial(&tabela).unwrap();
    fila.selecionar(&tabela).unwrap();
    fila.reinserir_apos_quantum(ProcessId(1), &tabela).unwrap();
    assert_eq!(ids(&fila), [2, 1]);
}

#[test]
fn reinsercao_rejeita_ausente_duplicado_estado_incorreto_e_fila_invalida() {
    let mut tabela = tabela(&[(1, 3, "a"), (2, 2, "b")]);
    let mut fila = FilaProntos::nova();
    fila.ordenar_inicial(&tabela).unwrap();
    invalido(fila.reinserir_apos_quantum(ProcessId(1), &tabela));
    invalido(fila.reinserir_apos_es(ProcessId(99), &tabela));
    assert_eq!(ids(&fila), [1, 2]);
    fila.selecionar(&tabela).unwrap();
    tabela.get_mut(ProcessId(1)).unwrap().estado = EstadoProcesso::Executando;
    invalido(fila.reinserir_apos_quantum(ProcessId(1), &tabela));
    invalido(fila.reinserir_apos_es(ProcessId(1), &tabela));
    assert_eq!(ids(&fila), [2]);
    tabela.get_mut(ProcessId(1)).unwrap().estado = EstadoProcesso::Pronto;
    tabela.remover(ProcessId(2));
    invalido(fila.reinserir_apos_quantum(ProcessId(1), &tabela));
    assert_eq!(ids(&fila), [2]);
}

fn iniciar_bloqueio(fila: &mut FilaBloqueados, tabela: &mut TabelaProcessos, pid: u32) {
    tabela.get_mut(ProcessId(pid)).unwrap().estado = EstadoProcesso::Executando;
    fila.bloquear(ProcessId(pid), tabela).unwrap();
}
fn ids_bloqueados(fila: &FilaBloqueados) -> Vec<u32> {
    fila.ids().iter().map(|pid| pid.0).collect()
}

#[test]
fn bloqueio_define_espera_dois_e_preserva_contexto() {
    let mut tabela = tabela(&[(1, 4, "a")]);
    let bcp = tabela.get_mut(ProcessId(1)).unwrap();
    bcp.pc = 5;
    bcp.x = 7;
    bcp.y = -9;
    let mut fila = FilaBloqueados::nova();
    iniciar_bloqueio(&mut fila, &mut tabela, 1);
    let bcp = tabela.get(ProcessId(1)).unwrap();
    assert_eq!(
        (bcp.estado, bcp.espera, bcp.pc, bcp.x, bcp.y, bcp.creditos),
        (EstadoProcesso::Bloqueado, 2, 5, 7, -9, 4)
    );
    assert_eq!(ids_bloqueados(&fila), [1]);
}

#[test]
fn novo_bloqueado_nao_perde_espera_no_proprio_surto() {
    let mut tabela = tabela(&[(1, 4, "a"), (2, 3, "b")]);
    let mut fila = FilaBloqueados::nova();
    iniciar_bloqueio(&mut fila, &mut tabela, 1);
    let anteriores: Vec<_> = fila.ids().iter().copied().collect();
    iniciar_bloqueio(&mut fila, &mut tabela, 2);
    assert!(fila
        .avancar_esperas(&anteriores, &mut tabela)
        .unwrap()
        .is_empty());
    assert_eq!(tabela.get(ProcessId(1)).unwrap().espera, 1);
    assert_eq!(tabela.get(ProcessId(2)).unwrap().espera, 2);
    assert_eq!(
        fila.avancar_esperas(&[ProcessId(1), ProcessId(2)], &mut tabela)
            .unwrap(),
        [ProcessId(1)]
    );
    assert_eq!(ids_bloqueados(&fila), [2]);
    let liberado = tabela.get(ProcessId(1)).unwrap();
    assert_eq!(
        (liberado.estado, liberado.espera),
        (EstadoProcesso::Pronto, 0)
    );
    assert_eq!(
        fila.avancar_esperas(&[ProcessId(2)], &mut tabela).unwrap(),
        [ProcessId(2)]
    );
}

#[test]
fn esperas_liberam_em_fifo_mesmo_com_snapshot_invertido() {
    let mut tabela = tabela(&[(1, 3, "a"), (2, 3, "b"), (3, 3, "c")]);
    let mut fila = FilaBloqueados::nova();
    for pid in [2, 1, 3] {
        iniciar_bloqueio(&mut fila, &mut tabela, pid);
    }
    assert!(fila
        .avancar_esperas(&[ProcessId(3), ProcessId(1), ProcessId(2)], &mut tabela)
        .unwrap()
        .is_empty());
    assert_eq!(
        fila.avancar_esperas(&[ProcessId(3), ProcessId(1), ProcessId(2)], &mut tabela)
            .unwrap(),
        [ProcessId(2), ProcessId(1), ProcessId(3)]
    );
    assert!(fila.ids().is_empty());
}

#[test]
fn tick_ocioso_nao_altera_cpu_ou_creditos() {
    let mut tabela = tabela(&[(1, 3, "a")]);
    let mut fila = FilaBloqueados::nova();
    iniciar_bloqueio(&mut fila, &mut tabela, 1);
    assert!(fila.avancar_esperas(&[], &mut tabela).unwrap().is_empty());
    assert_eq!(tabela.get(ProcessId(1)).unwrap().espera, 2);
    for _ in 0..2 {
        let anteriores: Vec<_> = fila.ids().iter().copied().collect();
        fila.avancar_esperas(&anteriores, &mut tabela).unwrap();
        let bcp = tabela.get(ProcessId(1)).unwrap();
        assert_eq!((bcp.pc, bcp.x, bcp.y, bcp.creditos), (0, 0, 0, 3));
    }
    assert!(fila.ids().is_empty());
    assert!(fila.avancar_esperas(&[], &mut tabela).unwrap().is_empty());
}

#[test]
fn bloqueio_invalido_nao_altera_fila_ou_tabela() {
    let mut tabela = tabela(&[(1, 3, "a"), (2, 2, "b")]);
    let mut fila = FilaBloqueados::nova();
    let antes = format!("{tabela:?}");
    invalido(fila.bloquear(ProcessId(1), &mut tabela));
    invalido(fila.bloquear(ProcessId(99), &mut tabela));
    assert_eq!(format!("{tabela:?}"), antes);
    assert!(fila.ids().is_empty());
    iniciar_bloqueio(&mut fila, &mut tabela, 1);
    let antes = format!("{tabela:?}");
    invalido(fila.bloquear(ProcessId(1), &mut tabela));
    assert_eq!(format!("{tabela:?}"), antes);
    tabela.get_mut(ProcessId(2)).unwrap().estado = EstadoProcesso::Executando;
    tabela.remover(ProcessId(1));
    let antes = format!("{tabela:?}");
    invalido(fila.bloquear(ProcessId(2), &mut tabela));
    assert_eq!(format!("{tabela:?}"), antes);
    assert_eq!(ids_bloqueados(&fila), [1]);
}

#[test]
fn snapshots_invalidos_nao_descontam_esperas_parcialmente() {
    let mut tabela = tabela(&[(1, 3, "a"), (2, 2, "b"), (3, 1, "c")]);
    let mut fila = FilaBloqueados::nova();
    iniciar_bloqueio(&mut fila, &mut tabela, 1);
    iniciar_bloqueio(&mut fila, &mut tabela, 2);
    for snapshot in [
        vec![ProcessId(1), ProcessId(99)],
        vec![ProcessId(1), ProcessId(1)],
        vec![ProcessId(1), ProcessId(3)],
    ] {
        let antes = format!("{tabela:?}");
        invalido(fila.avancar_esperas(&snapshot, &mut tabela));
        assert_eq!(format!("{tabela:?}"), antes);
        assert_eq!(ids_bloqueados(&fila), [1, 2]);
    }
    tabela.get_mut(ProcessId(2)).unwrap().estado = EstadoProcesso::Pronto;
    let antes = format!("{tabela:?}");
    invalido(fila.avancar_esperas(&[ProcessId(1)], &mut tabela));
    assert_eq!(format!("{tabela:?}"), antes);
}
