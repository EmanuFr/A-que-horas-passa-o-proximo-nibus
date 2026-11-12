# Fase 3 — AVL + índice de linhas (Rust)

Reimplementação do Problema 2 em Rust. Troca a árvore binária da Fase 1 por
uma **AVL própria** e acrescenta uma **estrutura auxiliar própria** para R4 e
R8, para não depender de varrer a árvore inteira nessas duas operações.
Zero dependências externas (`Cargo.toml` não lista nenhuma) e nenhuma
estrutura de dados pronta da linguagem no núcleo — só `Box`, `Option` e `Vec`
simples, como permitido pelo enunciado.

## Arquitetura

```
Sistema (sistema.rs)
├── Avl (avl.rs)                         — todas as chegadas, responde R1 R2 R3 R5 R6 R7
└── IndiceLinhas (indice_linhas.rs)      — responde R4 e R8
    └── ScapegoatTree<linha, ScapegoatTree<instante, ()>>  (scapegoat.rs)
```

- **`avl.rs`**: a árvore principal, exigida pelo enunciado. Fator de
  balanceamento, rotações simples e duplas, rebalanceamento na inserção.
  Chave de ordenação: `(data, horario, linha)`, comparados como string — como
  o formato é sempre `AAAA-MM-DD` e `HH:MM:SS` (largura fixa), ordem de string
  já é ordem cronológica.

- **`scapegoat.rs`**: árvore scapegoat própria, genérica, usada duas vezes
  para montar a estrutura de R4/R8 (não é a árvore principal — essa
  continua sendo só a AVL, como o enunciado pede). Guarda só os dados nos
  nós, sem altura nem fator de balanceamento; o equilíbrio é por peso
  (tamanho das subárvores) e só é checado quando necessário.

- **`indice_linhas.rs`**: uma árvore scapegoat de linhas, em que cada nó
  aponta para outra árvore scapegoat com os instantes daquela linha. Achar
  "a próxima chegada da linha X" vira: achar a linha (árvore pequena) e
  buscar o sucessor só dentro dela, em vez de varrer todas as chegadas.

- **`data.rs`**: aritmética de calendário própria (dias por mês, ano
  bissexto), só para o R8 recuar semanas sem precisar de biblioteca de
  datas.

- **`chegada.rs`**: o registro de uma chegada e a chave de comparação.

- **`main.rs`**: lê o arquivo inteiro, executa os comandos medindo só o
  tempo das operações, e escreve as respostas no final — mesmo contrato do
  `script.js` da Fase 1.

## Decisões de projeto

- **Remoção (R6) na AVL é lógica**: `REM` marca a chegada como removida, sem
  tirar o nó nem rebalancear. O enunciado só exige rebalanceamento na
  *inserção*, então não existe rotação de remoção aqui. Em troca, a árvore
  nunca encolhe: a altura (R7) depois da primeira remoção reflete o maior
  tamanho que a árvore já teve, não o número de chegadas ainda ativas — por
  isso o próprio enunciado não compara `ALT` depois da primeira remoção.
  `REG` depois de `REM` só reativa a mesma flag, sem inserir de novo.

- **Remoção na árvore scapegoat é diferente**: também marca uma flag, mas
  quando mais da metade dos nós físicos de uma árvore já são "lápides", ela
  é reconstruída do zero só com os nós ativos. É o mecanismo padrão de
  scapegoat tree — mantém o custo amortizado logarítmico mesmo com muitas
  remoções, o que a AVL (com remoção lógica) não garante a longo prazo.

- **Duas estruturas, uma fonte da verdade cada uma**: a AVL e o índice de
  linhas guardam a mesma informação (os mesmos instantes) de forma
  independente, sem ponteiro compartilhado entre elas — mais simples em
  Rust do que compartilhar um nó mutável entre duas árvores. Todo `REG` e
  todo `REM` atualiza as duas (ver `Sistema::registrar_chegada` e
  `Sistema::remover_chegada`).

## Compilar e rodar

Da raiz do repositório, como no `executar.txt`:

```
cargo build --release --manifest-path "3 parte/Cargo.toml"
"3 parte/target/release/fase3" testes/pequeno.txt
```

(No Windows, o binário sai em `3 parte\target\release\fase3.exe`, que é o
caminho usado no `executar.txt` da raiz.)

Testes de `data.rs`:

```
cargo test --manifest-path "3 parte/Cargo.toml"
```

Conferir contra a resposta esperada / medir tempo e altura nos arquivos de
carga (da raiz do repositório):

```
node testes/conferir.js 3
node testes/medir.js 3
```
