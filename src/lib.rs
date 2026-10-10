//! Base do EP1; núcleo de escalonamento reservado.
#![forbid(unsafe_code)]
pub mod modelos;
pub mod erros;
pub mod tabela_processos;
pub mod filas;
pub mod politica;
pub mod carregador;
pub mod registro;
pub mod estatisticas;
pub mod experimentos;
pub mod interpretador;
pub mod simulador;
pub mod cli;
