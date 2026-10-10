//! Contratos reservados de despacho, quantum e passagem do tempo lógico.
use crate::{
    erros::ErroEp,
    modelos::{Bcp, ResultadoQuantum, ResultadoSimulacao},
    registro::RegistroLog,
};

/// Executará no máximo `quantum` instruções, parando antes por E/S ou SAIDA.
/// E/S e SAIDA contam na quantidade executada e avançam o PC uma única vez.
/// O quantum deve ser positivo; sua validação faz parte do núcleo pendente.
/// O crédito será descontado uma vez por despacho pelo laço de simulação,
/// nunca por instrução ou por esta função. O BCP continuará pertencendo à
/// tabela durante a execução, acessado por um empréstimo mutável curto.
/// Nesta base retorna erro sem consumir instruções nem modificar o contexto.
pub fn executar_quantum(_bcp: &mut Bcp, _quantum: usize) -> Result<ResultadoQuantum, ErroEp> {
    Err(ErroEp::NaoImplementado(
        "execução do quantum com contagem de instruções e parada por E/S ou SAIDA",
    ))
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
