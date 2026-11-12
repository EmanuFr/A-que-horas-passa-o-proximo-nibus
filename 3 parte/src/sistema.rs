// Liga a AVL principal com o índice auxiliar de linhas e implementa R1-R8.
// Cada operação devolve as linhas de resposta já formatadas, no formato de
// saída exigido pelo enunciado.

use crate::avl::{Avl, ResultadoRegistro};
use crate::chegada::{Chave, Chegada};
use crate::indice_linhas::IndiceLinhas;

pub struct Sistema {
    arvore: Avl,
    indice: IndiceLinhas,
}

impl Sistema {
    pub fn novo() -> Self {
        Sistema { arvore: Avl::nova(), indice: IndiceLinhas::novo() }
    }

    // R1
    pub fn registrar_chegada(&mut self, data: &str, horario: &str, linha: &str) -> String {
        let chegada = Chegada {
            data: data.to_string(),
            horario: horario.to_string(),
            linha: linha.to_string(),
            removida: false,
        };

        match self.arvore.registrar(chegada) {
            ResultadoRegistro::Duplicada => "DUPLICADO".to_string(),
            ResultadoRegistro::Inserida | ResultadoRegistro::Reativada => {
                self.indice.registrar(data, horario, linha);
                "OK".to_string()
            }
        }
    }

    // R2
    pub fn buscar_chegada(&self, data: &str, horario: &str, linha: &str) -> String {
        let chave = Chave { data, horario, linha };
        if self.arvore.buscar(chave) {
            "ACHOU".to_string()
        } else {
            "NAO".to_string()
        }
    }

    // R3
    pub fn proximo(&self, data: &str, horario: &str) -> String {
        let minimo = Chave { data, horario, linha: "" };
        match self.arvore.iter_desde(minimo).find(|c| !c.removida) {
            Some(c) => format!("{} {} {}", c.data, c.horario, c.linha),
            None => "NENHUM".to_string(),
        }
    }

    // R4
    pub fn proximo_da_linha(&self, data: &str, horario: &str, linha: &str) -> String {
        match self.indice.proximo_da_linha(data, horario, linha) {
            Some((data_achada, horario_achado)) => format!("{data_achada} {horario_achado}"),
            None => "NENHUM".to_string(),
        }
    }

    // R5: devolve a quantidade seguida das linhas "data horario linha", já
    // no formato de saída (primeira linha = contagem).
    pub fn entre(&self, data1: &str, horario1: &str, data2: &str, horario2: &str) -> Vec<String> {
        let minimo = Chave { data: data1, horario: horario1, linha: "" };
        let fim = (data2, horario2);

        let mut encontrados = Vec::new();
        for c in self.arvore.iter_desde(minimo) {
            if (c.data.as_str(), c.horario.as_str()) > fim {
                break;
            }
            if !c.removida {
                encontrados.push(format!("{} {} {}", c.data, c.horario, c.linha));
            }
        }

        let mut saida = Vec::with_capacity(encontrados.len() + 1);
        saida.push(encontrados.len().to_string());
        saida.extend(encontrados);
        saida
    }

    // R6
    pub fn remover_chegada(&mut self, data: &str, horario: &str, linha: &str) -> String {
        let chave = Chave { data, horario, linha };
        if self.arvore.remover(chave) {
            self.indice.remover(data, horario, linha);
            "OK".to_string()
        } else {
            "NAO".to_string()
        }
    }

    // R7
    pub fn altura(&self) -> String {
        self.arvore.altura().to_string()
    }

    // R8
    pub fn espera_semanal(&self, linha: &str, data: &str, horario: &str, k: u32) -> Vec<String> {
        self.indice
            .espera_semanal(linha, data, horario, k)
            .into_iter()
            .map(|(data_semana, espera)| match espera {
                Some(segundos) => format!("{data_semana} {segundos}"),
                None => format!("{data_semana} NAOPASSOU"),
            })
            .collect()
    }
}
