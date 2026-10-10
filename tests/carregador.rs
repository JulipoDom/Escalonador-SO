use ep1_escalonador::{
    carregador::{carregar_programas, ler_quantum},
    modelos::{Instrucao, ProcessId},
};
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Entrada(PathBuf);
impl Entrada {
    fn nova() -> Self {
        let p = std::env::temp_dir().join(format!(
            "ep1-parser-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&p).unwrap();
        Self(p)
    }
    fn gravar(&self, nome: &str, conteudo: &str) {
        fs::write(self.0.join(nome), conteudo).unwrap();
    }
}
impl Drop for Entrada {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
#[test]
fn oficiais_preservam_ordem_prioridades_e_contagens() {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("programas");
    let bcps = carregar_programas(&p).unwrap();
    assert_eq!(bcps.len(), 10);
    assert_eq!(
        bcps.iter()
            .map(|b| b.programa.instrucoes.len())
            .sum::<usize>(),
        139
    );
    assert_eq!(
        bcps.iter().map(|b| b.prioridade).collect::<Vec<_>>(),
        [1, 3, 2, 10, 4, 6, 12, 8, 8, 4]
    );
    assert_eq!(bcps.iter().map(|b| b.creditos).sum::<u32>(), 58);
    assert_eq!(ler_quantum(&p).unwrap(), 3);
}
#[test]
fn seleciona_apenas_arquivos_ascii_e_ordena_antes_de_associar() {
    let e = Entrada::nova();
    e.gravar(
        "09.txt",
        "B\r\nX=-9223372036854775808\r\nY=9223372036854775807\r\nSAIDA\r\n",
    );
    e.gravar("02.txt", "A\nSAIDA\n");
    e.gravar("2.txt", "inválido");
    e.gravar("１２.txt", "inválido");
    fs::create_dir(e.0.join("01.txt")).unwrap();
    e.gravar("prioridades.txt", "0\r\n4294967295\r\n");
    let b = carregar_programas(&e.0).unwrap();
    assert_eq!(
        b.iter()
            .map(|b| (b.pid, b.programa.arquivo.as_str(), b.prioridade))
            .collect::<Vec<_>>(),
        [
            (ProcessId(1), "02.txt", 0),
            (ProcessId(2), "09.txt", u32::MAX)
        ]
    );
    assert_eq!(
        &*b[1].programa.instrucoes,
        &[
            Instrucao::AtribuirX(i64::MIN),
            Instrucao::AtribuirY(i64::MAX),
            Instrucao::Saida
        ]
    );
}
#[test]
fn rejeita_programas_invalidos_com_arquivo_e_linha() {
    let casos = [
        ("\nSAIDA\n", 1),
        ("   \nSAIDA\n", 1),
        ("P\n", 2),
        ("P\nCOM\n", 2),
        ("P\n\nSAIDA\n", 2),
        ("P\nXYZ\nSAIDA\n", 2),
        ("P\nX =1\nSAIDA\n", 2),
        ("P\nX= 1\nSAIDA\n", 2),
        ("P\nY=1 \nSAIDA\n", 2),
        ("P\nX=9223372036854775808\nSAIDA\n", 2),
        ("P\nSAIDA\nCOM\n", 2),
        ("P\nSAIDA\nSAIDA\n", 2),
    ];
    for (texto, linha) in casos {
        let e = Entrada::nova();
        e.gravar("01.txt", texto);
        e.gravar("prioridades.txt", "1\n");
        let erro = carregar_programas(&e.0).unwrap_err().to_string();
        assert!(
            erro.contains(&format!("01.txt:{linha}")),
            "{texto:?}: {erro}"
        );
    }
    let e = Entrada::nova();
    e.gravar("01.txt", &format!("P\n{}SAIDA\n", "COM\n".repeat(21)));
    e.gravar("prioridades.txt", "1");
    assert!(carregar_programas(&e.0)
        .unwrap_err()
        .to_string()
        .contains("01.txt:23"));
}
#[test]
fn rejeita_prioridades_invalidas_ou_quantidade_diferente() {
    for texto in ["", "-1", "4294967296", "1 2", "1\n2", "\n"] {
        let e = Entrada::nova();
        e.gravar("01.txt", "P\nSAIDA");
        e.gravar("prioridades.txt", texto);
        assert!(
            carregar_programas(&e.0)
                .unwrap_err()
                .to_string()
                .contains("prioridades.txt"),
            "{texto:?}"
        );
    }
    let e = Entrada::nova();
    e.gravar("prioridades.txt", "");
    assert!(carregar_programas(&e.0).is_err());
}
#[test]
fn quantum_exige_um_inteiro_positivo_representavel() {
    let e = Entrada::nova();
    for texto in [
        "0",
        "-1",
        "",
        "3\n4",
        "3 4",
        "184467440737095516160",
        "3\n\n",
    ] {
        e.gravar("quantum.txt", texto);
        assert!(
            ler_quantum(&e.0)
                .unwrap_err()
                .to_string()
                .contains("quantum.txt"),
            "{texto:?}"
        );
    }
    e.gravar("quantum.txt", "3\r\n");
    assert_eq!(ler_quantum(&e.0).unwrap(), 3);
}
#[test]
fn erros_de_leitura_incluem_caminho_e_linha_quando_disponivel() {
    let e = Entrada::nova();
    assert!(ler_quantum(&e.0)
        .unwrap_err()
        .to_string()
        .contains("quantum.txt"));
    e.gravar("01.txt", "P\nSAIDA");
    assert!(carregar_programas(&e.0)
        .unwrap_err()
        .to_string()
        .contains("prioridades.txt"));
    e.gravar("prioridades.txt", "1");
    fs::write(e.0.join("01.txt"), b"P\n\xff\n").unwrap();
    assert!(carregar_programas(&e.0)
        .unwrap_err()
        .to_string()
        .contains("01.txt:2"));
}
#[cfg(unix)]
#[test]
fn diretorio_com_nome_nao_utf8_nao_causa_panic() {
    use std::os::unix::ffi::OsStringExt;
    let e = Entrada::nova();
    let p = e.0.join(std::ffi::OsString::from_vec(vec![0xff]));
    fs::create_dir(&p).unwrap();
    fs::write(p.join("01.txt"), "P\nSAIDA").unwrap();
    fs::write(p.join("prioridades.txt"), "1").unwrap();
    fs::write(p.join(std::ffi::OsString::from_vec(vec![0xfe])), "ignorar").unwrap();
    assert_eq!(carregar_programas(&p).unwrap().len(), 1);
}
