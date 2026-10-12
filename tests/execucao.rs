use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);
const LOG_PEQUENO_Q3: &str = "Carregando PEQUENO\nExecutando PEQUENO\nPEQUENO terminado. X=8. Y=0\nMEDIA DE TROCAS: 1\nMEDIA DE INSTRUCOES: 2\nQUANTUM: 3\n";
const LOG_ANTERIOR: &[u8] = b"log anterior\n";

struct Entradas(PathBuf);

impl Entradas {
    fn novas() -> Self {
        let raiz = std::env::temp_dir().join(format!(
            "ep-execucao-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let programas = raiz.join("programas");
        fs::create_dir_all(&programas).unwrap();
        fs::write(programas.join("01.txt"), "PEQUENO\nX=8\nSAIDA\n").unwrap();
        fs::write(programas.join("prioridades.txt"), "4\n").unwrap();
        fs::write(programas.join("quantum.txt"), "3\n").unwrap();
        Self(raiz)
    }

    fn executar(&self, args: &[&str]) -> Output {
        executar(&self.0, args)
    }

    fn criar_log_anterior(&self) {
        fs::write(self.0.join("log03.txt"), LOG_ANTERIOR).unwrap();
    }

    fn log_anterior_preservado(&self) {
        assert_eq!(fs::read_dir(&self.0).unwrap().count(), 2);
        assert_eq!(fs::read(self.0.join("log03.txt")).unwrap(), LOG_ANTERIOR);
        assert_eq!(fs::read_dir(self.0.join("programas")).unwrap().count(), 3);
    }

    fn log(&self, nome: &str) -> String {
        fs::read_to_string(self.0.join(nome)).unwrap()
    }
}

impl Drop for Entradas {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn executar(cwd: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_Escalonador"))
        .current_dir(cwd)
        .args(args)
        .output()
        .unwrap()
}

fn erro(output: &Output) -> String {
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    String::from_utf8(output.stderr.clone()).unwrap()
}

fn sucesso(output: &Output) {
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

#[test]
fn rejeita_qualquer_argumento_incluindo_antigas_opcoes() {
    let entradas = Entradas::novas();
    entradas.criar_log_anterior();
    for args in [
        vec!["--help"],
        vec!["--validar"],
        vec!["--programas", "programas"],
        vec!["--saida", "resultados"],
        vec!["--quanta", "3"],
        vec!["inesperado"],
    ] {
        let output = entradas.executar(&args);
        let mensagem = erro(&output);
        assert!(
            mensagem.contains("Entrada inválida"),
            "{args:?}: {mensagem}"
        );
        entradas.log_anterior_preservado();
    }
}

#[test]
fn entradas_validas_executam_e_publicam_log_real_no_cwd() {
    let entradas = Entradas::novas();
    sucesso(&entradas.executar(&[]));
    assert_eq!(entradas.log("log03.txt"), LOG_PEQUENO_Q3);
    assert_eq!(fs::read_dir(&entradas.0).unwrap().count(), 2);
    assert_eq!(
        fs::read_to_string(entradas.0.join("programas/01.txt")).unwrap(),
        "PEQUENO\nX=8\nSAIDA\n"
    );
}

#[test]
fn quantum_um_despacha_duas_vezes_e_publica_apenas_log01() {
    let entradas = Entradas::novas();
    fs::write(entradas.0.join("programas/quantum.txt"), "1\n").unwrap();
    sucesso(&entradas.executar(&[]));
    assert_eq!(
        entradas.log("log01.txt"),
        "Carregando PEQUENO\nExecutando PEQUENO\nInterrompendo PEQUENO após 1 instrução\nExecutando PEQUENO\nPEQUENO terminado. X=8. Y=0\nMEDIA DE TROCAS: 2\nMEDIA DE INSTRUCOES: 1\nQUANTUM: 1\n"
    );
    assert!(!entradas.0.join("log03.txt").exists());
    assert_eq!(fs::read_dir(&entradas.0).unwrap().count(), 2);
}

#[test]
fn retoma_apos_es_e_publica_registradores_e_medias_reais() {
    let entradas = Entradas::novas();
    fs::write(
        entradas.0.join("programas/01.txt"),
        "PEQUENO\nCOM\nE/S\nX=7\nSAIDA\n",
    )
    .unwrap();
    fs::write(entradas.0.join("programas/quantum.txt"), "2\n").unwrap();
    sucesso(&entradas.executar(&[]));
    assert_eq!(
        entradas.log("log02.txt"),
        "Carregando PEQUENO\nExecutando PEQUENO\nE/S iniciada em PEQUENO\nInterrompendo PEQUENO após 2 instruções\nExecutando PEQUENO\nPEQUENO terminado. X=7. Y=0\nMEDIA DE TROCAS: 2\nMEDIA DE INSTRUCOES: 2\nQUANTUM: 2\n"
    );
    assert_eq!(fs::read_dir(&entradas.0).unwrap().count(), 2);
}

#[test]
fn execucao_valida_substitui_somente_log_do_quantum_lido() {
    let entradas = Entradas::novas();
    entradas.criar_log_anterior();
    fs::write(entradas.0.join("log01.txt"), b"outro quantum\n").unwrap();
    sucesso(&entradas.executar(&[]));
    assert_eq!(entradas.log("log03.txt"), LOG_PEQUENO_Q3);
    assert_eq!(
        fs::read(entradas.0.join("log01.txt")).unwrap(),
        b"outro quantum\n"
    );
    assert_eq!(fs::read_dir(&entradas.0).unwrap().count(), 3);
}

#[test]
fn falha_de_criacao_do_log_retorna_erro_de_es() {
    let entradas = Entradas::novas();
    fs::create_dir(entradas.0.join("log03.txt")).unwrap();
    let mensagem = erro(&entradas.executar(&[]));
    assert!(mensagem.contains("Erro de E/S"), "{mensagem}");
    assert!(entradas.0.join("log03.txt").is_dir());
    assert_eq!(fs::read_dir(&entradas.0).unwrap().count(), 2);
}

#[cfg(target_os = "linux")]
#[test]
fn link_para_dev_full_e_substituido_sem_escrever_no_dispositivo() {
    let entradas = Entradas::novas();
    std::os::unix::fs::symlink("/dev/full", entradas.0.join("log03.txt")).unwrap();
    sucesso(&entradas.executar(&[]));
    assert_eq!(entradas.log("log03.txt"), LOG_PEQUENO_Q3);
    assert!(fs::symlink_metadata(entradas.0.join("log03.txt"))
        .unwrap()
        .is_file());
    assert_eq!(fs::read_dir(&entradas.0).unwrap().count(), 2);
}

#[cfg(unix)]
#[test]
fn log_ligado_a_qualquer_entrada_preserva_todos_os_bytes() {
    for nome in ["01.txt", "prioridades.txt", "quantum.txt"] {
        let entradas = Entradas::novas();
        let alvo = entradas.0.join("programas").join(nome);
        let antes = fs::read(&alvo).unwrap();
        std::os::unix::fs::symlink(&alvo, entradas.0.join("log03.txt")).unwrap();
        sucesso(&entradas.executar(&[]));
        assert_eq!(fs::read(&alvo).unwrap(), antes, "{nome}");
        assert_eq!(entradas.log("log03.txt"), LOG_PEQUENO_Q3);
        assert!(fs::symlink_metadata(entradas.0.join("log03.txt"))
            .unwrap()
            .is_file());
        assert_eq!(fs::read_dir(&entradas.0).unwrap().count(), 2);
    }
}

#[cfg(unix)]
#[test]
fn log_ligado_a_outro_quantum_preserva_o_log_anterior() {
    let entradas = Entradas::novas();
    fs::write(entradas.0.join("log01.txt"), LOG_ANTERIOR).unwrap();
    std::os::unix::fs::symlink("log01.txt", entradas.0.join("log03.txt")).unwrap();
    sucesso(&entradas.executar(&[]));
    assert_eq!(
        fs::read(entradas.0.join("log01.txt")).unwrap(),
        LOG_ANTERIOR
    );
    assert_eq!(entradas.log("log03.txt"), LOG_PEQUENO_Q3);
    assert_eq!(fs::read_dir(&entradas.0).unwrap().count(), 3);
}

#[test]
fn log_com_hardlink_a_entrada_preserva_os_bytes_originais() {
    let entradas = Entradas::novas();
    let alvo = entradas.0.join("programas/01.txt");
    let antes = fs::read(&alvo).unwrap();
    fs::hard_link(&alvo, entradas.0.join("log03.txt")).unwrap();
    sucesso(&entradas.executar(&[]));
    assert_eq!(fs::read(&alvo).unwrap(), antes);
    assert_eq!(entradas.log("log03.txt"), LOG_PEQUENO_Q3);
    assert_eq!(fs::read_dir(&entradas.0).unwrap().count(), 2);
}

#[test]
fn programa_invalido_falha_com_contexto_antes_da_simulacao() {
    let entradas = Entradas::novas();
    entradas.criar_log_anterior();
    fs::write(
        entradas.0.join("programas/01.txt"),
        "PEQUENO\nINVALIDA\nSAIDA\n",
    )
    .unwrap();
    let mensagem = erro(&entradas.executar(&[]));
    assert!(mensagem.contains("Entrada inválida"), "{mensagem}");
    assert!(mensagem.contains("01.txt:2"), "{mensagem}");
    assert!(!mensagem.contains("Não implementado"), "{mensagem}");
    entradas.log_anterior_preservado();
}

#[test]
fn quantum_invalido_falha_com_contexto_antes_da_simulacao() {
    let entradas = Entradas::novas();
    entradas.criar_log_anterior();
    fs::write(entradas.0.join("programas/quantum.txt"), "0\n").unwrap();
    let mensagem = erro(&entradas.executar(&[]));
    assert!(mensagem.contains("Entrada inválida"), "{mensagem}");
    assert!(mensagem.contains("quantum.txt:1"), "{mensagem}");
    assert!(!mensagem.contains("Não implementado"), "{mensagem}");
    entradas.log_anterior_preservado();
}

#[test]
fn prioridades_invalidas_falham_com_contexto_e_preservam_log() {
    let entradas = Entradas::novas();
    entradas.criar_log_anterior();
    fs::write(entradas.0.join("programas/prioridades.txt"), "-1\n").unwrap();
    let mensagem = erro(&entradas.executar(&[]));
    assert!(mensagem.contains("Entrada inválida"), "{mensagem}");
    assert!(mensagem.contains("prioridades.txt:1"), "{mensagem}");
    entradas.log_anterior_preservado();
}

#[test]
fn ausencia_de_programas_no_cwd_informa_o_caminho() {
    let entradas = Entradas::novas();
    fs::remove_dir_all(entradas.0.join("programas")).unwrap();
    let mensagem = erro(&entradas.executar(&[]));
    assert!(mensagem.contains("programas"), "{mensagem}");
    assert!(mensagem.contains("Erro de E/S"), "{mensagem}");
    assert!(!mensagem.contains("Não implementado"), "{mensagem}");
    assert_eq!(fs::read_dir(&entradas.0).unwrap().count(), 0);
}
