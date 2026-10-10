use ep1_escalonador::{
    erros::ErroEp,
    estatisticas::{calcular_resumo, Estatisticas},
    experimentos::executar_experimentos,
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
fn metricas_e_lote_permanecem_pendentes_sem_criar_destino() {
    let tmp = Temporario::novo();
    let destino = tmp.0.join("ausente");
    assert!(matches!(
        calcular_resumo(3, 2, 15, 6),
        Err(ErroEp::NaoImplementado("cálculo das médias D/P e I/D"))
    ));
    let q = ResultadoQuantum {
        pid: ProcessId(1),
        instrucoes_executadas: 3,
        motivo: MotivoParada::Quantum,
    };
    assert!(matches!(
        Estatisticas::default().registrar(&q),
        Err(ErroEp::NaoImplementado(
            "coleta de despachos e instruções executadas"
        ))
    ));
    assert!(matches!(
        executar_experimentos(&tmp.0, &[1, 3], &destino),
        Err(ErroEp::NaoImplementado(
            "execuções independentes por quantum e exportação CSV"
        ))
    ));
    assert!(!destino.exists());
}
