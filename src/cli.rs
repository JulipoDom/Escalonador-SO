//! Argumentos de terminal e coordenação, sem executar o núcleo reservado.
use crate::{
    carregador::{carregar_programas, ler_quantum},
    erros::ErroEp,
    experimentos::executar_experimentos,
};
use std::{
    ffi::{OsStr, OsString},
    path::PathBuf,
};

const AJUDA: &str = "Uso: Escalonador [--validar] [--programas CAMINHO] [--saida CAMINHO]\n     Escalonador --quanta N ... [--programas CAMINHO] [--saida CAMINHO]\n     Escalonador --help\n\n--validar mostra nome, prioridade, instruções e quantum, sem simular.\n--programas seleciona entradas; padrão: programas/ ao lado do executável, depois no diretório atual.\n--saida seleciona pasta do log individual logXX.txt (padrão: pai de programas/).\n--quanta seleciona inteiros positivos para lote, sem alterar quantum.txt.\nNo lote, --saida é a raiz de logs/ e tabelas/ (padrão: pai de programas/resultados/).\nSimulação e lote ainda não implementados; não publicam resultados.\n";

fn entrada(mensagem: impl Into<String>) -> ErroEp {
    ErroEp::Entrada(mensagem.into())
}
fn definir<T: PartialEq>(campo: &mut Option<T>, valor: T, opcao: &str) -> Result<(), ErroEp> {
    if campo.as_ref().is_some_and(|anterior| anterior != &valor) {
        return Err(entrada(format!("{opcao} repetida com valores diferentes")));
    }
    *campo = Some(valor);
    Ok(())
}
fn opcao(valor: &OsStr) -> bool {
    valor.to_string_lossy().starts_with("--")
}

/// Recebe argumentos sem o nome do executável, preservando caminhos não UTF-8.
pub fn executar(args: impl Iterator<Item = OsString>) -> Result<(), ErroEp> {
    let mut args = args.peekable();
    let (mut programas, mut saida, mut quanta) = (None, None, None);
    let (mut validar, mut ajuda) = (false, false);
    while let Some(arg) = args.next() {
        match arg.to_str() {
            Some("--help") => ajuda = true,
            Some("--validar") => validar = true,
            Some(nome @ ("--programas" | "--saida")) => {
                let valor = args
                    .next()
                    .filter(|v| !opcao(v))
                    .ok_or_else(|| entrada(format!("{nome} exige CAMINHO")))?;
                let campo = if nome == "--programas" {
                    &mut programas
                } else {
                    &mut saida
                };
                definir(campo, PathBuf::from(valor), nome)?;
            }
            Some("--quanta") => {
                let mut valores = Vec::new();
                while args.peek().is_some_and(|v| !opcao(v)) {
                    let valor = args.next().expect("peek confirmou argumento");
                    let n = valor
                        .to_str()
                        .and_then(|v| v.parse::<usize>().ok())
                        .filter(|n| *n > 0)
                        .ok_or_else(|| {
                            entrada(format!(
                                "--quanta exige inteiro positivo: {}",
                                valor.to_string_lossy()
                            ))
                        })?;
                    valores.push(n);
                }
                if valores.is_empty() {
                    return Err(entrada("--quanta exige ao menos um inteiro positivo"));
                }
                definir(&mut quanta, valores, "--quanta")?;
            }
            _ => {
                return Err(entrada(format!(
                    "opção desconhecida: {}",
                    arg.to_string_lossy()
                )))
            }
        }
    }
    if validar && quanta.is_some() {
        return Err(entrada("--validar e --quanta são mutuamente exclusivos"));
    }
    if ajuda {
        print!("{AJUDA}");
        return Ok(());
    }
    let diretorio = match programas {
        Some(p) => p,
        None => {
            let vizinho = std::env::current_exe()?
                .parent()
                .map(|p| p.join("programas"));
            if let Some(p) = vizinho.filter(|p| p.is_dir()) {
                p
            } else {
                let p = std::env::current_dir()?.join("programas");
                if !p.is_dir() {
                    return Err(entrada("entradas não encontradas; use --programas CAMINHO"));
                }
                p
            }
        }
    };
    let raiz = diretorio
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| std::path::Path::new("."));
    let processos = carregar_programas(&diretorio)?;
    if let Some(quanta) = quanta {
        let destino = saida.unwrap_or_else(|| raiz.join("resultados"));
        executar_experimentos(&diretorio, &quanta, &destino)?;
        return Ok(());
    }
    let quantum = ler_quantum(&diretorio)?;
    if validar {
        let mut total = 0;
        for bcp in &processos {
            let n = bcp.programa.instrucoes.len();
            total += n;
            println!(
                "{}: prioridade {}, instruções {}",
                bcp.programa.nome, bcp.prioridade, n
            );
        }
        println!(
            "quantum: {quantum}\nTotal: {} processos, {total} instruções",
            processos.len()
        );
        return Ok(());
    }
    let _destino = saida.unwrap_or_else(|| raiz.to_path_buf());
    // Não abrir RegistroLog: simular é um stub; criar o escritor publicaria log vazio.
    Err(ErroEp::NaoImplementado(
        "laço de simulação, despacho e transições de processos",
    ))
}
