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
fn contratos_pendentes_retornam_erro_sem_alterar_filas_ou_contexto() {
    use ep1_escalonador::{
        filas::{FilaBloqueados, FilaProntos},
        politica::{descontar_credito, redistribuir_se_necessario},
    };
    let mut tabela = TabelaProcessos::nova();
    tabela.inserir(processo(1, 3)).unwrap();
    let mut prontos = FilaProntos::nova();
    let mut bloqueados = FilaBloqueados::nova();
    let antes = format!("{tabela:?}");
    fn pendente<T>(resultado: Result<T, ErroEp>) {
        assert!(matches!(resultado, Err(ErroEp::NaoImplementado(regra)) if !regra.is_empty()));
    }
    pendente(prontos.ordenar_inicial(&tabela));
    pendente(prontos.selecionar(&tabela));
    pendente(prontos.reinserir_apos_quantum(ProcessId(1), &tabela));
    pendente(prontos.reinserir_apos_es(ProcessId(1), &tabela));
    pendente(prontos.reordenar_apos_redistribuicao(&tabela));
    pendente(bloqueados.bloquear(ProcessId(1), &mut tabela));
    pendente(bloqueados.avancar_esperas(&[ProcessId(1)], &mut tabela));
    pendente(descontar_credito(ProcessId(1), &mut tabela));
    pendente(redistribuir_se_necessario(&mut tabela));
    assert!(prontos.ids().is_empty());
    assert!(bloqueados.ids().is_empty());
    assert_eq!(format!("{tabela:?}"), antes);
}
