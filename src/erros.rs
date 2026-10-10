//! Erros de entrada, infraestrutura e regras ainda reservadas.
use std::{error::Error, fmt, io};

#[derive(Debug)]
pub enum ErroEp {
    Entrada(String),
    Io(io::Error),
    EstadoInvalido(String),
    NaoImplementado(&'static str),
}

impl fmt::Display for ErroEp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Entrada(contexto) => write!(f, "Entrada inválida: {contexto}"),
            Self::Io(erro) => write!(f, "Erro de E/S: {erro}"),
            Self::EstadoInvalido(contexto) => write!(f, "Estado inválido: {contexto}"),
            Self::NaoImplementado(regra) => write!(f, "Não implementado: {regra}"),
        }
    }
}

impl Error for ErroEp {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(erro) => Some(erro),
            _ => None,
        }
    }
}

impl From<io::Error> for ErroEp {
    fn from(erro: io::Error) -> Self {
        Self::Io(erro)
    }
}
