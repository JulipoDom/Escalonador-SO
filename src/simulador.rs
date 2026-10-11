//! Execução de quantum, passo de escalonamento e contrato reservado do laço.
use crate::{
    erros::ErroEp,
    filas::{FilaBloqueados, FilaProntos},
    interpretador::executar_instrucao,
    modelos::{Bcp, EstadoProcesso, MotivoParada, ProcessId, ResultadoQuantum, ResultadoSimulacao},
    politica::{descontar_credito, redistribuir_se_necessario},
    registro::RegistroLog,
    tabela_processos::TabelaProcessos,
};
use std::collections::HashSet;

/// Resultado de um único despacho ou tick lógico, sem estatísticas ou logs.
#[derive(Debug)]
pub enum ResultadoPasso {
    Surto {
        resultado: ResultadoQuantum,
        terminado: Option<Bcp>,
    },
    Ocioso {
        liberados: Vec<ProcessId>,
    },
    Concluido,
}

/// Valida o estado e executa um único surto, ou um tick se todos estão bloqueados.
/// Erros de entrada/estrutura não mutam o estado. Erros da CPU preservam o
/// despacho abortado: BCP Executando fora das filas, crédito descontado e
/// progresso válido; as esperas anteriores não avançam nesse caso.
pub fn executar_passo(
    tabela: &mut TabelaProcessos,
    prontos: &mut FilaProntos,
    bloqueados: &mut FilaBloqueados,
    quantum: usize,
) -> Result<ResultadoPasso, ErroEp> {
    if quantum == 0 {
        return Err(ErroEp::Entrada("quantum deve ser positivo".into()));
    }
    validar_estado(tabela, prontos, bloqueados)?;
    if tabela.is_empty() {
        return Ok(ResultadoPasso::Concluido);
    }
    let anteriores: Vec<_> = bloqueados.ids().iter().copied().collect();
    if prontos.ids().is_empty() {
        let liberados = avancar_e_reinserir(&anteriores, tabela, prontos, bloqueados)?;
        return Ok(ResultadoPasso::Ocioso { liberados });
    }
    if redistribuir_se_necessario(tabela)? {
        prontos.reordenar_apos_redistribuicao(tabela)?;
    }
    let pid = prontos
        .selecionar(tabela)?
        .expect("fila pronta não vazia e validada");
    descontar_credito(pid, tabela)?;
    let bcp = tabela
        .get_mut(pid)
        .expect("PID selecionado da tabela validada");
    bcp.estado = EstadoProcesso::Executando;
    let resultado = executar_quantum(bcp, quantum)?;
    let terminado = match resultado.motivo {
        MotivoParada::Quantum => {
            tabela.get_mut(pid).expect("processo ativo").estado = EstadoProcesso::Pronto;
            prontos.reinserir_apos_quantum(pid, tabela)?;
            None
        }
        MotivoParada::Saida => tabela.remover(pid),
        MotivoParada::Es => {
            bloqueados.bloquear(pid, tabela)?;
            None
        }
    };
    avancar_e_reinserir(&anteriores, tabela, prontos, bloqueados)?;
    Ok(ResultadoPasso::Surto {
        resultado,
        terminado,
    })
}

fn avancar_e_reinserir(
    anteriores: &[ProcessId],
    tabela: &mut TabelaProcessos,
    prontos: &mut FilaProntos,
    bloqueados: &mut FilaBloqueados,
) -> Result<Vec<ProcessId>, ErroEp> {
    let liberados = bloqueados.avancar_esperas(anteriores, tabela)?;
    for pid in &liberados {
        prontos.reinserir_apos_es(*pid, tabela)?;
    }
    Ok(liberados)
}

