use ep1_escalonador::{
    erros::ErroEp,
    estatisticas::{calcular_resumo, Estatisticas},
    modelos::{MotivoParada, ProcessId, ResultadoQuantum, Resumo},
    registro::{EventoLog, RegistroLog},
};
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};
static SEQUENCIA: AtomicUsize = AtomicUsize::new(0);
struct Temporario(PathBuf);
impl Temporario {
    fn novo() -> Self {
        let p = std::env::temp_dir().join(format!(
            "ep1-logs-{}-{}",
            std::process::id(),
            SEQUENCIA.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&p).unwrap();
        Self(p)
    }
}
impl Drop for Temporario {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}
fn resumo() -> Resumo {
    Resumo {
        quantum: 3,
        total_processos: 2,
        total_instrucoes: 15,
        total_despachos: 6,
        media_trocas: 3.0,
        media_instrucoes: 2.5,
    }
}
#[test]
fn formato_dos_eventos_obedece_contrato_utf8_e_singular() {
    let casos = [
        (EventoLog::Carregando("AÇÃO"), "Carregando AÇÃO"),
        (EventoLog::Executando("AÇÃO"), "Executando AÇÃO"),
        (EventoLog::EntradaSaida("AÇÃO"), "E/S iniciada em AÇÃO"),
        (
            EventoLog::Interrompendo {
                nome: "AÇÃO",
                instrucoes: 1,
            },
            "Interrompendo AÇÃO após 1 instrução",
        ),
        (
            EventoLog::Interrompendo {
                nome: "AÇÃO",
                instrucoes: 5,
            },
            "Interrompendo AÇÃO após 5 instruções",
        ),
        (
            EventoLog::Terminado {
                nome: "AÇÃO",
                x: -2,
                y: 10,
            },
            "AÇÃO terminado. X=-2. Y=10",
        ),
    ];
    for (evento, esperado) in casos {
        assert_eq!(evento.to_string(), esperado);
    }
}
#[test]
fn cria_diretorio_e_publica_evento_antes_de_finalizar() {
    let tmp = Temporario::novo();
    let dir = tmp.0.join("novo");
    let mut log = RegistroLog::criar(&dir, 3).unwrap();
    log.registrar(EventoLog::Carregando("AÇÃO")).unwrap();
    assert_eq!(
        fs::read_to_string(dir.join("log03.txt")).unwrap(),
        "Carregando AÇÃO\n"
    );
    log.escrever_resumo(&resumo()).unwrap();
    assert_eq!(
        fs::read_to_string(dir.join("log03.txt")).unwrap(),
        "Carregando AÇÃO\nMEDIA DE TROCAS: 3\nMEDIA DE INSTRUCOES: 2.5\nQUANTUM: 3\n"
    );
    log.finalizar().unwrap();
}
#[test]
fn resumo_com_denominador_zero_nao_publica_resultados() {
    let tmp = Temporario::novo();
    let mut log = RegistroLog::criar(&tmp.0, 3).unwrap();
    for (p, d) in [(0, 6), (2, 0)] {
        let mut r = resumo();
        r.total_processos = p;
        r.total_despachos = d;
        assert!(matches!(
            log.escrever_resumo(&r),
            Err(ErroEp::EstadoInvalido(_))
        ));
    }
    assert_eq!(fs::read(tmp.0.join("log03.txt")).unwrap(), b"");
}
#[test]
fn criacao_propaga_erro_de_io() {
    let tmp = Temporario::novo();
    let arquivo = tmp.0.join("arquivo");
    fs::write(&arquivo, b"x").unwrap();
    assert!(matches!(
        RegistroLog::criar(&arquivo, 3),
        Err(ErroEp::Io(_))
    ));
}
#[cfg(target_os = "linux")]
#[test]
fn falha_de_flush_e_propagada() {
    use std::os::unix::fs::symlink;
    let tmp = Temporario::novo();
    symlink("/dev/full", tmp.0.join("log03.txt")).unwrap();
    let mut log = RegistroLog::criar(&tmp.0, 3).unwrap();
    assert!(matches!(
        log.registrar(EventoLog::Executando("P")),
        Err(ErroEp::Io(_))
    ));
    assert!(matches!(log.finalizar(), Err(ErroEp::Io(_))));
}
#[test]
fn calcula_medias_com_divisao_fracionaria() {
    let r = calcular_resumo(3, 2, 15, 6).unwrap();
    assert_eq!(r.quantum, 3);
    assert_eq!(r.total_processos, 2);
    assert_eq!(r.total_instrucoes, 15);
    assert_eq!(r.total_despachos, 6);
    assert_eq!(r.media_trocas, 3.0);
    assert_eq!(r.media_instrucoes, 2.5);
    let r = calcular_resumo(3, 2, 4, 3).unwrap();
    assert_eq!(r.media_trocas, 1.5);
    assert_eq!(r.media_instrucoes, 4.0 / 3.0);
}
#[test]
fn calculo_rejeita_denominadores_zero() {
    for (p, d) in [(0, 6), (2, 0)] {
        assert!(matches!(
            calcular_resumo(3, p, 15, d),
            Err(ErroEp::EstadoInvalido(_))
        ));
    }
}
#[test]
fn calculo_rejeita_quantum_zero() {
    assert!(matches!(
        calcular_resumo(0, 2, 15, 6),
        Err(ErroEp::Entrada(_))
    ));
}
#[test]
fn registra_surtos_com_todos_os_motivos_e_pid_repetido() {
    let mut e = Estatisticas::default();
    for motivo in [MotivoParada::Quantum, MotivoParada::Es, MotivoParada::Saida] {
        e.registrar(&ResultadoQuantum {
            pid: ProcessId(1),
            instrucoes_executadas: 2,
            motivo,
        })
        .unwrap();
    }
    let r = e.resumo(3, 2).unwrap();
    assert_eq!(r.total_despachos, 3);
    assert_eq!(r.total_instrucoes, 6);
    assert_eq!(r.media_trocas, 1.5);
    assert_eq!(r.media_instrucoes, 2.0);
}
#[test]
fn registro_rejeita_surto_sem_instrucao() {
    let mut e = Estatisticas::default();
    assert!(matches!(
        e.registrar(&ResultadoQuantum {
            pid: ProcessId(1),
            instrucoes_executadas: 0,
            motivo: MotivoParada::Quantum,
        }),
        Err(ErroEp::EstadoInvalido(_))
    ));
}

#[test]
fn resumo_de_contadores_vazios_nao_inventa_medias() {
    let e = Estatisticas::default();
    assert!(matches!(e.resumo(3, 2), Err(ErroEp::EstadoInvalido(_))));
}
#[test]
fn resumo_dos_contadores_valida_quantum_e_processos() {
    let mut e = Estatisticas::default();
    e.registrar(&ResultadoQuantum {
        pid: ProcessId(1),
        instrucoes_executadas: 2,
        motivo: MotivoParada::Saida,
    })
    .unwrap();
    assert!(matches!(e.resumo(0, 1), Err(ErroEp::Entrada(_))));
    assert!(matches!(e.resumo(3, 0), Err(ErroEp::EstadoInvalido(_))));
    let r = e.resumo(3, 1).unwrap();
    assert_eq!(r.total_despachos, 1);
    assert_eq!(r.total_instrucoes, 2);
}
#[test]
fn surto_invalido_nao_altera_resumo_anterior() {
    let mut e = Estatisticas::default();
    e.registrar(&ResultadoQuantum {
        pid: ProcessId(1),
        instrucoes_executadas: 2,
        motivo: MotivoParada::Quantum,
    })
    .unwrap();
    assert!(matches!(
        e.registrar(&ResultadoQuantum {
            pid: ProcessId(1),
            instrucoes_executadas: 0,
            motivo: MotivoParada::Es
        }),
        Err(ErroEp::EstadoInvalido(_))
    ));
    let r = e.resumo(3, 1).unwrap();
    assert_eq!(r.total_despachos, 1);
    assert_eq!(r.total_instrucoes, 2);
}
