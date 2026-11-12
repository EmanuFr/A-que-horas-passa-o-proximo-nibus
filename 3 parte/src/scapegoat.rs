// Árvore scapegoat própria, genérica em chave e valor. É a estrutura
// auxiliar (fora do núcleo obrigatório de AVL) usada para R4 e R8: o índice
// de linhas é uma ScapegoatTree<String, ScapegoatTree<(String, String), ()>>
// — uma árvore de linhas que aponta para outra árvore, com as chegadas
// daquela linha (ver indice_linhas.rs).
//
// Diferença chave com a AVL: o nó não guarda altura nem fator de
// balanceamento, só os dados. O balanceamento é por peso (tamanho das
// subárvores), checado só quando necessário:
//
// - Inserção: ao subir da folha recém-inserida até a raiz, o primeiro
//   ancestral cuja subárvore maior passa de ALFA do tamanho total é o
//   "bode expiatório" (scapegoat): sua subárvore inteira é achatada (percurso
//   em ordem) e reconstruída perfeitamente balanceada. Isso já garante que
//   todos os ancestrais acima também ficam balanceados, então paramos ali.
// - Remoção: é lógica (marca `removido`); quando mais da metade dos nós
//   físicos da árvore já são "lápides", a árvore inteira é reconstruída
//   mantendo só os nós ativos.
//
// Como a invariante de peso nunca deixa a altura passar de
// log(1/ALFA) do tamanho, a profundidade é sempre O(log n) e as funções
// recursivas abaixo (achatar, construir, contar) nunca arriscam estourar a
// pilha de chamadas.

use std::cmp::Ordering;

const ALFA: f64 = 2.0 / 3.0;

struct No<K, V> {
    chave: K,
    valor: V,
    removido: bool,
    esquerda: Option<Box<No<K, V>>>,
    direita: Option<Box<No<K, V>>>,
}

pub struct ScapegoatTree<K, V> {
    raiz: Option<Box<No<K, V>>>,
    tamanho: usize,
    tamanho_maximo: usize,
}

fn tamanho_subarvore<K, V>(no: &Option<Box<No<K, V>>>) -> usize {
    match no {
        None => 0,
        Some(n) => 1 + tamanho_subarvore(&n.esquerda) + tamanho_subarvore(&n.direita),
    }
}

fn obter_no<'a, K: Ord, V>(raiz: &'a Option<Box<No<K, V>>>, chave: &K) -> Option<&'a No<K, V>> {
    let mut no = raiz.as_deref();
    while let Some(atual) = no {
        match chave.cmp(&atual.chave) {
            Ordering::Less => no = atual.esquerda.as_deref(),
            Ordering::Greater => no = atual.direita.as_deref(),
            Ordering::Equal => return Some(atual),
        }
    }
    None
}

fn obter_no_mut<'a, K: Ord, V>(raiz: &'a mut Option<Box<No<K, V>>>, chave: &K) -> Option<&'a mut No<K, V>> {
    let mut no = raiz.as_deref_mut();
    while let Some(atual) = no {
        match chave.cmp(&atual.chave) {
            Ordering::Less => no = atual.esquerda.as_deref_mut(),
            Ordering::Greater => no = atual.direita.as_deref_mut(),
            Ordering::Equal => return Some(atual),
        }
    }
    None
}

// Percurso em ordem, consumindo a subárvore. Com `apenas_ativos`, descarta os
// nós marcados como removidos (usado na reconstrução total após remoções).
fn achatar<K, V>(no: Option<Box<No<K, V>>>, saida: &mut Vec<Option<No<K, V>>>, apenas_ativos: bool) {
    if let Some(no) = no {
        let No { chave, valor, removido, esquerda, direita } = *no;
        achatar(esquerda, saida, apenas_ativos);
        if !apenas_ativos || !removido {
            saida.push(Some(No { chave, valor, removido, esquerda: None, direita: None }));
        }
        achatar(direita, saida, apenas_ativos);
    }
}

// Reconstrói uma árvore perfeitamente balanceada a partir de uma lista já em
// ordem: o elemento do meio vira raiz, e as duas metades viram as subárvores,
// recursivamente.
fn construir_balanceado<K, V>(nos: &mut [Option<No<K, V>>]) -> Option<Box<No<K, V>>> {
    if nos.is_empty() {
        return None;
    }
    let meio = nos.len() / 2;
    let esquerda = construir_balanceado(&mut nos[..meio]);
    let mut no_meio = nos[meio].take().expect("nó do meio presente");
    let direita = construir_balanceado(&mut nos[meio + 1..]);
    no_meio.esquerda = esquerda;
    no_meio.direita = direita;
    Some(Box::new(no_meio))
}

fn reconstruir<K, V>(no: Box<No<K, V>>) -> Box<No<K, V>> {
    let mut lista = Vec::new();
    achatar(Some(no), &mut lista, false);
    construir_balanceado(&mut lista).expect("subárvore não pode ficar vazia")
}

