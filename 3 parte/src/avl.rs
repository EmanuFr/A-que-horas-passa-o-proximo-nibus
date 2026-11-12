// Árvore AVL própria: guarda todas as chegadas e responde R1 (REG), R2
// (BUSCA), R3 (PROX), R5 (ENTRE), R6 (REM) e R7 (ALT).
//
// Remoção (R6) é lógica: marcamos a chegada como `removida` e não tocamos na
// forma da árvore. O enunciado só exige rebalanceamento na inserção, então
// não há rotação de remoção para implementar; em troca, a árvore nunca
// encolhe e ALT não reflete o número de chegadas ainda ativas (o relatório
// discute esse custo). REG depois de REM só reativa a mesma flag.
//
// Inserção é recursiva: como a AVL garante altura O(log n), a profundidade
// da recursão nunca estoura a pilha (diferente da árvore da Fase 1, que
// degenerava e precisou ser reescrita de forma iterativa). Busca e o
// iterador em-ordem são iterativos, com uma pilha explícita.

use std::cmp::Ordering;

use crate::chegada::{Chave, Chegada};

struct NoAvl {
    chegada: Chegada,
    altura: i32,
    esquerda: Option<Box<NoAvl>>,
    direita: Option<Box<NoAvl>>,
}

pub struct Avl {
    raiz: Option<Box<NoAvl>>,
}

pub enum ResultadoRegistro {
    Inserida,
    Reativada,
    Duplicada,
}

fn altura_no(no: &Option<Box<NoAvl>>) -> i32 {
    match no {
        Some(n) => n.altura,
        None => 0,
    }
}

fn atualizar_altura(no: &mut NoAvl) {
    no.altura = 1 + altura_no(&no.esquerda).max(altura_no(&no.direita));
}

fn fator_balanceamento(no: &NoAvl) -> i32 {
    altura_no(&no.esquerda) - altura_no(&no.direita)
}

// Rotação simples à direita: usada quando a subárvore esquerda está alta
// demais e o filho esquerdo também pende para a esquerda (caso esquerda-
// esquerda).
fn rotacao_direita(mut no: Box<NoAvl>) -> Box<NoAvl> {
    let mut esquerda = no.esquerda.take().expect("rotação direita exige filho esquerdo");
    no.esquerda = esquerda.direita.take();
    atualizar_altura(&mut no);
    esquerda.direita = Some(no);
    atualizar_altura(&mut esquerda);
    esquerda
}

// Rotação simples à esquerda: espelho da anterior (caso direita-direita).
fn rotacao_esquerda(mut no: Box<NoAvl>) -> Box<NoAvl> {
    let mut direita = no.direita.take().expect("rotação esquerda exige filho direito");
    no.direita = direita.esquerda.take();
    atualizar_altura(&mut no);
    direita.esquerda = Some(no);
    atualizar_altura(&mut direita);
    direita
}

// Depois de inserir, o nó pode ter ficado desbalanceado por no máximo 1. Os
// quatro casos clássicos: esquerda-esquerda e direita-direita pedem uma
// rotação simples; esquerda-direita e direita-esquerda pedem uma rotação
// dupla (primeiro endireita o filho, depois rotaciona o nó).
fn balancear(mut no: Box<NoAvl>) -> Box<NoAvl> {
    let fb = fator_balanceamento(&no);

    if fb > 1 {
        if fator_balanceamento(no.esquerda.as_ref().unwrap()) < 0 {
            no.esquerda = Some(rotacao_esquerda(no.esquerda.take().unwrap()));
        }
        rotacao_direita(no)
    } else if fb < -1 {
        if fator_balanceamento(no.direita.as_ref().unwrap()) > 0 {
            no.direita = Some(rotacao_direita(no.direita.take().unwrap()));
        }
        rotacao_esquerda(no)
    } else {
        atualizar_altura(&mut no);
        no
    }
}

