//! Registro proprietário dos contextos ativos.
use crate::{
    erros::ErroEp,
    modelos::{Bcp, ProcessId},
};
use std::collections::HashMap;
#[derive(Debug, Default)]
pub struct TabelaProcessos {
    processos: HashMap<ProcessId, Bcp>,
}
impl TabelaProcessos {
    pub fn nova() -> Self {
        Self::default()
    }
    /// Registra um contexto ativo sem sobrescrever um PID existente.
    pub fn inserir(&mut self, bcp: Bcp) -> Result<(), ErroEp> {
        match self.processos.entry(bcp.pid) {
            std::collections::hash_map::Entry::Vacant(entrada) => {
                entrada.insert(bcp);
                Ok(())
            }
            std::collections::hash_map::Entry::Occupied(entrada) => Err(ErroEp::EstadoInvalido(
                format!("PID {} já registrado", entrada.key().0),
            )),
        }
    }
    pub fn get(&self, pid: ProcessId) -> Option<&Bcp> {
        self.processos.get(&pid)
    }
    pub fn get_mut(&mut self, pid: ProcessId) -> Option<&mut Bcp> {
        self.processos.get_mut(&pid)
    }
    pub fn remover(&mut self, pid: ProcessId) -> Option<Bcp> {
        self.processos.remove(&pid)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&ProcessId, &Bcp)> {
        self.processos.iter()
    }
    pub fn len(&self) -> usize {
        self.processos.len()
    }
    pub fn is_empty(&self) -> bool {
        self.processos.is_empty()
    }
    pub fn vazia(&self) -> bool {
        self.is_empty()
    }
}
