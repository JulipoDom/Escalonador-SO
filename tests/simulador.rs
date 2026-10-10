use ep1_escalonador::{
    erros::ErroEp,
    interpretador::executar_instrucao,
    modelos::{Bcp, EstadoProcesso, Instrucao, ProcessId, Programa},
    registro::RegistroLog,
    simulador::{executar_quantum, simular},
};

fn processo(instrucao: Instrucao) -> Bcp {
    let mut bcp = Bcp::novo(
        ProcessId(7),
        Programa {
            nome: "CPU".into(),
            arquivo: "07.txt".into(),
            instrucoes: vec![instrucao, Instrucao::Saida].into_boxed_slice(),
        },
        4,
    );
    bcp.estado = EstadoProcesso::Executando;
    bcp.creditos = 3;
    bcp.x = -8;
    bcp.y = 12;
    bcp
}

fn conferir_contexto(bcp: &Bcp, instrucao: Instrucao) {
    assert_eq!(bcp.pid, ProcessId(7));
    assert_eq!(bcp.programa.nome, "CPU");
    assert_eq!(bcp.programa.arquivo, "07.txt");
    assert_eq!(&*bcp.programa.instrucoes, &[instrucao, Instrucao::Saida]);
    assert_eq!(bcp.pc, 0);
    assert_eq!(bcp.estado, EstadoProcesso::Executando);
    assert_eq!(bcp.prioridade, 4);
    assert_eq!(bcp.creditos, 3);
    assert_eq!((bcp.x, bcp.y, bcp.espera), (-8, 12, 0));
}

#[test]
fn instrucao_pendente_nao_executa_nem_modifica_contexto() {
    for instrucao in [
        Instrucao::AtribuirX(99),
        Instrucao::AtribuirY(99),
        Instrucao::Com,
        Instrucao::EntradaSaida,
        Instrucao::Saida,
    ] {
        let mut bcp = processo(instrucao);
        assert!(matches!(
            executar_instrucao(&mut bcp),
            Err(ErroEp::NaoImplementado(_))
        ));
        conferir_contexto(&bcp, instrucao);
    }
}

#[test]
fn quantum_pendente_nao_consome_instrucoes_ou_creditos() {
    for quantum in [0, 1, 3] {
        let mut bcp = processo(Instrucao::EntradaSaida);
        assert!(matches!(
            executar_quantum(&mut bcp, quantum),
            Err(ErroEp::NaoImplementado(_))
        ));
        conferir_contexto(&bcp, Instrucao::EntradaSaida);
    }
}

#[test]
fn simulacao_pendente_nao_produz_eventos_ou_resumo() {
    let diretorio = std::env::temp_dir().join(format!("ep1-cpu-{}", std::process::id()));
    std::fs::create_dir(&diretorio).unwrap();
    let resultado = || {
        let mut registro = RegistroLog::criar(&diretorio, 3).unwrap();
        assert!(matches!(
            simular(vec![processo(Instrucao::Com)], 3, &mut registro),
            Err(ErroEp::NaoImplementado(_))
        ));
        registro.finalizar().unwrap();
        assert_eq!(std::fs::read(diretorio.join("log03.txt")).unwrap(), b"");
    };
    // Limpeza também quando uma asserção falha.
    let resultado = std::panic::catch_unwind(resultado);
    std::fs::remove_dir_all(diretorio).unwrap();
    if let Err(erro) = resultado {
        std::panic::resume_unwind(erro);
    }
}
