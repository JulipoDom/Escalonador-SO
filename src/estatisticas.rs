//! Contagem dos surtos concluídos e cálculo das médias da execução.
use crate::{
    erros::ErroEp,
    modelos::{ResultadoQuantum, Resumo},
};

/// Acumula despachos e instruções de uma única execução.
#[derive(Debug, Default)]
pub struct Estatisticas {
    despachos: usize,
    instrucoes: usize,
}

impl Estatisticas {
    /// Conta um surto concluído, inclusive E/S, SAIDA e repetições do PID.
    /// Falhas de validação ou overflow preservam os dois contadores.
    pub fn registrar(&mut self, resultado: &ResultadoQuantum) -> Result<(), ErroEp> {
        if resultado.instrucoes_executadas == 0 {
            return Err(ErroEp::EstadoInvalido(
                "surto concluído exige ao menos uma instrução".into(),
            ));
        }
        let despachos = self
            .despachos
            .checked_add(1)
            .ok_or_else(|| ErroEp::EstadoInvalido("overflow na contagem de despachos".into()))?;
        let instrucoes = self
            .instrucoes
            .checked_add(resultado.instrucoes_executadas)
            .ok_or_else(|| ErroEp::EstadoInvalido("overflow na contagem de instruções".into()))?;
        self.despachos = despachos;
        self.instrucoes = instrucoes;
        Ok(())
    }

    /// Calcula as médias usando somente os surtos registrados neste contexto.
    pub fn resumo(&self, quantum: usize, processos: usize) -> Result<Resumo, ErroEp> {
        calcular_resumo(quantum, processos, self.instrucoes, self.despachos)
    }
}

/// Calcula D/P e I/D em ponto flutuante com quantum e denominadores positivos.
pub fn calcular_resumo(
    quantum: usize,
    processos: usize,
    instrucoes: usize,
    despachos: usize,
) -> Result<Resumo, ErroEp> {
    if quantum == 0 {
        return Err(ErroEp::Entrada("quantum deve ser positivo".into()));
    }
    if processos == 0 || despachos == 0 {
        return Err(ErroEp::EstadoInvalido(
            "resumo exige processos e despachos positivos".into(),
        ));
    }
    Ok(Resumo {
        quantum,
        total_processos: processos,
        total_instrucoes: instrucoes,
        total_despachos: despachos,
        media_trocas: despachos as f64 / processos as f64,
        media_instrucoes: instrucoes as f64 / despachos as f64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelos::{MotivoParada, ProcessId};

    #[test]
    fn overflow_de_instrucoes_nao_muta_contadores() {
        let mut e = Estatisticas {
            despachos: 3,
            instrucoes: usize::MAX,
        };
        assert!(matches!(
            e.registrar(&ResultadoQuantum {
                pid: ProcessId(1),
                instrucoes_executadas: 1,
                motivo: MotivoParada::Saida
            }),
            Err(ErroEp::EstadoInvalido(_))
        ));
        assert_eq!(e.despachos, 3);
        assert_eq!(e.instrucoes, usize::MAX);
    }

    #[test]
    fn overflow_de_despachos_nao_muta_contadores() {
        let mut e = Estatisticas {
            despachos: usize::MAX,
            instrucoes: 3,
        };
        assert!(matches!(
            e.registrar(&ResultadoQuantum {
                pid: ProcessId(1),
                instrucoes_executadas: 1,
                motivo: MotivoParada::Es
            }),
            Err(ErroEp::EstadoInvalido(_))
        ));
        assert_eq!(e.despachos, usize::MAX);
        assert_eq!(e.instrucoes, 3);
    }
}
