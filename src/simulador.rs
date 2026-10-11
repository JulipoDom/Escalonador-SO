//! Execução de quantum, laço do núcleo e simulação com eventos e estatísticas reais.
use crate::{
    erros::ErroEp,
    estatisticas::Estatisticas,
    filas::{FilaBloqueados, FilaProntos},
    interpretador::executar_instrucao,
    modelos::{Bcp, EstadoProcesso, MotivoParada, ProcessId, ResultadoQuantum, ResultadoSimulacao},
    politica::{descontar_credito, redistribuir_se_necessario},
    registro::{EventoLog, RegistroLog},
    tabela_processos::TabelaProcessos,
};
use std::collections::{HashMap, HashSet};

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
    executar_passo_com_despacho(tabela, prontos, bloqueados, quantum, |_| Ok(()))
}

/// Observa o despacho real antes da CPU, inclusive quando ela retorna erro.
fn executar_passo_com_despacho(
    tabela: &mut TabelaProcessos,
    prontos: &mut FilaProntos,
    bloqueados: &mut FilaBloqueados,
    quantum: usize,
    mut ao_despachar: impl FnMut(&Bcp) -> Result<(), ErroEp>,
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
    ao_despachar(bcp)?;
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

/// Executa o núcleo até a conclusão, sem estatísticas ou registro.
/// Preserva os contextos fornecidos e propaga erros sem resultados parciais.
/// A conclusão depende de `executar_passo`, inclusive durante a ociosidade.
pub fn executar_laco(
    processos: Vec<Bcp>,
    quantum: usize,
) -> Result<HashMap<ProcessId, (i64, i64)>, ErroEp> {
    if quantum == 0 {
        return Err(ErroEp::Entrada("quantum deve ser positivo".into()));
    }
    let mut tabela = TabelaProcessos::nova();
    for bcp in processos {
        tabela.inserir(bcp)?;
    }
    let mut prontos = FilaProntos::nova();
    prontos.ordenar_inicial(&tabela)?;
    let mut bloqueados = FilaBloqueados::nova();
    let mut registradores_finais = HashMap::new();
    loop {
        match executar_passo(&mut tabela, &mut prontos, &mut bloqueados, quantum)? {
            ResultadoPasso::Surto {
                terminado: Some(bcp),
                ..
            } => {
                registradores_finais.insert(bcp.pid, (bcp.x, bcp.y));
            }
            ResultadoPasso::Surto {
                terminado: None, ..
            }
            | ResultadoPasso::Ocioso { .. } => {}
            ResultadoPasso::Concluido => return Ok(registradores_finais),
        }
    }
}

/// Executa o núcleo, registra os eventos reais e devolve contagens e contexto final.
/// Valida a entrada antes de escrever; entrada vazia tem denominadores indefinidos.
/// Erros abortam sem resumo de sucesso, preservando eventos já publicados.
pub fn simular(
    processos: Vec<Bcp>,
    quantum: usize,
    registro: &mut RegistroLog,
) -> Result<ResultadoSimulacao, ErroEp> {
    if quantum == 0 {
        return Err(ErroEp::Entrada("quantum deve ser positivo".into()));
    }
    let total_processos = processos.len();
    if total_processos == 0 {
        return Err(ErroEp::EstadoInvalido(
            "simulação exige ao menos um processo para calcular as médias".into(),
        ));
    }
    let mut tabela = TabelaProcessos::nova();
    for bcp in processos {
        tabela.inserir(bcp)?;
    }
    let mut prontos = FilaProntos::nova();
    prontos.ordenar_inicial(&tabela)?;
    let mut bloqueados = FilaBloqueados::nova();
    validar_estado(&tabela, &prontos, &bloqueados)?;
    for pid in prontos.ids() {
        let bcp = tabela.get(*pid).expect("fila inicial validada");
        registro.registrar(EventoLog::Carregando(&bcp.programa.nome))?;
    }
    let mut estatisticas = Estatisticas::default();
    let mut registradores_finais = HashMap::new();
    loop {
        let passo = executar_passo_com_despacho(
            &mut tabela,
            &mut prontos,
            &mut bloqueados,
            quantum,
            |bcp| registro.registrar(EventoLog::Executando(&bcp.programa.nome)),
        )?;
        match passo {
            ResultadoPasso::Surto {
                resultado,
                terminado,
            } => {
                estatisticas.registrar(&resultado)?;
                if let Some(bcp) = terminado {
                    registro.registrar(EventoLog::Terminado {
                        nome: &bcp.programa.nome,
                        x: bcp.x,
                        y: bcp.y,
                    })?;
                    registradores_finais.insert(bcp.pid, (bcp.x, bcp.y));
                } else {
                    let bcp = tabela.get(resultado.pid).expect("surto de processo ativo");
                    if resultado.motivo == MotivoParada::Es {
                        registro.registrar(EventoLog::EntradaSaida(&bcp.programa.nome))?;
                    }
                    registro.registrar(EventoLog::Interrompendo {
                        nome: &bcp.programa.nome,
                        instrucoes: resultado.instrucoes_executadas,
                    })?;
                }
            }
            ResultadoPasso::Ocioso { .. } => {}
            ResultadoPasso::Concluido => {
                let resumo = estatisticas.resumo(quantum, total_processos)?;
                registro.escrever_resumo(&resumo)?;
                registro.finalizar()?;
                return Ok(ResultadoSimulacao {
                    resumo,
                    registradores_finais,
                });
            }
        }
    }
}
