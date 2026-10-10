//! Leitura e validação dos programas e configurações, sem executar instruções.
use crate::{
    erros::ErroEp,
    modelos::{Bcp, Instrucao, ProcessId, Programa},
};
use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::Path,
};

fn entrada(path: &Path, linha: usize, mensagem: &str) -> ErroEp {
    ErroEp::Entrada(format!("{}:{linha}: {mensagem}", path.display()))
}
fn erro_io(path: &Path, linha: Option<usize>, erro: io::Error) -> ErroEp {
    let local = linha
        .map(|n| format!("{}:{n}", path.display()))
        .unwrap_or_else(|| path.display().to_string());
    ErroEp::Io(io::Error::new(erro.kind(), format!("{local}: {erro}")))
}
fn linhas(path: &Path) -> Result<Vec<String>, ErroEp> {
    let arquivo = File::open(path).map_err(|e| erro_io(path, None, e))?;
    BufReader::new(arquivo)
        .lines()
        .enumerate()
        .map(|(i, linha)| linha.map_err(|e| erro_io(path, Some(i + 1), e)))
        .collect()
}
fn inteiro_decimal(texto: &str) -> bool {
    !texto.is_empty() && texto.bytes().all(|b| b.is_ascii_digit())
}
fn instrucao(texto: &str) -> Option<Instrucao> {
    match texto {
        "COM" => Some(Instrucao::Com),
        "E/S" => Some(Instrucao::EntradaSaida),
        "SAIDA" => Some(Instrucao::Saida),
        _ => {
            let (registrador, valor) = texto.split_once('=')?;
            let digitos = valor
                .strip_prefix('-')
                .or_else(|| valor.strip_prefix('+'))
                .unwrap_or(valor);
            if !inteiro_decimal(digitos) {
                return None;
            }
            let valor = valor.parse::<i64>().ok()?;
            match registrador {
                "X" => Some(Instrucao::AtribuirX(valor)),
                "Y" => Some(Instrucao::AtribuirY(valor)),
                _ => None,
            }
        }
    }
}

/// Carrega arquivos ASCII NN.txt em ordem de nome, mantendo prioridades e PIDs nessa ordem.
pub fn carregar_programas(diretorio: &Path) -> Result<Vec<Bcp>, ErroEp> {
    let entradas = fs::read_dir(diretorio).map_err(|e| erro_io(diretorio, None, e))?;
    let mut nomes = Vec::new();
    for item in entradas {
        let item = item.map_err(|e| erro_io(diretorio, None, e))?;
        let nome = item.file_name();
        let Some(nome) = nome.to_str() else {
            continue;
        };
        let bytes = nome.as_bytes();
        if bytes.len() == 6
            && bytes[0].is_ascii_digit()
            && bytes[1].is_ascii_digit()
            && &bytes[2..] == b".txt"
            && item
                .file_type()
                .map_err(|e| erro_io(&item.path(), None, e))?
                .is_file()
        {
            nomes.push(nome.to_owned());
        }
    }
    nomes.sort();
    if nomes.is_empty() {
        return Err(entrada(diretorio, 1, "nenhum programa NN.txt encontrado"));
    }
    let arquivo_prioridades = diretorio.join("prioridades.txt");
    let prioridades = linhas(&arquivo_prioridades)?;
    if prioridades.len() != nomes.len() {
        return Err(entrada(
            &arquivo_prioridades,
            prioridades.len().min(nomes.len()) + 1,
            "exigida exatamente uma prioridade por programa",
        ));
    }
    let prioridades: Vec<u32> = prioridades
        .iter()
        .enumerate()
        .map(|(i, valor)| {
            if !inteiro_decimal(valor) {
                return Err(entrada(
                    &arquivo_prioridades,
                    i + 1,
                    "prioridade deve ser inteiro u32 não negativo",
                ));
            }
            valor.parse().map_err(|_| {
                entrada(
                    &arquivo_prioridades,
                    i + 1,
                    "prioridade fora do intervalo u32",
                )
            })
        })
        .collect::<Result<_, _>>()?;
    nomes
        .into_iter()
        .zip(prioridades)
        .enumerate()
        .map(|(i, (arquivo, prioridade))| {
            let path = diretorio.join(&arquivo);
            let linhas = linhas(&path)?;
            let nome = linhas
                .first()
                .filter(|n| !n.trim().is_empty())
                .ok_or_else(|| entrada(&path, 1, "nome de programa vazio"))?;
            let textos = &linhas[1..];
            if textos.is_empty() {
                return Err(entrada(
                    &path,
                    2,
                    "exigida ao menos uma instrução e SAIDA final",
                ));
            }
            if textos.len() > 21 {
                return Err(entrada(&path, 23, "limite de 21 instruções excedido"));
            }
            let mut instrucoes = Vec::with_capacity(textos.len());
            for (j, texto) in textos.iter().enumerate() {
                let instrucao = instrucao(texto).ok_or_else(|| {
                    entrada(
                        &path,
                        j + 2,
                        "instrução inválida ou inteiro fora do intervalo i64",
                    )
                })?;
                if instrucao == Instrucao::Saida && j + 1 != textos.len() {
                    return Err(entrada(&path, j + 2, "SAIDA deve ser única e final"));
                }
                instrucoes.push(instrucao);
            }
            if instrucoes.last() != Some(&Instrucao::Saida) {
                return Err(entrada(
                    &path,
                    linhas.len(),
                    "última instrução deve ser SAIDA",
                ));
            }
            let programa = Programa {
                nome: nome.to_owned(),
                arquivo,
                instrucoes: instrucoes.into_boxed_slice(),
            };
            Ok(Bcp::novo(ProcessId((i + 1) as u32), programa, prioridade))
        })
        .collect()
}

/// Lê um único inteiro decimal positivo em quantum.txt.
pub fn ler_quantum(diretorio: &Path) -> Result<usize, ErroEp> {
    let path = diretorio.join("quantum.txt");
    let valores = linhas(&path)?;
    if valores.len() != 1 {
        return Err(entrada(
            &path,
            if valores.len() > 1 { 2 } else { 1 },
            "exigido um único inteiro positivo",
        ));
    }
    let valor = &valores[0];
    if !inteiro_decimal(valor) {
        return Err(entrada(&path, 1, "quantum deve ser inteiro positivo usize"));
    }
    let quantum = valor
        .parse::<usize>()
        .map_err(|_| entrada(&path, 1, "quantum fora do intervalo usize"))?;
    if quantum == 0 {
        return Err(entrada(&path, 1, "quantum deve ser positivo"));
    }
    Ok(quantum)
}
