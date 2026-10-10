//! Execuções independentes e exportação CSV reservadas.
use crate::{erros::ErroEp, modelos::Resumo};
use std::path::Path;
/// Futuramente recria BCPs para cada quantum e exporta resultados medidos.
/// Enquanto o núcleo estiver pendente, não lê nem cria arquivos.
pub fn executar_experimentos(
    _diretorio: &Path,
    _quanta: &[usize],
    _destino: &Path,
) -> Result<Vec<Resumo>, ErroEp> {
    Err(ErroEp::NaoImplementado(
        "execuções independentes por quantum e exportação CSV",
    ))
}