fn validar_estado(
    tabela: &TabelaProcessos,
    prontos: &FilaProntos,
    bloqueados: &FilaBloqueados,
) -> Result<(), ErroEp> {
    let mut vistos = HashSet::new();
    for (pid, estado) in prontos
        .ids()
        .iter()
        .map(|pid| (*pid, EstadoProcesso::Pronto))
        .chain(
            bloqueados
                .ids()
                .iter()
                .map(|pid| (*pid, EstadoProcesso::Bloqueado)),
        )
    {
        let bcp = tabela
            .get(pid)
            .ok_or_else(|| ErroEp::EstadoInvalido(format!("PID {} ausente na tabela", pid.0)))?;
        let espera_compativel = match estado {
            EstadoProcesso::Pronto => bcp.espera == 0,
            EstadoProcesso::Bloqueado => bcp.espera > 0,
            EstadoProcesso::Executando => false,
        };
        if !vistos.insert(pid) || bcp.pid != pid || bcp.estado != estado || !espera_compativel {
            return Err(ErroEp::EstadoInvalido(format!(
                "PID {} inconsistente com as filas",
                pid.0
            )));
        }
    }
    for (pid, bcp) in tabela.iter() {
        if bcp.pid != *pid || !vistos.contains(pid) || bcp.estado == EstadoProcesso::Executando {
            return Err(ErroEp::EstadoInvalido(format!(
                "PID {} ativo fora de uma fila válida",
                pid.0
            )));
        }
    }
    Ok(())
}

/// Executa no máximo `quantum` instruções, parando antes por E/S ou SAIDA.
/// E/S e SAIDA contam na quantidade executada e avançam o PC uma única vez.
/// Quantum zero retorna erro de entrada sem modificar o contexto. Erros de
/// instrução são propagados, preservando o progresso das instruções válidas.
/// Preserva créditos, estado e espera: as transições cabem ao laço de simulação.
pub fn executar_quantum(bcp: &mut Bcp, quantum: usize) -> Result<ResultadoQuantum, ErroEp> {
    if quantum == 0 {
        return Err(ErroEp::Entrada("quantum deve ser positivo".into()));
    }
    for executadas in 0..quantum {
        if let Some(motivo) = executar_instrucao(bcp)? {
            return Ok(ResultadoQuantum {
                pid: bcp.pid,
                instrucoes_executadas: executadas + 1,
                motivo,
            });
        }
    }
    Ok(ResultadoQuantum {
        pid: bcp.pid,
        instrucoes_executadas: quantum,
        motivo: MotivoParada::Quantum,
    })
}

/// Coordenará a tabela proprietária de todos os BCPs ativos e filas de IDs.
/// Cada despacho descontará um crédito com saturação em zero. Redistribuirá
/// somente quando todos os ativos, inclusive bloqueados, tiverem zero.
/// O BCP executando permanecerá na tabela; antes de remover um terminado,
/// salvará seu nome e registradores finais. A simulação termina com tabela vazia.
///
/// E/S define espera dois e entra no fim da fila FIFO. Ao fim do surto, somente
/// bloqueados que já existiam em seu início terão a espera descontada: o surto
/// que iniciou a própria E/S não conta. Dois surtos posteriores bastam, mesmo
/// do mesmo PID ou mais curtos que o quantum. Liberar elegíveis em ordem FIFO.
/// Sem prontos, avançar espera em unidades lógicas até haver pronto; ociosidade
/// não executa instruções, desconta créditos ou participa das médias.
///
/// Ordem das transições: registrar parada; reinserir, bloquear ou remover o
/// atual; atualizar bloqueados anteriores; liberar elegíveis; verificar
/// redistribuição antes do próximo despacho. Reinserção por quantum precede
/// empates, retorno de E/S sucede empates; FIFO prevalece quando prontos têm
/// zero e algum bloqueado tem crédito. Não duplicar IDs nem contextos.
/// Nesta base retorna erro sem escrever eventos, resumo ou resultados fictícios.
pub fn simular(
    _processos: Vec<Bcp>,
    _quantum: usize,
    _registro: &mut RegistroLog,
) -> Result<ResultadoSimulacao, ErroEp> {
    Err(ErroEp::NaoImplementado(
        "laço de simulação, despacho por créditos, transições e espera lógica de E/S",
    ))
}
