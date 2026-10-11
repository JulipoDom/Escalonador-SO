use ep1_escalonador::{
    carregador::{carregar_programas, ler_quantum},
    erros::ErroEp,
    registro::RegistroLog,
    simulador::simular,
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
    let processos = carregar_programas(diretorio)?;
    let quantum = ler_quantum(diretorio)?;
    let mut registro = RegistroLog::criar(Path::new("."), quantum)?;
    simular(processos, quantum, &mut registro)?;
    Ok(())
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
