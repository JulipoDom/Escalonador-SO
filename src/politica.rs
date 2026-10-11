//! Política de créditos dos contextos ativos.
use crate::{erros::ErroEp, modelos::ProcessId, tabela_processos::TabelaProcessos};

/// Redistribuir somente quando todos os ativos, inclusive bloqueados, têm zero.
/// Restaurar os créditos às prioridades originais; o chamador reordena os prontos
/// estavelmente quando o retorno for `true`. Todos com prioridade zero usam FIFO
/// sem redistribuição infinita; processos terminados não participam.
pub fn redistribuir_se_necessario(tabela: &mut TabelaProcessos) -> Result<bool, ErroEp> {
    if tabela.iter().any(|(_, bcp)| bcp.creditos > 0)
        || !tabela.iter().any(|(_, bcp)| bcp.prioridade > 0)
    {
        return Ok(false);
    }
    let ids: Vec<_> = tabela.iter().map(|(pid, _)| *pid).collect();
    for pid in ids {
        let bcp = tabela.get_mut(pid).expect("PID coletado da própria tabela");
        bcp.creditos = bcp.prioridade;
    }
    Ok(true)
}

/// Descontar um crédito no início do despacho usando saturating_sub(1),
/// independentemente do quantum ou da quantidade de instruções executadas.
pub fn descontar_credito(pid: ProcessId, tabela: &mut TabelaProcessos) -> Result<(), ErroEp> {
    let bcp = tabela
        .get_mut(pid)
        .ok_or_else(|| ErroEp::EstadoInvalido(format!("PID {} ausente para desconto", pid.0)))?;
    bcp.creditos = bcp.creditos.saturating_sub(1);
    Ok(())
}
