use ep1_escalonador::{
    erros::ErroEp,
    modelos::{Bcp, Instrucao, ProcessId, Programa},
    tabela_processos::TabelaProcessos,
};

fn processo(pid: u32, prioridade: u32) -> Bcp {
    Bcp::novo(
        ProcessId(pid),
        Programa {
            nome: format!("P{pid}"),
            arquivo: format!("{pid:02}.txt"),
            instrucoes: vec![Instrucao::Saida].into_boxed_slice(),
        },
        prioridade,
    )
}

#[test]
fn tabela_preserva_contextos_e_remove_apenas_o_pid_solicitado() {
    let mut tabela = TabelaProcessos::nova();
    assert!(tabela.vazia());
    tabela.inserir(processo(1, 3)).unwrap();
    tabela.inserir(processo(2, 8)).unwrap();
    tabela.get_mut(ProcessId(1)).unwrap().x = 42;
    assert_eq!(tabela.get(ProcessId(1)).unwrap().x, 42);
    assert_eq!(tabela.get(ProcessId(2)).unwrap().creditos, 8);
    assert_eq!(tabela.len(), 2);
    let mut ids: Vec<_> = tabela.iter().map(|(pid, _)| pid.0).collect();
    ids.sort();
    assert_eq!(ids, vec![1, 2]);
    assert_eq!(tabela.remover(ProcessId(1)).unwrap().x, 42);
    assert!(tabela.get(ProcessId(1)).is_none());
    assert!(tabela.get_mut(ProcessId(9)).is_none());
    assert!(tabela.remover(ProcessId(9)).is_none());
    assert_eq!(tabela.len(), 1);
    assert!(!tabela.vazia());
}

#[test]
fn pid_duplicado_nao_substitui_contexto_original() {
    let mut tabela = TabelaProcessos::nova();
    tabela.inserir(processo(1, 3)).unwrap();
    tabela.get_mut(ProcessId(1)).unwrap().pc = 7;
    assert!(matches!(
        tabela.inserir(processo(1, 99)),
        Err(ErroEp::EstadoInvalido(_))
    ));
    assert_eq!(tabela.len(), 1);
    let original = tabela.get(ProcessId(1)).unwrap();
    assert_eq!(
        (original.prioridade, original.creditos, original.pc),
        (3, 3, 7)
    );
}

#[test]
fn desconto_e_unitario_saturado_e_preserva_contexto() {
    use ep1_escalonador::politica::descontar_credito;
    let mut tabela = TabelaProcessos::nova();
    let mut bcp = processo(1, 2);
    bcp.pc = 7;
    bcp.x = 42;
    bcp.y = -3;
    tabela.inserir(bcp).unwrap();
    for esperado in [1, 0, 0] {
        descontar_credito(ProcessId(1), &mut tabela).unwrap();
        let bcp = tabela.get(ProcessId(1)).unwrap();
        assert_eq!(
            (bcp.creditos, bcp.prioridade, bcp.pc, bcp.x, bcp.y),
            (esperado, 2, 7, 42, -3)
        );
    }
    let antes = format!("{tabela:?}");
    assert!(matches!(
        descontar_credito(ProcessId(99), &mut tabela),
        Err(ErroEp::EstadoInvalido(_))
    ));
    assert_eq!(format!("{tabela:?}"), antes);
}

#[test]
fn credito_de_bloqueado_impede_redistribuicao() {
    use ep1_escalonador::{modelos::EstadoProcesso, politica::redistribuir_se_necessario};
    let mut tabela = TabelaProcessos::nova();
    let mut pronto = processo(1, 5);
    pronto.creditos = 0;
    let mut bloqueado = processo(2, 3);
    bloqueado.estado = EstadoProcesso::Bloqueado;
    tabela.inserir(pronto).unwrap();
    tabela.inserir(bloqueado).unwrap();
    let antes = format!("{tabela:?}");
    assert!(!redistribuir_se_necessario(&mut tabela).unwrap());
    assert_eq!(format!("{tabela:?}"), antes);
}

#[test]
fn redistribuicao_restaura_prioridades_de_todos_os_ativos() {
    use ep1_escalonador::{modelos::EstadoProcesso, politica::redistribuir_se_necessario};
    let mut tabela = TabelaProcessos::nova();
    for (pid, prioridade, estado) in [
        (1, 5, EstadoProcesso::Pronto),
        (2, 3, EstadoProcesso::Bloqueado),
        (3, 0, EstadoProcesso::Executando),
    ] {
        let mut bcp = processo(pid, prioridade);
        bcp.creditos = 0;
        bcp.estado = estado;
        bcp.espera = 1;
        tabela.inserir(bcp).unwrap();
    }
    assert!(redistribuir_se_necessario(&mut tabela).unwrap());
    assert_eq!(tabela.get(ProcessId(1)).unwrap().creditos, 5);
    let bloqueado = tabela.get(ProcessId(2)).unwrap();
    assert_eq!(
        (bloqueado.creditos, bloqueado.estado, bloqueado.espera),
        (3, EstadoProcesso::Bloqueado, 1)
    );
    assert_eq!(tabela.get(ProcessId(3)).unwrap().creditos, 0);
    assert!(!redistribuir_se_necessario(&mut tabela).unwrap());
}

#[test]
fn vazia_e_prioridades_zero_nao_redistribuem() {
    use ep1_escalonador::politica::redistribuir_se_necessario;
    let mut tabela = TabelaProcessos::nova();
    assert!(!redistribuir_se_necessario(&mut tabela).unwrap());
    tabela.inserir(processo(1, 0)).unwrap();
    assert!(!redistribuir_se_necessario(&mut tabela).unwrap());
    assert!(!redistribuir_se_necessario(&mut tabela).unwrap());
}

#[test]
fn terminado_removido_nao_impede_redistribuicao() {
    use ep1_escalonador::politica::redistribuir_se_necessario;
    let mut tabela = TabelaProcessos::nova();
    let mut ativo = processo(1, 4);
    ativo.creditos = 0;
    tabela.inserir(ativo).unwrap();
    tabela.inserir(processo(2, 9)).unwrap();
    tabela.remover(ProcessId(2)).unwrap();
    assert!(redistribuir_se_necessario(&mut tabela).unwrap());
    assert_eq!(tabela.get(ProcessId(1)).unwrap().creditos, 4);
    assert_eq!(tabela.len(), 1);
}
