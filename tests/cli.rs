use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "ep-cli-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&p).unwrap();
        Self(p)
    }
    fn entradas(&self, nome: impl AsRef<Path>) -> PathBuf {
        let p = self.0.join(nome);
        fs::create_dir_all(&p).unwrap();
        fs::write(p.join("01.txt"), "PEQUENO\nX=8\nSAIDA\n").unwrap();
        fs::write(p.join("prioridades.txt"), "4\n").unwrap();
        fs::write(p.join("quantum.txt"), "3\n").unwrap();
        p
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn run(cwd: &Path, args: &[&OsStr]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_Escalonador"))
        .current_dir(cwd)
        .args(args)
        .output()
        .unwrap()
}
fn text(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}
#[test]
fn ajuda_nao_precisa_entradas() {
    let t = Temp::new();
    let o = run(&t.0, &[OsStr::new("--help")]);
    assert!(o.status.success(), "{}", text(&o.stderr));
    for s in [
        "--validar",
        "--programas",
        "--saida",
        "--quanta",
        "logs",
        "tabelas",
        "logXX",
    ] {
        assert!(text(&o.stdout).contains(s), "{s}");
    }
    assert!(o.stderr.is_empty());
}
#[test]
fn valida_oficiais_e_total() {
    let p = Path::new(env!("CARGO_MANIFEST_DIR")).join("programas");
    let t = Temp::new();
    let o = run(
        &t.0,
        &[
            OsStr::new("--validar"),
            OsStr::new("--programas"),
            p.as_os_str(),
        ],
    );
    assert!(o.status.success(), "{}", text(&o.stderr));
    let s = text(&o.stdout);
    for v in [
        "prioridade",
        "instruções",
        "quantum: 3",
        "139",
        "10 processos",
    ] {
        assert!(s.contains(v), "{s}");
    }
    assert!(o.stderr.is_empty());
}
#[test]
fn rejeita_argumentos_invalidos() {
    let t = Temp::new();
    for a in [
        vec!["--desconhecida"],
        vec!["--programas"],
        vec!["--saida", "--validar"],
        vec!["--quanta"],
        vec!["--quanta", "0"],
        vec!["--quanta", "x"],
        vec!["--quanta", "-1"],
        vec!["--validar", "--quanta", "2"],
        vec!["--programas", "a", "--programas", "b"],
        vec!["--saida", "a", "--saida", "b"],
        vec!["--quanta", "1", "--quanta", "2"],
    ] {
        let args: Vec<_> = a.iter().map(OsStr::new).collect();
        let o = run(&t.0, &args);
        assert!(!o.status.success(), "{a:?}");
        assert!(
            text(&o.stderr).contains("Entrada inválida"),
            "{a:?}: {}",
            text(&o.stderr)
        );
        assert!(o.stdout.is_empty());
    }
}
#[test]
fn fallback_cwd_e_ausencia_com_orientacao() {
    let t = Temp::new();
    let o = run(&t.0, &[OsStr::new("--validar")]);
    assert!(!o.status.success());
    assert!(text(&o.stderr).contains("--programas"));
    t.entradas("programas");
    let o = run(&t.0, &[OsStr::new("--validar")]);
    assert!(o.status.success(), "{}", text(&o.stderr));
    assert!(text(&o.stdout).contains("PEQUENO"));
}
#[test]
fn modos_pendentes_nao_criam_resultados_nem_alteram_quantum() {
    let t = Temp::new();
    let p = t.entradas("programas");
    for args in [
        vec![],
        vec!["--quanta", "1", "7"],
        vec!["--saida", "personalizado"],
        vec!["--quanta", "1", "--saida", "personalizado"],
    ] {
        let args: Vec<_> = args.iter().map(OsStr::new).collect();
        let o = run(&t.0, &args);
        assert!(!o.status.success());
        assert!(
            text(&o.stderr).contains("Não implementado"),
            "{}",
            text(&o.stderr)
        );
        assert!(o.stdout.is_empty());
        assert_eq!(fs::read_dir(&t.0).unwrap().count(), 1);
        assert_eq!(fs::read_to_string(p.join("quantum.txt")).unwrap(), "3\n");
    }
}
#[test]
fn entradas_ao_lado_do_executavel_precedem_cwd() {
    let t = Temp::new();
    let b = t.0.join("bin");
    fs::create_dir(&b).unwrap();
    let exe = b.join("Escalonador");
    fs::copy(env!("CARGO_BIN_EXE_Escalonador"), &exe).unwrap();
    let p = t.entradas("bin/programas");
    t.entradas("programas");
    fs::write(p.join("01.txt"), "VIZINHO\nSAIDA\n").unwrap();
    let o = Command::new(exe)
        .current_dir(&t.0)
        .arg("--validar")
        .output()
        .unwrap();
    assert!(o.status.success(), "{}", text(&o.stderr));
    assert!(text(&o.stdout).contains("VIZINHO"));
    assert!(!text(&o.stdout).contains("PEQUENO"));
}
#[cfg(unix)]
#[test]
fn preserva_caminhos_nao_utf8() {
    use std::os::unix::ffi::OsStringExt;
    let t = Temp::new();
    let p = t.entradas(std::ffi::OsString::from_vec(b"entrada-\xff".to_vec()));
    let o = run(
        &t.0,
        &[
            OsStr::new("--programas"),
            p.as_os_str(),
            OsStr::new("--validar"),
        ],
    );
    assert!(o.status.success(), "{}", text(&o.stderr));
    assert!(text(&o.stdout).contains("PEQUENO"));
}
