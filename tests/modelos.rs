use ep1_escalonador::erros::ErroEp;
use ep1_escalonador::modelos::*;
use std::{error::Error, io};

fn programa() -> Programa {
    Programa {
        nome: "TESTE".into(),
        arquivo: "01.txt".into(),
        instrucoes: vec![Instrucao::AtribuirX(-7), Instrucao::Saida].into_boxed_slice(),
    }
}

// Captura PC/registradores/espera não zerados ou créditos diferentes da prioridade.
#[test]
fn contexto_novo_preserva_programa_e_inicia_pronto() {
    for prioridade in [0, 12, u32::MAX] {
        let bcp = Bcp::novo(ProcessId(3), programa(), prioridade);
        assert_eq!(bcp.pid, ProcessId(3));
        assert_eq!((bcp.pc, bcp.x, bcp.y, bcp.espera), (0, 0, 0, 0));
        assert_eq!(bcp.estado, EstadoProcesso::Pronto);
        assert_eq!((bcp.prioridade, bcp.creditos), (prioridade, prioridade));
        assert_eq!(bcp.programa.nome, "TESTE");
        assert_eq!(bcp.programa.arquivo, "01.txt");
        assert_eq!(
            &*bcp.programa.instrucoes,
            &[Instrucao::AtribuirX(-7), Instrucao::Saida]
        );
    }
}

// Captura contexto compartilhado entre processos criados separadamente.
#[test]
fn alterar_um_contexto_preserva_o_outro() {
    let mut primeiro = Bcp::novo(ProcessId(1), programa(), 3);
    let segundo = Bcp::novo(ProcessId(2), programa(), 8);
    primeiro.pc = 1;
    primeiro.x = -7;
    primeiro.y = 22;
    primeiro.estado = EstadoProcesso::Bloqueado;
    primeiro.creditos = 2;
    primeiro.espera = 2;
    assert_eq!(
        (segundo.pc, segundo.x, segundo.y, segundo.espera),
        (0, 0, 0, 0)
    );
    assert_eq!(
        (segundo.estado, segundo.creditos),
        (EstadoProcesso::Pronto, 8)
    );
    assert_eq!(primeiro.programa.instrucoes[0], Instrucao::AtribuirX(-7));
}

// Captura perda do erro original ao converter e formatar E/S.
#[test]
fn erro_io_preserva_causa_e_mensagem() {
    let erro = ErroEp::from(io::Error::new(
        io::ErrorKind::PermissionDenied,
        "acesso negado ao arquivo",
    ));
    assert!(matches!(&erro, ErroEp::Io(causa) if causa.kind() == io::ErrorKind::PermissionDenied));
    assert!(erro.to_string().contains("acesso negado ao arquivo"));
    assert_eq!(
        erro.source().unwrap().to_string(),
        "acesso negado ao arquivo"
    );
}

// Captura mensagens vazias ou perda do contexto fornecido pelo componente.
#[test]
fn erros_de_dominio_exibem_contexto_sem_causa_artificial() {
    for (erro, contexto) in [
        (
            ErroEp::Entrada("01.txt:3: instrução inválida".into()),
            "01.txt:3",
        ),
        (
            ErroEp::EstadoInvalido("PID 9 ausente".into()),
            "PID 9 ausente",
        ),
        (
            ErroEp::NaoImplementado("redistribuição de créditos"),
            "redistribuição de créditos",
        ),
    ] {
        assert!(erro.to_string().contains(contexto));
        assert!(erro.source().is_none());
    }
}
