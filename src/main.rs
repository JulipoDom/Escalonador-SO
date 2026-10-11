use ep1_escalonador::{
    carregador::{carregar_programas, ler_quantum},
    erros::ErroEp,
};
use std::{path::Path, process::ExitCode};

fn executar() -> Result<(), ErroEp> {
    if let Some(argumento) = std::env::args_os().nth(1) {
        return Err(ErroEp::Entrada(format!(
            "argumento não suportado: {}",
            argumento.to_string_lossy()
        )));
    }
    let diretorio = Path::new("programas");
    let _processos = carregar_programas(diretorio)?;
    let _quantum = ler_quantum(diretorio)?;
    // Não abrir o log enquanto a simulação está pendente.
    Err(ErroEp::NaoImplementado(
        "laço de simulação, despacho e transições de processos",
    ))
}

fn main() -> ExitCode {
    match executar() {
        Ok(()) => ExitCode::SUCCESS,
        Err(erro) => {
            eprintln!("{erro}");
            ExitCode::FAILURE
        }
    }
}
