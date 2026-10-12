//! Escritor dedicado dos eventos de uma execução, sem redirecionar stdout.
use crate::{erros::ErroEp, modelos::Resumo};
use std::{
    fmt,
    fs::{self, File, OpenOptions},
    io::{self, BufWriter, Write},
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
    /// Publica um arquivo novo; links preexistentes não alteram seus alvos.
    pub fn criar(diretorio: &Path, quantum: usize) -> Result<Self, ErroEp> {
        fs::create_dir_all(diretorio)?;
        let destino = diretorio.join(format!("log{quantum:02}.txt"));
        let mut tentativa = 0usize;
        let (temporario, arquivo) = loop {
            let caminho = diretorio.join(format!(
                ".log{quantum:02}.txt.{}.{tentativa}.tmp",
                std::process::id()
            ));
            match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&caminho)
            {
                Ok(arquivo) => break (caminho, arquivo),
                Err(erro) if erro.kind() == io::ErrorKind::AlreadyExists => {
                    tentativa = tentativa.checked_add(1).ok_or(erro)?;
                }
                Err(erro) => return Err(erro.into()),
            }
        };
        if let Err(erro) = fs::rename(&temporario, destino) {
            drop(arquivo);
            fs::remove_file(temporario)?;
            return Err(erro.into());
        }
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

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    fn registro_dev_full() -> RegistroLog {
        RegistroLog {
            escritor: BufWriter::new(File::options().write(true).open("/dev/full").unwrap()),
        }
    }

    #[test]
    fn erro_real_de_flush_do_evento_e_propagado() {
        let mut registro = registro_dev_full();
        assert!(matches!(
            registro.registrar(EventoLog::Executando("P")),
            Err(ErroEp::Io(_))
        ));
        assert!(matches!(registro.finalizar(), Err(ErroEp::Io(_))));
    }

    #[test]
    fn erro_real_de_escrita_de_evento_maior_que_buffer_e_propagado() {
        let mut registro = registro_dev_full();
        let nome = "P".repeat(2 * registro.escritor.capacity());
        assert!(matches!(
            registro.registrar(EventoLog::Executando(&nome)),
            Err(ErroEp::Io(_))
        ));
    }

    #[test]
    fn erro_real_de_flush_do_resumo_e_propagado() {
        let mut registro = registro_dev_full();
        let resumo = Resumo {
            quantum: 1,
            total_processos: 1,
            total_instrucoes: 1,
            total_despachos: 1,
            media_trocas: 1.0,
            media_instrucoes: 1.0,
        };
        assert!(matches!(
            registro.escrever_resumo(&resumo),
            Err(ErroEp::Io(_))
        ));
    }

    #[test]
    fn erro_real_de_flush_final_e_propagado() {
        let mut registro = registro_dev_full();
        registro.escritor.write_all(b"pendente\n").unwrap();
        assert!(matches!(registro.finalizar(), Err(ErroEp::Io(_))));
    }
}
