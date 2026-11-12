// Lê o arquivo de operações inteiro, executa tudo e só então escreve as
// respostas na saída padrão, no formato exigido pelo enunciado. Mede, por
// fora da leitura e da escrita, só o tempo das operações, e escreve esse
// total na saída de erro como "TEMPO_MS valor" (mesmo contrato do
// script.js da Fase 1).

mod avl;
mod chegada;
mod data;
mod indice_linhas;
mod scapegoat;
mod sistema;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::time::Instant;

use sistema::Sistema;

struct Comando {
    nome: String,
    args: Vec<String>,
}

fn ler_comandos(caminho: &str) -> Vec<Comando> {
    let conteudo = fs::read_to_string(caminho).expect("não consegui ler o arquivo de operações");

    conteudo
        .lines()
        .map(str::trim)
        .filter(|linha| !linha.is_empty() && !linha.starts_with('#'))
        .map(|linha| {
            let mut partes = linha.split_whitespace();
            let nome = partes.next().unwrap_or("").to_uppercase();
            let args = partes.map(str::to_string).collect();
            Comando { nome, args }
        })
        .collect()
}

fn executar(sistema: &mut Sistema, comando: &Comando, saida: &mut Vec<String>) {
    let a = &comando.args;
    match comando.nome.as_str() {
        "REG" => saida.push(sistema.registrar_chegada(&a[0], &a[1], &a[2])),
        "BUSCA" => saida.push(sistema.buscar_chegada(&a[0], &a[1], &a[2])),
        "PROX" => saida.push(sistema.proximo(&a[0], &a[1])),
        "PROXL" => saida.push(sistema.proximo_da_linha(&a[0], &a[1], &a[2])),
        "ENTRE" => saida.extend(sistema.entre(&a[0], &a[1], &a[2], &a[3])),
        "REM" => saida.push(sistema.remover_chegada(&a[0], &a[1], &a[2])),
        "ALT" => saida.push(sistema.altura()),
        "SEMANA" | "ESPERA" => {
            let k: u32 = a[3].parse().unwrap_or(0);
            saida.extend(sistema.espera_semanal(&a[0], &a[1], &a[2], k));
        }
        "LINHAS" => saida.extend(sistema.linhas(&a[0], &a[1], &a[2], &a[3])),
        outro => eprintln!("Comando inválido: {outro}"),
    }
}

fn main() {
    let caminho = env::args().nth(1).expect("uso: fase3 <arquivo-de-operacoes>");
    let comandos = ler_comandos(&caminho);

    let mut sistema = Sistema::novo();
    let mut saida = Vec::new();

    let inicio = Instant::now();
    for comando in &comandos {
        executar(&mut sistema, comando, &mut saida);
    }
    let tempo_total_ms = inicio.elapsed().as_secs_f64() * 1000.0;

    if !saida.is_empty() {
        let mut stdout = io::stdout();
        writeln!(stdout, "{}", saida.join("\n")).expect("falha ao escrever a saída");
    }

    eprintln!("TEMPO_MS {tempo_total_ms:.3}");
}
