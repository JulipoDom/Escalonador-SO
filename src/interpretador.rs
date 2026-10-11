//! Execução de uma instrução da CPU fictícia sobre o contexto do processo.
use crate::{
    erros::ErroEp,
    modelos::{Bcp, Instrucao, MotivoParada},
};

/// Executa a instrução apontada pelo PC e avança o PC exatamente uma vez.
/// COM somente consome CPU; atribuições atualizam X ou Y. E/S e SAIDA
/// interrompem imediatamente com seu motivo; as demais retornam `Ok(None)`.
/// Retomar E/S usa a próxima instrução, sem relê-la. Preserva os demais
/// campos do BCP. PC fora do texto retorna erro sem alterar o contexto.
pub fn executar_instrucao(bcp: &mut Bcp) -> Result<Option<MotivoParada>, ErroEp> {
    let instrucao = bcp
        .programa
        .instrucoes
        .get(bcp.pc)
        .copied()
        .ok_or_else(|| {
            ErroEp::EstadoInvalido(format!(
                "PC {} fora do texto do processo {}",
                bcp.pc, bcp.pid.0
            ))
        })?;
    let motivo = match instrucao {
        Instrucao::AtribuirX(valor) => {
            bcp.x = valor;
            None
        }
        Instrucao::AtribuirY(valor) => {
            bcp.y = valor;
            None
        }
        Instrucao::Com => None,
        Instrucao::EntradaSaida => Some(MotivoParada::Es),
        Instrucao::Saida => Some(MotivoParada::Saida),
    };
    bcp.pc += 1;
    Ok(motivo)
}
