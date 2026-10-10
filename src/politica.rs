//! Contratos reservados da política de créditos.
use crate::{erros::ErroEp, tabela_processos::TabelaProcessos};

/// Redistribuir somente quando todos os ativos, inclusive bloqueados, têm zero.
/// Restaurar os créditos às prioridades originais; o chamador reordena os prontos
/// estavelmente quando o retorno for `true`. Todos com prioridade zero usam FIFO
/// sem redistribuição infinita; processos terminados não participam.
pub fn redistribuir_se_necessario(_tabela: &mut TabelaProcessos) -> Result<bool, ErroEp> {
    Err(ErroEp::NaoImplementado(
        "redistribuição de créditos de todos os processos ativos",
    ))
}

/// Descontar um crédito no início do despacho usando saturating_sub(1),
/// independentemente do quantum ou da quantidade de instruções executadas.
pub fn descontar_credito(
    _pid: crate::modelos::ProcessId,
    _tabela: &mut TabelaProcessos,
) -> Result<(), ErroEp> {
    Err(ErroEp::NaoImplementado(
        "desconto saturado de um crédito por despacho",
    ))
}
