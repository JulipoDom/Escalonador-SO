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
    // A integração da simulação com estatísticas e logs continua pendente.
    Err(ErroEp::NaoImplementado(
        "integração da simulação com estatísticas e logs",
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
