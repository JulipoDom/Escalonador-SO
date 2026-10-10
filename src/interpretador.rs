//! Contrato reservado da CPU fictícia; nenhuma instrução é executada nesta base.
use crate::{
    erros::ErroEp,
    modelos::{Bcp, MotivoParada},
};

/// Executará a instrução apontada pelo PC, preservando o contexto no BCP.
/// Cada instrução consumirá uma unidade, inclusive E/S e SAIDA, e avançará
/// o PC exatamente uma vez. COM somente consome CPU; atribuições atualizam
/// X ou Y. E/S e SAIDA interrompem imediatamente com seu motivo; as demais
/// retornam `Ok(None)`. Retomar E/S usará a próxima instrução, sem relê-la.
/// Nesta base retorna erro sem alterar PC, registradores ou qualquer campo.
pub fn executar_instrucao(_bcp: &mut Bcp) -> Result<Option<MotivoParada>, ErroEp> {
    Err(ErroEp::NaoImplementado(
        "execução de instruções, atualização de registradores e avanço único do PC",
    ))
}
