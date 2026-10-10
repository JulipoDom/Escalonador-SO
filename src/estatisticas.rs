//! Coleta e cálculo reservados para o núcleo futuro.
use crate::{
    erros::ErroEp,
    modelos::{ResultadoQuantum, Resumo},
};
/// Contexto local reservado à coleta de uma execução.
/// A representação dos contadores será acrescentada junto da coleta efetiva.
#[derive(Debug, Default)]
pub struct Estatisticas;
impl Estatisticas {
    /// Futuramente acumula um despacho e suas instruções neste contexto.
    pub fn registrar(&mut self, _resultado: &ResultadoQuantum) -> Result<(), ErroEp> {
        Err(ErroEp::NaoImplementado(
            "coleta de despachos e instruções executadas",
        ))
    }
}
/// Calcula D/P e I/D após validar denominadores; implementação pendente.
pub fn calcular_resumo(
    _quantum: usize,
    _processos: usize,
    _instrucoes: usize,
    _despachos: usize,
) -> Result<Resumo, ErroEp> {
    Err(ErroEp::NaoImplementado("cálculo das médias D/P e I/D"))
}
