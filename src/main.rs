use std::process::ExitCode;
fn main() -> ExitCode {
    match ep1_escalonador::cli::executar(std::env::args_os().skip(1)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(erro) => {
            eprintln!("{erro}");
            ExitCode::FAILURE
        }
    }
}
