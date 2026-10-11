use ep1_escalonador::{
    erros::ErroEp,
    interpretador::executar_instrucao,
    modelos::{Bcp, EstadoProcesso, Instrucao, MotivoParada, ProcessId, Programa},
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

// Detecta atribuição ao registrador errado, perda de contexto ou PC avançado incorretamente.
#[test]
fn instrucao_atualiza_so_o_registrador_alvo_e_avanca_pc_uma_vez() {
    for (instrucao, x, y, motivo) in [
        (Instrucao::AtribuirX(-5), -5, 12, None),
        (Instrucao::AtribuirY(99), -8, 99, None),
        (Instrucao::Com, -8, 12, None),
        (Instrucao::EntradaSaida, -8, 12, Some(MotivoParada::Es)),
        (Instrucao::Saida, -8, 12, Some(MotivoParada::Saida)),
    ] {
        let mut bcp = processo(instrucao);
        bcp.espera = 2;
        assert_eq!(executar_instrucao(&mut bcp).unwrap(), motivo);
        assert_eq!(bcp.pc, 1);
        assert_eq!((bcp.x, bcp.y), (x, y));
        conferir_metadados(&bcp, &[instrucao, Instrucao::Saida]);
    }
}

fn conferir_metadados(bcp: &Bcp, instrucoes: &[Instrucao]) {
    assert_eq!(bcp.pid, ProcessId(7));
    assert_eq!(bcp.programa.nome, "CPU");
    assert_eq!(bcp.programa.arquivo, "07.txt");
    assert_eq!(&*bcp.programa.instrucoes, instrucoes);
    assert_eq!(bcp.estado, EstadoProcesso::Executando);
    assert_eq!(bcp.prioridade, 4);
    assert_eq!(bcp.creditos, 3);
    assert_eq!(bcp.espera, 2);
}

// Detecta releitura de E/S em vez de retomada na próxima instrução.
#[test]
fn instrucao_retoma_apos_es_na_instrucao_seguinte() {
    let mut bcp = processo(Instrucao::EntradaSaida);
    assert_eq!(
        executar_instrucao(&mut bcp).unwrap(),
        Some(MotivoParada::Es)
    );
    assert_eq!(bcp.pc, 1);
    assert_eq!(
        executar_instrucao(&mut bcp).unwrap(),
        Some(MotivoParada::Saida)
    );
    assert_eq!(bcp.pc, 2);
}

// Detecta acesso fora do texto ou mutação antes da validação do PC.
#[test]
fn instrucao_rejeita_pc_fora_do_texto_sem_mutacao() {
    for pc in [2, 3, usize::MAX] {
        let mut bcp = processo(Instrucao::AtribuirX(-5));
        bcp.pc = pc;
        bcp.espera = 2;
        assert!(matches!(
            executar_instrucao(&mut bcp),
            Err(ErroEp::EstadoInvalido(_))
        ));
        assert_eq!(bcp.pc, pc);
        assert_eq!((bcp.x, bcp.y), (-8, 12));
        conferir_metadados(&bcp, &[Instrucao::AtribuirX(-5), Instrucao::Saida]);
    }
}

#[test]
fn instrucao_rejeita_segmento_vazio_sem_mutacao() {
    let mut bcp = processo(Instrucao::Com);
    bcp.programa.instrucoes = Box::new([]);
    bcp.espera = 2;
    assert!(matches!(
        executar_instrucao(&mut bcp),
        Err(ErroEp::EstadoInvalido(_))
    ));
    assert_eq!(bcp.pc, 0);
    assert_eq!((bcp.x, bcp.y), (-8, 12));
    conferir_metadados(&bcp, &[]);
}

fn processo_com_programa(instrucoes: &[Instrucao]) -> Bcp {
    let mut bcp = processo(Instrucao::Com);
    bcp.programa.instrucoes = instrucoes.into();
    bcp.x = 0;
    bcp.y = 0;
    bcp.espera = 2;
    bcp
}

// Detecta quantum ultrapassado, contagem errada ou contexto perdido entre chamadas.
#[test]
fn quantum_limita_instrucoes_e_retoma_com_registradores_preservados() {
    let instrucoes = [
        Instrucao::AtribuirX(8),
        Instrucao::Com,
        Instrucao::AtribuirY(10),
        Instrucao::Saida,
    ];
    let mut bcp = processo_com_programa(&instrucoes);
    let primeiro = executar_quantum(&mut bcp, 2).unwrap();
    assert_eq!(primeiro.pid, ProcessId(7));
    assert_eq!(primeiro.instrucoes_executadas, 2);
    assert_eq!(primeiro.motivo, MotivoParada::Quantum);
    assert_eq!(bcp.pc, 2);
    assert_eq!((bcp.x, bcp.y), (8, 0));
    conferir_metadados(&bcp, &instrucoes);
    let segundo = executar_quantum(&mut bcp, 2).unwrap();
    assert_eq!(segundo.pid, ProcessId(7));
    assert_eq!(segundo.instrucoes_executadas, 2);
    assert_eq!(segundo.motivo, MotivoParada::Saida);
    assert_eq!(bcp.pc, 4);
    assert_eq!((bcp.x, bcp.y), (8, 10));
    conferir_metadados(&bcp, &instrucoes);
}

// Detecta E/S ignorada na fronteira ou executada de novo durante a retomada.
#[test]
fn quantum_para_em_es_antes_ou_na_fronteira_e_retoma_na_seguinte() {
    let instrucoes = [
        Instrucao::Com,
        Instrucao::EntradaSaida,
        Instrucao::AtribuirX(9),
        Instrucao::Saida,
    ];
    for quantum in [2, 4] {
        let mut bcp = processo_com_programa(&instrucoes);
        let primeiro = executar_quantum(&mut bcp, quantum).unwrap();
        assert_eq!(primeiro.pid, ProcessId(7));
        assert_eq!(primeiro.instrucoes_executadas, 2);
        assert_eq!(primeiro.motivo, MotivoParada::Es);
        assert_eq!(bcp.pc, 2);
        assert_eq!((bcp.x, bcp.y), (0, 0));
        conferir_metadados(&bcp, &instrucoes);
        let segundo = executar_quantum(&mut bcp, quantum).unwrap();
        assert_eq!(segundo.pid, ProcessId(7));
        assert_eq!(segundo.instrucoes_executadas, 2);
        assert_eq!(segundo.motivo, MotivoParada::Saida);
        assert_eq!(bcp.pc, 4);
        assert_eq!((bcp.x, bcp.y), (9, 0));
        conferir_metadados(&bcp, &instrucoes);
    }
}

// Detecta classificação como Quantum quando SAIDA ocupa a última unidade.
#[test]
fn quantum_para_em_saida_antes_ou_na_fronteira() {
    let instrucoes = [Instrucao::Com, Instrucao::Saida];
    for quantum in [2, 4] {
        let mut bcp = processo_com_programa(&instrucoes);
        let resultado = executar_quantum(&mut bcp, quantum).unwrap();
        assert_eq!(resultado.pid, ProcessId(7));
        assert_eq!(resultado.instrucoes_executadas, 2);
        assert_eq!(resultado.motivo, MotivoParada::Saida);
        assert_eq!(bcp.pc, 2);
        assert_eq!((bcp.x, bcp.y), (0, 0));
        conferir_metadados(&bcp, &instrucoes);
    }
}

// Detecta contagem zero de E/S ou SAIDA e reservas proporcionais ao quantum.
#[test]
fn quantum_conta_es_e_saida_isoladas_inclusive_com_limite_maximo() {
    for (instrucao, motivo) in [
        (Instrucao::EntradaSaida, MotivoParada::Es),
        (Instrucao::Saida, MotivoParada::Saida),
    ] {
        for quantum in [1, 4, usize::MAX] {
            let mut bcp = processo_com_programa(&[instrucao]);
            let resultado = executar_quantum(&mut bcp, quantum).unwrap();
            assert_eq!(resultado.pid, ProcessId(7));
            assert_eq!(resultado.instrucoes_executadas, 1);
            assert_eq!(resultado.motivo, motivo);
            assert_eq!(bcp.pc, 1);
            assert_eq!((bcp.x, bcp.y), (0, 0));
            conferir_metadados(&bcp, &[instrucao]);
        }
    }
}

#[test]
fn quantum_um_impede_execucao_da_segunda_instrucao() {
    let instrucoes = [Instrucao::Com, Instrucao::AtribuirX(9), Instrucao::Saida];
    let mut bcp = processo_com_programa(&instrucoes);
    let resultado = executar_quantum(&mut bcp, 1).unwrap();
    assert_eq!(resultado.pid, ProcessId(7));
    assert_eq!(resultado.instrucoes_executadas, 1);
    assert_eq!(resultado.motivo, MotivoParada::Quantum);
    assert_eq!(bcp.pc, 1);
    assert_eq!((bcp.x, bcp.y), (0, 0));
    conferir_metadados(&bcp, &instrucoes);
}

// Detecta mutação ou leitura de PC antes da validação do quantum zero.
#[test]
fn quantum_zero_rejeita_entrada_sem_mutacao_mesmo_com_pc_invalido() {
    let instrucoes = [Instrucao::AtribuirX(9), Instrucao::Saida];
    for pc in [0, 2, usize::MAX] {
        let mut bcp = processo_com_programa(&instrucoes);
        bcp.pc = pc;
        assert!(matches!(
            executar_quantum(&mut bcp, 0),
            Err(ErroEp::Entrada(_))
        ));
        assert_eq!(bcp.pc, pc);
        assert_eq!((bcp.x, bcp.y), (0, 0));
        conferir_metadados(&bcp, &instrucoes);
    }
}

#[test]
fn quantum_rejeita_pc_inicial_fora_do_texto_sem_mutacao() {
    let instrucoes = [Instrucao::AtribuirX(9), Instrucao::Saida];
    for pc in [2, 3, usize::MAX] {
        let mut bcp = processo_com_programa(&instrucoes);
        bcp.pc = pc;
        assert!(matches!(
            executar_quantum(&mut bcp, 2),
            Err(ErroEp::EstadoInvalido(_))
        ));
        assert_eq!(bcp.pc, pc);
        assert_eq!((bcp.x, bcp.y), (0, 0));
        conferir_metadados(&bcp, &instrucoes);
    }
}

#[test]
fn quantum_rejeita_segmento_vazio_sem_mutacao() {
    let mut bcp = processo_com_programa(&[]);
    assert!(matches!(
        executar_quantum(&mut bcp, 2),
        Err(ErroEp::EstadoInvalido(_))
    ));
    assert_eq!(bcp.pc, 0);
    assert_eq!((bcp.x, bcp.y), (0, 0));
    conferir_metadados(&bcp, &[]);
}

// Detecta erro engolido ao esgotar o texto ou rollback do progresso válido.
#[test]
fn quantum_propaga_erro_ao_esgotar_texto_preservando_progresso_valido() {
    let instrucoes = [Instrucao::AtribuirX(8), Instrucao::Com];
    let mut bcp = processo_com_programa(&instrucoes);
    assert!(matches!(
        executar_quantum(&mut bcp, 4),
        Err(ErroEp::EstadoInvalido(_))
    ));
    assert_eq!(bcp.pc, 2);
    assert_eq!((bcp.x, bcp.y), (8, 0));
    conferir_metadados(&bcp, &instrucoes);
}

#[test]
fn simulacao_rejeita_contexto_executando_antes_de_produzir_eventos() {
    let diretorio = std::env::temp_dir().join(format!("ep1-cpu-{}", std::process::id()));
    std::fs::create_dir(&diretorio).unwrap();
    let resultado = || {
        let mut registro = RegistroLog::criar(&diretorio, 3).unwrap();
        assert!(matches!(
            simular(vec![processo(Instrucao::Com)], 3, &mut registro),
            Err(ErroEp::EstadoInvalido(_))
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
