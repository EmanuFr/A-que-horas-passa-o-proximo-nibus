// Estrutura auxiliar para R4 (PROXL) e R8 (SEMANA): uma árvore scapegoat de
// linhas, em que cada nó aponta para outra árvore scapegoat com os instantes
// (data, horário) em que aquela linha chegou. Isso evita varrer a AVL
// inteira para achar "a próxima chegada da linha X": primeiro acha-se a
// linha (árvore pequena, uma entrada por linha distinta) e depois busca-se o
// sucessor dentro da árvore só daquela linha.
//
// A cada REG/REM na AVL principal, o mesmo instante é espelhado aqui
// (ver sistema.rs), para as duas estruturas nunca ficarem desatualizadas
// entre si.

use crate::chegada::horario_em_segundos;
use crate::data::subtrair_dias;
use crate::scapegoat::ScapegoatTree;

type Instante = (String, String); // (data, horario)

pub struct IndiceLinhas {
    linhas: ScapegoatTree<String, ScapegoatTree<Instante, ()>>,
}

impl IndiceLinhas {
    pub fn novo() -> Self {
        IndiceLinhas { linhas: ScapegoatTree::nova() }
    }

    fn arvore_da_linha_mut(&mut self, linha: &str) -> &mut ScapegoatTree<Instante, ()> {
        if self.linhas.obter_mut(&linha.to_string()).is_none() {
            self.linhas.inserir(linha.to_string(), ScapegoatTree::nova());
        }
        self.linhas.obter_mut(&linha.to_string()).unwrap()
    }

    pub fn registrar(&mut self, data: &str, horario: &str, linha: &str) {
        self.arvore_da_linha_mut(linha)
            .inserir((data.to_string(), horario.to_string()), ());
    }

    pub fn remover(&mut self, data: &str, horario: &str, linha: &str) -> bool {
        match self.linhas.obter_mut(&linha.to_string()) {
            Some(arvore) => arvore.marcar_removido(&(data.to_string(), horario.to_string())),
            None => false,
        }
    }

    // R4: primeiro instante ativo da linha a partir de (data, horario),
    // inclusive.
    pub fn proximo_da_linha(&self, data: &str, horario: &str, linha: &str) -> Option<(String, String)> {
        let minimo = (data.to_string(), horario.to_string());
        self.linhas
            .obter(&linha.to_string())?
            .iter_desde(&minimo)
            .find(|(_, _, removido)| !removido)
            .map(|(chave, _, _)| chave.clone())
    }

    // R8: para a data informada e as k-1 semanas anteriores (sempre mesmo
    // dia da semana e mesmo horário), quanto tempo quem esperava desde
    // `horario` aguardou pela linha naquele dia. `None` = NAOPASSOU.
    // Resultado em ordem da semana mais antiga para a mais recente.
    pub fn espera_semanal(&self, linha: &str, data: &str, horario: &str, k: u32) -> Vec<(String, Option<i64>)> {
        let arvore = self.linhas.obter(&linha.to_string());
        let segundos_referencia = horario_em_segundos(horario);
        let mut resultados = Vec::with_capacity(k as usize);

        for semanas_atras in (0..k).rev() {
            let data_semana = subtrair_dias(data, 7 * semanas_atras);

            let espera = arvore.and_then(|arvore| {
                let minimo = (data_semana.clone(), horario.to_string());
                arvore
                    .iter_desde(&minimo)
                    .find(|(_, _, removido)| !removido)
                    .filter(|(chave, _, _)| chave.0 == data_semana)
                    .map(|(chave, _, _)| horario_em_segundos(&chave.1) - segundos_referencia)
            });

            resultados.push((data_semana, espera));
        }

        resultados
    }
}
