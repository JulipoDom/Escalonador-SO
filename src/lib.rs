//! Base do EP1; núcleo de escalonamento reservado.
#![forbid(unsafe_code)]
pub mod carregador;
pub mod cli;
pub mod erros;
pub mod estatisticas;
pub mod experimentos;
pub mod filas;
pub mod interpretador;
pub mod modelos;
pub mod politica;
pub mod registro;
pub mod simulador;
pub mod tabela_processos;