fn inserir(no: Option<Box<NoAvl>>, chegada: Chegada) -> (Box<NoAvl>, ResultadoRegistro) {
    let mut atual = match no {
        None => {
            return (
                Box::new(NoAvl {
                    chegada,
                    altura: 1,
                    esquerda: None,
                    direita: None,
                }),
                ResultadoRegistro::Inserida,
            );
        }
        Some(atual) => atual,
    };

    match chegada.chave().cmp(&atual.chegada.chave()) {
        Ordering::Less => {
            let (novo, resultado) = inserir(atual.esquerda.take(), chegada);
            atual.esquerda = Some(novo);
            (balancear(atual), resultado)
        }
        Ordering::Greater => {
            let (novo, resultado) = inserir(atual.direita.take(), chegada);
            atual.direita = Some(novo);
            (balancear(atual), resultado)
        }
        Ordering::Equal => {
            let resultado = if atual.chegada.removida {
                atual.chegada.removida = false;
                ResultadoRegistro::Reativada
            } else {
                ResultadoRegistro::Duplicada
            };
            (atual, resultado)
        }
    }
}

fn empilhar_esquerda<'a>(mut no: Option<&'a NoAvl>, pilha: &mut Vec<&'a NoAvl>) {
    while let Some(atual) = no {
        pilha.push(atual);
        no = atual.esquerda.as_deref();
    }
}

// Desce a árvore empilhando só os nós cuja chave é >= minimo, escolhendo
// esquerda ou direita a cada passo (igual a uma busca por sucessor).
fn descer_desde<'a>(mut no: Option<&'a NoAvl>, minimo: Chave<'_>, pilha: &mut Vec<&'a NoAvl>) {
    while let Some(atual) = no {
        if atual.chegada.chave() >= minimo {
            pilha.push(atual);
            no = atual.esquerda.as_deref();
        } else {
            no = atual.direita.as_deref();
        }
    }
}

pub struct IterDesde<'a> {
    pilha: Vec<&'a NoAvl>,
}

impl<'a> Iterator for IterDesde<'a> {
    type Item = &'a Chegada;

    fn next(&mut self) -> Option<&'a Chegada> {
        let atual = self.pilha.pop()?;
        empilhar_esquerda(atual.direita.as_deref(), &mut self.pilha);
        Some(&atual.chegada)
    }
}

impl Avl {
    pub fn nova() -> Self {
        Avl { raiz: None }
    }

    pub fn registrar(&mut self, chegada: Chegada) -> ResultadoRegistro {
        let (novo, resultado) = inserir(self.raiz.take(), chegada);
        self.raiz = Some(novo);
        resultado
    }

    pub fn buscar(&self, chave: Chave<'_>) -> bool {
        let mut no = self.raiz.as_deref();
        while let Some(atual) = no {
            match chave.cmp(&atual.chegada.chave()) {
                Ordering::Less => no = atual.esquerda.as_deref(),
                Ordering::Greater => no = atual.direita.as_deref(),
                Ordering::Equal => return !atual.chegada.removida,
            }
        }
        false
    }

    // Remoção lógica: acha a chegada e marca removida. Devolve false se a
    // chegada não existe ou já estava removida.
    pub fn remover(&mut self, chave: Chave<'_>) -> bool {
        let mut no = self.raiz.as_deref_mut();
        while let Some(atual) = no {
            match chave.cmp(&atual.chegada.chave()) {
                Ordering::Less => no = atual.esquerda.as_deref_mut(),
                Ordering::Greater => no = atual.direita.as_deref_mut(),
                Ordering::Equal => {
                    if atual.chegada.removida {
                        return false;
                    }
                    atual.chegada.removida = true;
                    return true;
                }
            }
        }
        false
    }

    pub fn altura(&self) -> i32 {
        altura_no(&self.raiz)
    }

    // Itera, em ordem, todas as chegadas (removidas ou não — quem chama
    // filtra) a partir da primeira cuja chave é >= minimo.
    pub fn iter_desde<'a>(&'a self, minimo: Chave<'_>) -> IterDesde<'a> {
        let mut pilha = Vec::new();
        descer_desde(self.raiz.as_deref(), minimo, &mut pilha);
        IterDesde { pilha }
    }
}