fn verificar_e_reconstruir<K, V>(no: Box<No<K, V>>) -> (Box<No<K, V>>, bool) {
    let tam_esq = tamanho_subarvore(&no.esquerda);
    let tam_dir = tamanho_subarvore(&no.direita);
    let tam_total = (tam_esq + tam_dir + 1) as f64;

    let desbalanceado = tam_esq as f64 > ALFA * tam_total || tam_dir as f64 > ALFA * tam_total;

    if desbalanceado {
        (reconstruir(no), true)
    } else {
        (no, false)
    }
}

// Insere assumindo que `chave` ainda não existe na subárvore (quem chama já
// tratou o caso de chave já existente). Devolve, além da nova subárvore, se
// algum ancestral no caminho já foi reconstruído (nesse caso os ancestrais
// acima não precisam ser verificados: a reconstrução já garante o equilíbrio
// deles também).
fn inserir_no<K: Ord, V>(no: Option<Box<No<K, V>>>, chave: K, valor: V) -> (Box<No<K, V>>, bool) {
    let mut atual = match no {
        None => {
            return (
                Box::new(No { chave, valor, removido: false, esquerda: None, direita: None }),
                false,
            );
        }
        Some(atual) => atual,
    };

    let ja_parou = if chave < atual.chave {
        let (novo, parou) = inserir_no(atual.esquerda.take(), chave, valor);
        atual.esquerda = Some(novo);
        parou
    } else {
        let (novo, parou) = inserir_no(atual.direita.take(), chave, valor);
        atual.direita = Some(novo);
        parou
    };

    if ja_parou {
        (atual, true)
    } else {
        verificar_e_reconstruir(atual)
    }
}

fn empilhar_esquerda<'a, K, V>(mut no: Option<&'a No<K, V>>, pilha: &mut Vec<&'a No<K, V>>) {
    while let Some(atual) = no {
        pilha.push(atual);
        no = atual.esquerda.as_deref();
    }
}

fn descer_desde<'a, K: Ord, V>(mut no: Option<&'a No<K, V>>, minimo: &K, pilha: &mut Vec<&'a No<K, V>>) {
    while let Some(atual) = no {
        if &atual.chave >= minimo {
            pilha.push(atual);
            no = atual.esquerda.as_deref();
        } else {
            no = atual.direita.as_deref();
        }
    }
}

pub struct IterDesde<'a, K, V> {
    pilha: Vec<&'a No<K, V>>,
}

impl<'a, K, V> Iterator for IterDesde<'a, K, V> {
    type Item = (&'a K, &'a V, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let atual = self.pilha.pop()?;
        empilhar_esquerda(atual.direita.as_deref(), &mut self.pilha);
        Some((&atual.chave, &atual.valor, atual.removido))
    }
}

impl<K: Ord + Clone, V> ScapegoatTree<K, V> {
    pub fn nova() -> Self {
        ScapegoatTree { raiz: None, tamanho: 0, tamanho_maximo: 0 }
    }

    // Insere `chave` com `valor`. Se a chave já existe (mesmo removida),
    // reaproveita o nó físico: reativa a flag e troca o valor, sem disparar
    // nenhuma reconstrução (a forma da árvore não mudou).
    pub fn inserir(&mut self, chave: K, valor: V) {
        if let Some(no) = obter_no_mut(&mut self.raiz, &chave) {
            if no.removido {
                no.removido = false;
                self.tamanho += 1;
            }
            no.valor = valor;
            return;
        }

        let (nova_raiz, _) = inserir_no(self.raiz.take(), chave, valor);
        self.raiz = Some(nova_raiz);
        self.tamanho += 1;
        self.tamanho_maximo += 1;
    }

    pub fn obter(&self, chave: &K) -> Option<&V> {
        obter_no(&self.raiz, chave).and_then(|no| if no.removido { None } else { Some(&no.valor) })
    }

    pub fn obter_mut(&mut self, chave: &K) -> Option<&mut V> {
        obter_no_mut(&mut self.raiz, chave).and_then(|no| if no.removido { None } else { Some(&mut no.valor) })
    }

    // Remoção lógica. Quando mais da metade dos nós físicos vira lápide,
    // reconstrói a árvore inteira só com os ativos.
    pub fn marcar_removido(&mut self, chave: &K) -> bool {
        let encontrou = match obter_no_mut(&mut self.raiz, chave) {
            Some(no) if !no.removido => {
                no.removido = true;
                true
            }
            _ => false,
        };

        if encontrou {
            self.tamanho -= 1;
            if self.tamanho <= self.tamanho_maximo / 2 {
                let mut lista = Vec::new();
                achatar(self.raiz.take(), &mut lista, true);
                self.tamanho_maximo = lista.len();
                self.raiz = construir_balanceado(&mut lista);
            }
        }

        encontrou
    }

    // Percorre, em ordem, a partir da primeira chave >= minimo. Cada item
    // vem com a flag `removido`; quem chama decide se pula.
    pub fn iter_desde<'a>(&'a self, minimo: &K) -> IterDesde<'a, K, V> {
        let mut pilha = Vec::new();
        descer_desde(self.raiz.as_deref(), minimo, &mut pilha);
        IterDesde { pilha }
    }
}
