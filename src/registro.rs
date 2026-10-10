//! Escritor dedicado dos eventos de uma execução, sem redirecionar stdout.
use crate::{erros::ErroEp, modelos::Resumo};
use std::{
    fmt,
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

/// Eventos de decisão; o chamador fornece a ordem e a contagem do último surto.
#[derive(Debug, Clone, Copy)]
pub enum EventoLog<'a> {
    Carregando(&'a str),
    Executando(&'a str),
    EntradaSaida(&'a str),
    Interrompendo { nome: &'a str, instrucoes: usize },
    Terminado { nome: &'a str, x: i64, y: i64 },
}
impl fmt::Display for EventoLog<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Carregando(nome) => write!(f, "Carregando {nome}"),
            Self::Executando(nome) => write!(f, "Executando {nome}"),
            Self::EntradaSaida(nome) => write!(f, "E/S iniciada em {nome}"),
            Self::Interrompendo { nome, instrucoes } => {
                let unidade = if *instrucoes == 1 {
                    "instrução"
                } else {
                    "instruções"
                };
                write!(f, "Interrompendo {nome} após {instrucoes} {unidade}")
            }
            Self::Terminado { nome, x, y } => write!(f, "{nome} terminado. X={x}. Y={y}"),
        }
    }
}

/// Possui o arquivo de uma execução e verifica erros também nos flushes.
pub struct RegistroLog {
    escritor: BufWriter<File>,
}
impl RegistroLog {
    /// Cria os diretórios necessários e substitui o log deste quantum.
    pub fn criar(diretorio: &Path, quantum: usize) -> Result<Self, ErroEp> {
        fs::create_dir_all(diretorio)?;
        let arquivo = File::create(diretorio.join(format!("log{quantum:02}.txt")))?;
        Ok(Self {
            escritor: BufWriter::new(arquivo),
        })
    }
    /// Escreve uma decisão completa e a torna visível antes de retornar.
    pub fn registrar(&mut self, evento: EventoLog<'_>) -> Result<(), ErroEp> {
        writeln!(self.escritor, "{evento}")?;
        self.escritor.flush()?;
        Ok(())
    }
    /// Formata médias fornecidas; não coleta nem calcula resultados.
    pub fn escrever_resumo(&mut self, resumo: &Resumo) -> Result<(), ErroEp> {
        if resumo.total_processos == 0 || resumo.total_despachos == 0 {
            return Err(ErroEp::EstadoInvalido(
                "resumo exige processos e despachos positivos".into(),
            ));
        }
        writeln!(
            self.escritor,
            "MEDIA DE TROCAS: {}\nMEDIA DE INSTRUCOES: {}\nQUANTUM: {}",
            resumo.media_trocas, resumo.media_instrucoes, resumo.quantum
        )?;
        self.escritor.flush()?;
        Ok(())
    }
    /// Confirma a escrita final; erros nunca dependem apenas de Drop.
    pub fn finalizar(&mut self) -> Result<(), ErroEp> {
        self.escritor.flush()?;
        Ok(())
    }
}
