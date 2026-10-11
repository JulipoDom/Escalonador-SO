use ep1_escalonador::{
    erros::ErroEp,
    modelos::{Bcp, EstadoProcesso, Instrucao, ProcessId, Programa, ResultadoSimulacao},
    registro::RegistroLog,
    simulador::simular,
};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};

static PROXIMO_DIRETORIO: AtomicUsize = AtomicUsize::new(0);

struct DiretorioTemporario(PathBuf);
impl DiretorioTemporario {
    fn novo() -> Self {
        let id = PROXIMO_DIRETORIO.fetch_add(1, Ordering::Relaxed);
        let caminho =
            std::env::temp_dir().join(format!("ep1-logs-simulacao-{}-{id}", std::process::id()));
        std::fs::create_dir(&caminho).unwrap();
        Self(caminho)
    }
}
impl Drop for DiretorioTemporario {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn processo(pid: u32, prioridade: u32, instrucoes: &[Instrucao]) -> Bcp {
    Bcp::novo(
        ProcessId(pid),
        Programa {
            nome: format!("P{pid}"),
            arquivo: format!("{pid:02}.txt"),
            instrucoes: instrucoes.into(),
        },
        prioridade,
    )
}

fn executar(processos: Vec<Bcp>, quantum: usize) -> (Result<ResultadoSimulacao, ErroEp>, String) {
    let diretorio = DiretorioTemporario::novo();
    let mut registro = RegistroLog::criar(&diretorio.0, quantum).unwrap();
    let resultado = simular(processos, quantum, &mut registro);
    // Leitura antes de Drop/finalizar detecta eventos ou resumo ainda no buffer.
    let log = std::fs::read_to_string(diretorio.0.join(format!("log{quantum:02}.txt"))).unwrap();
    (resultado, log)
}

// Detecta E/S vencida pelo quantum, ociosidade contada ou término com contexto perdido.
#[test]
fn es_na_fronteira_e_ociosidade_produzem_apenas_eventos_e_contagens_reais() {
    use Instrucao::*;
    let (resultado, log) = executar(
        vec![processo(1, 2, &[Com, EntradaSaida, AtribuirX(7), Saida])],
        2,
    );
    let resultado = resultado.unwrap();
    assert_eq!(
        resultado.registradores_finais,
        HashMap::from([(ProcessId(1), (7, 0))])
    );
    assert_eq!(resultado.resumo.quantum, 2);
    assert_eq!(resultado.resumo.total_processos, 1);
    assert_eq!(resultado.resumo.total_despachos, 2);
    assert_eq!(resultado.resumo.total_instrucoes, 4);
    assert_eq!(resultado.resumo.media_trocas, 2.0);
    assert_eq!(resultado.resumo.media_instrucoes, 2.0);
    assert_eq!(
        log,
        concat!(
            "Carregando P1\n",
            "Executando P1\n",
            "E/S iniciada em P1\n",
            "Interrompendo P1 após 2 instruções\n",
            "Executando P1\n",
            "P1 terminado. X=7. Y=0\n",
            "MEDIA DE TROCAS: 2\nMEDIA DE INSTRUCOES: 2\nQUANTUM: 2\n"
        )
    );
}

// Detecta deduplicação de despachos do mesmo PID ou uso de contagem acumulada na interrupção.
#[test]
fn quantum_repetido_conta_cada_surto_e_retoma_registradores() {
    use Instrucao::*;
    let (resultado, log) = executar(
        vec![processo(1, 1, &[AtribuirX(8), Com, AtribuirY(-4), Saida])],
        1,
    );
    let resultado = resultado.unwrap();
    assert_eq!(resultado.resumo.total_despachos, 4);
    assert_eq!(resultado.resumo.total_instrucoes, 4);
    assert_eq!(
        resultado.registradores_finais,
        HashMap::from([(ProcessId(1), (8, -4))])
    );
    assert_eq!(
        log,
        concat!(
            "Carregando P1\n",
            "Executando P1\nInterrompendo P1 após 1 instrução\n",
            "Executando P1\nInterrompendo P1 após 1 instrução\n",
            "Executando P1\nInterrompendo P1 após 1 instrução\n",
            "Executando P1\nP1 terminado. X=8. Y=-4\n",
            "MEDIA DE TROCAS: 4\nMEDIA DE INSTRUCOES: 1\nQUANTUM: 1\n"
        )
    );
}

// Detecta carga pela ordem de entrada, prioridade em vez de crédito ou empate instável de HashMap.
#[test]
fn carregando_segue_credito_arquivo_e_pid_da_fila_inicial() {
    let mut primeiro = processo(4, 0, &[Instrucao::Saida]);
    primeiro.creditos = 5;
    let mut segundo = processo(3, 9, &[Instrucao::Saida]);
    segundo.creditos = 2;
    segundo.programa.arquivo = "01.txt".into();
    let mut terceiro = processo(2, 1, &[Instrucao::Saida]);
    terceiro.creditos = 2;
    terceiro.programa.arquivo = "02.txt".into();
    let mut quarto = processo(5, 1, &[Instrucao::Saida]);
    quarto.creditos = 2;
    quarto.programa.arquivo = "02.txt".into();
    let (resultado, log) = executar(vec![quarto, terceiro, primeiro, segundo], 3);
    assert_eq!(resultado.unwrap().resumo.total_processos, 4);
    assert_eq!(
        log,
        concat!(
            "Carregando P4\nCarregando P3\nCarregando P2\nCarregando P5\n",
            "Executando P4\nP4 terminado. X=0. Y=0\n",
            "Executando P3\nP3 terminado. X=0. Y=0\n",
            "Executando P2\nP2 terminado. X=0. Y=0\n",
            "Executando P5\nP5 terminado. X=0. Y=0\n",
            "MEDIA DE TROCAS: 1\nMEDIA DE INSTRUCOES: 1\nQUANTUM: 3\n"
        )
    );
}

// Detecta reconstrução do BCP ou contagem de instruções anteriores ao PC fornecido.
#[test]
fn contexto_pronto_preserva_pc_e_registradores_iniciais() {
    use Instrucao::*;
    let mut bcp = processo(1, 3, &[AtribuirX(99), AtribuirY(3), Saida]);
    bcp.pc = 1;
    bcp.x = -12;
    bcp.y = 88;
    bcp.creditos = 0;
    let (resultado, log) = executar(vec![bcp], 1);
    let resultado = resultado.unwrap();
    assert_eq!(resultado.resumo.total_despachos, 2);
    assert_eq!(resultado.resumo.total_instrucoes, 2);
    assert_eq!(
        resultado.registradores_finais,
        HashMap::from([(ProcessId(1), (-12, 3))])
    );
    assert_eq!(
        log,
        concat!(
            "Carregando P1\nExecutando P1\nInterrompendo P1 após 1 instrução\n",
            "Executando P1\nP1 terminado. X=-12. Y=3\n",
            "MEDIA DE TROCAS: 2\nMEDIA DE INSTRUCOES: 1\nQUANTUM: 1\n"
        )
    );
}

// Detecta contagem de tick ocioso e avanço da espera recém-criada no mesmo surto.
#[test]
fn todos_bloqueados_retornam_sem_eventos_ou_instrucoes_de_ociosidade() {
    use Instrucao::*;
    let (resultado, log) = executar(
        vec![
            processo(1, 2, &[EntradaSaida, AtribuirX(1), Saida]),
            processo(2, 2, &[EntradaSaida, AtribuirY(2), Saida]),
        ],
        2,
    );
    let resultado = resultado.unwrap();
    assert_eq!(resultado.resumo.total_processos, 2);
    assert_eq!(resultado.resumo.total_despachos, 4);
    assert_eq!(resultado.resumo.total_instrucoes, 6);
    assert_eq!(resultado.resumo.media_trocas, 2.0);
    assert_eq!(resultado.resumo.media_instrucoes, 1.5);
    assert_eq!(
        resultado.registradores_finais,
        HashMap::from([(ProcessId(1), (1, 0)), (ProcessId(2), (0, 2))])
    );
    assert_eq!(
        log,
        concat!(
            "Carregando P1\nCarregando P2\n",
            "Executando P1\nE/S iniciada em P1\nInterrompendo P1 após 1 instrução\n",
            "Executando P2\nE/S iniciada em P2\nInterrompendo P2 após 1 instrução\n",
            "Executando P1\nP1 terminado. X=1. Y=0\n",
            "Executando P2\nP2 terminado. X=0. Y=2\n",
            "MEDIA DE TROCAS: 2\nMEDIA DE INSTRUCOES: 1.5\nQUANTUM: 2\n"
        )
    );
}

// Detecta política paralela em simular que altera FIFO ou a ordem após redistribuição.
#[test]
fn prioridades_zero_e_redistribuicao_preservam_a_politica_do_nucleo() {
    for (prioridade, finais) in [
        (
            0,
            "Executando P1\nP1 terminado. X=0. Y=0\nExecutando P2\nP2 terminado. X=0. Y=0\n",
        ),
        (
            1,
            "Executando P2\nP2 terminado. X=0. Y=0\nExecutando P1\nP1 terminado. X=0. Y=0\n",
        ),
    ] {
        let (resultado, log) = executar(
            vec![
                processo(1, prioridade, &[Instrucao::Com, Instrucao::Saida]),
                processo(2, prioridade, &[Instrucao::Com, Instrucao::Saida]),
            ],
            1,
        );
        let resultado = resultado.unwrap();
        assert_eq!(resultado.resumo.total_despachos, 4);
        assert_eq!(resultado.resumo.total_instrucoes, 4);
        assert_eq!(log, format!(
            "Carregando P1\nCarregando P2\nExecutando P1\nInterrompendo P1 após 1 instrução\nExecutando P2\nInterrompendo P2 após 1 instrução\n{finais}MEDIA DE TROCAS: 2\nMEDIA DE INSTRUCOES: 1\nQUANTUM: 1\n"
        ));
    }
}

// Detecta P medido após remoção da tabela ou divisão inteira das médias.
#[test]
fn resumo_usa_processos_iniciais_e_medias_fracionarias() {
    let (resultado, log) = executar(
        vec![
            processo(1, 3, &[Instrucao::Com, Instrucao::Com, Instrucao::Saida]),
            processo(2, 1, &[Instrucao::Saida]),
        ],
        2,
    );
    let resultado = resultado.unwrap();
    assert_eq!(resultado.resumo.total_processos, 2);
    assert_eq!(resultado.resumo.total_despachos, 3);
    assert_eq!(resultado.resumo.total_instrucoes, 4);
    assert_eq!(resultado.resumo.media_trocas, 1.5);
    assert_eq!(resultado.resumo.media_instrucoes, 4.0 / 3.0);
    assert_eq!(
        log,
        concat!(
            "Carregando P1\nCarregando P2\n",
            "Executando P1\nInterrompendo P1 após 2 instruções\n",
            "Executando P1\nP1 terminado. X=0. Y=0\n",
            "Executando P2\nP2 terminado. X=0. Y=0\n",
            "MEDIA DE TROCAS: 1.5\nMEDIA DE INSTRUCOES: 1.3333333333333333\nQUANTUM: 2\n"
        )
    );
}

// Detecta eventos publicados antes da validação de toda a entrada.
#[test]
fn entradas_invalidas_falham_antes_de_qualquer_evento() {
    for processos in [vec![], vec![processo(1, 1, &[Instrucao::Saida])]] {
        let (resultado, log) = executar(processos, 0);
        assert!(matches!(resultado, Err(ErroEp::Entrada(_))));
        assert!(log.is_empty());
    }
    let (resultado, log) = executar(vec![], 1);
    assert!(matches!(resultado, Err(ErroEp::EstadoInvalido(_))));
    assert!(log.is_empty());
    let (resultado, log) = executar(
        vec![
            processo(1, 1, &[Instrucao::Saida]),
            processo(1, 2, &[Instrucao::Saida]),
        ],
        1,
    );
    assert!(matches!(resultado, Err(ErroEp::EstadoInvalido(_))));
    assert!(log.is_empty());
    for (estado, espera) in [
        (EstadoProcesso::Executando, 0),
        (EstadoProcesso::Bloqueado, 0),
        (EstadoProcesso::Bloqueado, 2),
        (EstadoProcesso::Pronto, 1),
    ] {
        let mut invalido = processo(2, 1, &[Instrucao::Saida]);
        invalido.estado = estado;
        invalido.espera = espera;
        let (resultado, log) = executar(vec![processo(1, 5, &[Instrucao::Saida]), invalido], 1);
        assert!(matches!(resultado, Err(ErroEp::EstadoInvalido(_))));
        assert!(log.is_empty());
    }
}

// Detecta Executando omitido quando a CPU aborta e resumo publicado após progresso incompleto.
#[test]
fn erro_da_cpu_preserva_apenas_eventos_reais_sem_resumo_de_sucesso() {
    for apos_termino in [false, true] {
        let mut processos = vec![processo(2, 1, &[Instrucao::Com])];
        if apos_termino {
            processos.push(processo(1, 5, &[Instrucao::AtribuirY(4), Instrucao::Saida]));
        }
        let (resultado, log) = executar(processos, 2);
        assert!(matches!(resultado, Err(ErroEp::EstadoInvalido(_))));
        let esperado = if apos_termino {
            "Carregando P1\nCarregando P2\nExecutando P1\nP1 terminado. X=0. Y=4\nExecutando P2\n"
        } else {
            "Carregando P2\nExecutando P2\n"
        };
        assert_eq!(log, esperado);
    }
}

// Detecta erros de flush ignorados pela integração.
#[cfg(target_os = "linux")]
#[test]
fn escrita_em_dev_full_propaga_erro_de_io() {
    let diretorio = DiretorioTemporario::novo();
    std::os::unix::fs::symlink("/dev/full", diretorio.0.join("log01.txt")).unwrap();
    let mut registro = RegistroLog::criar(&diretorio.0, 1).unwrap();
    assert!(matches!(
        simular(vec![processo(1, 1, &[Instrucao::Saida])], 1, &mut registro),
        Err(ErroEp::Io(_))
    ));
}
