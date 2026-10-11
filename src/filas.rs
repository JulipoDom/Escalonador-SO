//! Filas de identificadores; os contextos pertencem à tabela.
use crate::{
    erros::ErroEp,
    modelos::{EstadoProcesso, ProcessId},
    tabela_processos::TabelaProcessos,
};
use std::collections::{HashSet, VecDeque};

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
    /// Carrega todos os prontos, ordenados por créditos, arquivo e PID.
    pub fn ordenar_inicial(&mut self, tabela: &TabelaProcessos) -> Result<(), ErroEp> {
        self.ids = tabela
            .iter()
            .filter(|(_, bcp)| bcp.estado == EstadoProcesso::Pronto)
            .map(|(pid, _)| *pid)
            .collect();
        self.ids.sort_by(|a, b| {
            let a_bcp = tabela.get(*a).expect("PID coletado da tabela");
            let b_bcp = tabela.get(*b).expect("PID coletado da tabela");
            b_bcp
                .creditos
                .cmp(&a_bcp.creditos)
                .then_with(|| a_bcp.programa.arquivo.cmp(&b_bcp.programa.arquivo))
                .then_with(|| a.0.cmp(&b.0))
        });
        Ok(())
    }
    /// Retira o maior crédito, preservando FIFO nos empates; não altera o BCP.
    pub fn selecionar(&mut self, tabela: &TabelaProcessos) -> Result<Option<ProcessId>, ErroEp> {
        validar_ids(self.ids.iter().copied(), tabela, EstadoProcesso::Pronto)?;
        let mut melhor = None;
        for (indice, pid) in self.ids.iter().enumerate() {
            let creditos = tabela.get(*pid).expect("fila validada").creditos;
            if melhor.is_none_or(|(_, maior)| creditos > maior) {
                melhor = Some((indice, creditos));
            }
        }
        Ok(melhor.map(|(indice, _)| self.ids.remove(indice)))
    }
    /// Reinserir antes dos empates após quantum, salvo a regra FIFO dos prontos sem créditos.
    pub fn reinserir_apos_quantum(
        &mut self,
        pid: ProcessId,
        tabela: &TabelaProcessos,
    ) -> Result<(), ErroEp> {
        self.reinserir(pid, tabela, true)
    }
    /// Reinserir após os empates no retorno da E/S, preservando a ordem de retorno.
    pub fn reinserir_apos_es(
        &mut self,
        pid: ProcessId,
        tabela: &TabelaProcessos,
    ) -> Result<(), ErroEp> {
        self.reinserir(pid, tabela, false)
    }
    fn reinserir(
        &mut self,
        pid: ProcessId,
        tabela: &TabelaProcessos,
        antes_empates: bool,
    ) -> Result<(), ErroEp> {
        validar_ids(
            self.ids.iter().copied().chain(std::iter::once(pid)),
            tabela,
            EstadoProcesso::Pronto,
        )?;
        let creditos = tabela.get(pid).expect("candidato validado").creditos;
        let todos_zero = creditos == 0
            && self
                .ids
                .iter()
                .all(|id| tabela.get(*id).expect("fila validada").creditos == 0);
        let bloqueado_positivo = tabela
            .iter()
            .any(|(_, bcp)| bcp.estado == EstadoProcesso::Bloqueado && bcp.creditos > 0);
        let prioridades_zero = tabela.iter().all(|(_, bcp)| bcp.prioridade == 0);
        if (todos_zero && bloqueado_positivo) || prioridades_zero {
            self.ids.push(pid);
        } else {
            self.reordenar_apos_redistribuicao(tabela)?;
            let posicao = self
                .ids
                .iter()
                .position(|id| {
                    let atual = tabela.get(*id).expect("fila validada").creditos;
                    atual < creditos || (antes_empates && atual == creditos)
                })
                .unwrap_or(self.ids.len());
            self.ids.insert(posicao, pid);
        }
        Ok(())
    }
    /// Reordena estavelmente por créditos, preservando a ordem dos empates.
    pub fn reordenar_apos_redistribuicao(
        &mut self,
        tabela: &TabelaProcessos,
    ) -> Result<(), ErroEp> {
        validar_ids(self.ids.iter().copied(), tabela, EstadoProcesso::Pronto)?;
        self.ids.sort_by_key(|pid| {
            std::cmp::Reverse(tabela.get(*pid).expect("fila validada").creditos)
        });
        Ok(())
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
    /// Insere um Executando ao final após E/S, com espera dois.
    pub fn bloquear(&mut self, pid: ProcessId, tabela: &mut TabelaProcessos) -> Result<(), ErroEp> {
        validar_ids(self.ids.iter().copied(), tabela, EstadoProcesso::Bloqueado)?;
        validar_ids(std::iter::once(pid), tabela, EstadoProcesso::Executando)?;
        if self.ids.contains(&pid) {
            return Err(ErroEp::EstadoInvalido(format!(
                "PID {} já bloqueado",
                pid.0
            )));
        }
        let bcp = tabela.get_mut(pid).expect("candidato validado");
        bcp.estado = EstadoProcesso::Bloqueado;
        bcp.espera = 2;
        self.ids.push_back(pid);
        Ok(())
    }
    /// Reduzir apenas esperas anteriores ao surto e liberar elegíveis em FIFO.
    /// Em ociosidade, avançar uma unidade lógica sem instruções, créditos ou despachos.
    pub fn avancar_esperas(
        &mut self,
        anteriores: &[ProcessId],
        tabela: &mut TabelaProcessos,
    ) -> Result<Vec<ProcessId>, ErroEp> {
        validar_ids(self.ids.iter().copied(), tabela, EstadoProcesso::Bloqueado)?;
        validar_ids(
            anteriores.iter().copied(),
            tabela,
            EstadoProcesso::Bloqueado,
        )?;
        if let Some(pid) = anteriores.iter().find(|pid| !self.ids.contains(pid)) {
            return Err(ErroEp::EstadoInvalido(format!(
                "PID {} não pertence à fila de bloqueados",
                pid.0
            )));
        }
        for pid in anteriores {
            let bcp = tabela.get_mut(*pid).expect("snapshot validado");
            bcp.espera = bcp.espera.saturating_sub(1);
        }
        let mut liberados = Vec::new();
        self.ids.retain(|pid| {
            let bcp = tabela.get_mut(*pid).expect("fila validada");
            if bcp.espera == 0 {
                bcp.estado = EstadoProcesso::Pronto;
                liberados.push(*pid);
                false
            } else {
                true
            }
        });
        Ok(liberados)
    }
}

fn validar_ids(
    ids: impl Iterator<Item = ProcessId>,
    tabela: &TabelaProcessos,
    estado: EstadoProcesso,
) -> Result<(), ErroEp> {
    let mut vistos = HashSet::new();
    for pid in ids {
        let bcp = tabela
            .get(pid)
            .ok_or_else(|| ErroEp::EstadoInvalido(format!("PID {} ausente na tabela", pid.0)))?;
        if !vistos.insert(pid) || bcp.estado != estado {
            return Err(ErroEp::EstadoInvalido(format!(
                "PID {} duplicado ou em estado incompatível com a fila",
                pid.0
            )));
        }
    }
    Ok(())
}
