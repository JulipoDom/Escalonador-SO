use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);

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

    fn sem_resultados(&self) {
        assert_eq!(fs::read_dir(&self.0).unwrap().count(), 1);
        assert_eq!(fs::read_dir(self.0.join("programas")).unwrap().count(), 3);
        assert_eq!(
            fs::read_to_string(self.0.join("programas/quantum.txt")).unwrap(),
            "3\n"
        );
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

#[test]
fn rejeita_qualquer_argumento_incluindo_antigas_opcoes() {
    let entradas = Entradas::novas();
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
        entradas.sem_resultados();
    }
}

#[test]
fn entradas_validas_chegam_ao_stub_sem_publicar_arquivos() {
    let entradas = Entradas::novas();
    let mensagem = erro(&entradas.executar(&[]));
    assert!(mensagem.contains("Não implementado"), "{mensagem}");
    assert!(mensagem.contains("simulação"), "{mensagem}");
    entradas.sem_resultados();
}

#[test]
fn programa_invalido_falha_com_contexto_antes_da_simulacao() {
    let entradas = Entradas::novas();
    fs::write(
        entradas.0.join("programas/01.txt"),
        "PEQUENO\nINVALIDA\nSAIDA\n",
    )
    .unwrap();
    let mensagem = erro(&entradas.executar(&[]));
    assert!(mensagem.contains("Entrada inválida"), "{mensagem}");
    assert!(mensagem.contains("01.txt:2"), "{mensagem}");
    assert!(!mensagem.contains("Não implementado"), "{mensagem}");
    entradas.sem_resultados();
}

#[test]
fn quantum_invalido_falha_com_contexto_antes_da_simulacao() {
    let entradas = Entradas::novas();
    fs::write(entradas.0.join("programas/quantum.txt"), "0\n").unwrap();
    let mensagem = erro(&entradas.executar(&[]));
    assert!(mensagem.contains("Entrada inválida"), "{mensagem}");
    assert!(mensagem.contains("quantum.txt:1"), "{mensagem}");
    assert!(!mensagem.contains("Não implementado"), "{mensagem}");
    assert_eq!(fs::read_dir(&entradas.0).unwrap().count(), 1);
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
