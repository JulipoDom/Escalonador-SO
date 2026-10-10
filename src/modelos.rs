//! Dados proprietários de programas e contextos de processos.
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProcessId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instrucao {
    AtribuirX(i64),
    AtribuirY(i64),
    Com,
    EntradaSaida,
    Saida,
}

#[derive(Debug)]
pub struct Programa {
    pub nome: String,
    pub arquivo: String,
    pub instrucoes: Box<[Instrucao]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EstadoProcesso {
    Pronto,
    Executando,
    Bloqueado,
}

#[derive(Debug)]
pub struct Bcp {
    pub pid: ProcessId,
    pub programa: Programa,
    pub pc: usize,
    pub estado: EstadoProcesso,
    pub prioridade: u32,
    pub creditos: u32,
    pub x: i64,
    pub y: i64,
    pub espera: u8,
}
impl Bcp {
    /// Cria um contexto pronto, com créditos iniciais iguais à prioridade.
    pub fn novo(pid: ProcessId, programa: Programa, prioridade: u32) -> Self {
        Self {
            pid,
            programa,
            pc: 0,
            estado: EstadoProcesso::Pronto,
            prioridade,
            creditos: prioridade,
            x: 0,
            y: 0,
            espera: 0,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotivoParada {
    Quantum,
    Es,
    Saida,
}
#[derive(Debug)]
pub struct ResultadoQuantum {
    pub pid: ProcessId,
    pub instrucoes_executadas: usize,
    pub motivo: MotivoParada,
}
#[derive(Debug)]
pub struct Resumo {
    pub quantum: usize,
    pub total_processos: usize,
    pub total_instrucoes: usize,
    pub total_despachos: usize,
    pub media_trocas: f64,
    pub media_instrucoes: f64,
}
#[derive(Debug)]
pub struct ResultadoSimulacao {
    pub resumo: Resumo,
    pub registradores_finais: HashMap<ProcessId, (i64, i64)>,
}
