//! Filas possuem somente identificadores; decisões de escalonamento estão reservadas.
use crate::{erros::ErroEp, modelos::ProcessId, tabela_processos::TabelaProcessos};
use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct FilaProntos {
    ids: Vec<ProcessId>,
}
impl FilaProntos {
    pub fn nova() -> Self {
        Self::default()
    }
    pub fn ids(&self) -> &[ProcessId] {
        &self.ids
    }
    /// Ordenar inicialmente por créditos decrescentes e desempatar pelo arquivo.
    pub fn ordenar_inicial(&mut self, _tabela: &TabelaProcessos) -> Result<(), ErroEp> {
        Err(ErroEp::NaoImplementado(
            "ordenação inicial por créditos e arquivo",
        ))
    }
    /// Retirar o pronto de maior crédito; usar FIFO com créditos zero enquanto um bloqueado tem créditos.
    pub fn selecionar(&mut self, _tabela: &TabelaProcessos) -> Result<Option<ProcessId>, ErroEp> {
        Err(ErroEp::NaoImplementado(
            "seleção do próximo pronto por créditos ou FIFO",
        ))
    }
    /// Reinserir antes dos empates após quantum, salvo a regra FIFO dos prontos sem créditos.
    pub fn reinserir_apos_quantum(
        &mut self,
        _pid: ProcessId,
        _tabela: &TabelaProcessos,
    ) -> Result<(), ErroEp> {
        Err(ErroEp::NaoImplementado(
            "reinserção após quantum e exceção FIFO",
        ))
    }
    /// Reinserir após os empates no retorno da E/S, preservando a ordem de retorno.
    pub fn reinserir_apos_es(
        &mut self,
        _pid: ProcessId,
        _tabela: &TabelaProcessos,
    ) -> Result<(), ErroEp> {
        Err(ErroEp::NaoImplementado(
            "reinserção após E/S atrás dos empates",
        ))
    }
    /// Reordenar estavelmente por créditos após redistribuição, preservando empates.
    pub fn reordenar_apos_redistribuicao(
        &mut self,
        _tabela: &TabelaProcessos,
    ) -> Result<(), ErroEp> {
        Err(ErroEp::NaoImplementado(
            "reordenação estável após redistribuição",
        ))
    }
}

#[derive(Debug, Default)]
pub struct FilaBloqueados {
    ids: VecDeque<ProcessId>,
}
impl FilaBloqueados {
    pub fn nova() -> Self {
        Self::default()
    }
    pub fn ids(&self) -> &VecDeque<ProcessId> {
        &self.ids
    }
    /// Inserir ao final após E/S; definir espera dois sem descontá-la no próprio surto.
    pub fn bloquear(
        &mut self,
        _pid: ProcessId,
        _tabela: &mut TabelaProcessos,
    ) -> Result<(), ErroEp> {
        Err(ErroEp::NaoImplementado(
            "bloqueio FIFO por E/S com espera inicial dois",
        ))
    }
    /// Reduzir apenas esperas anteriores ao surto e liberar elegíveis em FIFO.
    /// Em ociosidade, avançar uma unidade lógica sem instruções, créditos ou despachos.
    pub fn avancar_esperas(
        &mut self,
        _anteriores: &[ProcessId],
        _tabela: &mut TabelaProcessos,
    ) -> Result<Vec<ProcessId>, ErroEp> {
        Err(ErroEp::NaoImplementado(
            "avanço lógico das esperas e liberação FIFO",
        ))
    }
}
