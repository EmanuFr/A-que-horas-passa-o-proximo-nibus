# Fase 3: Troque a estrutura e meça a diferença

> Executem a AVL com os mesmos arquivos de carga da Fase 2, confiram se a
> altura final respeita o limite da AVL, altura ≤ 1,45 × log₂(n + 2), em que
> n é o número de nós da árvore ao final da execução, e apresentem a
> comparação lado a lado com a Fase 1, em tempo, altura e memória.

Todas as medições abaixo foram feitas na mesma máquina, nos mesmos seis
arquivos de carga da Fase 2 (`testes/carga_*.txt`), para que a comparação
Fase 1 × Fase 3 seja justa (sem misturar tempos de computadores diferentes).

## Tempo e altura (AVL)

Comando (da raiz do repositório): `node testes/medir.js 3` — roda cada
arquivo 5 vezes e guarda a mediana do tempo e a altura final, igual à
Fase 2.

| Arquivo de carga | N | Mediana do tempo (ms) | Altura final |
|---|---|---|---|
| carga_quase_ordem_1000.txt | 1000 | 0.947 | 11 |
| carga_quase_ordem_5000.txt | 5000 | 9.331 | 13 |
| carga_quase_ordem_20000.txt | 20000 | 94.688 | 15 |
| carga_embaralhada_1000.txt | 1000 | 0.909 | 12 |
| carga_embaralhada_5000.txt | 5000 | 9.717 | 15 |
| carga_embaralhada_20000.txt | 20000 | 111.016 | 17 |

## Verificação do limite de altura da AVL (3.3)

altura ≤ 1,45 × log₂(n + 2), com n = número de nós ao final da execução
(aqui, n = N, já que as cargas só têm `REG` e todas as chegadas são
distintas).

| n | log₂(n+2) | limite (1,45×log₂(n+2)) | altura obtida (quase ordem) | altura obtida (embaralhada) | dentro do limite? |
|---|---|---|---|---|---|
| 1000 | 9,97 | 14,45 | 11 | 12 | sim, nos dois |
| 5000 | 12,29 | 17,82 | 13 | 15 | sim, nos dois |
| 20000 | 14,29 | 20,72 | 15 | 17 | sim, nos dois |

## Memória de pico

Comando usado nesta máquina (macOS), uma execução por arquivo:

```
/usr/bin/time -l "3 parte/target/release/fase3" testes/<arquivo>.txt > /dev/null
```

(a saída traz uma linha `... peak memory footprint`, em bytes). No Windows
não existe um equivalente direto a `/usr/bin/time -l`; uma alternativa é
rodar o processo e observar o pico de "memória de trabalho" no Gerenciador
de Tarefas, ou usar `Get-Process` do PowerShell enquanto o programa roda.

| Arquivo de carga | Memória de pico — Fase 3 (Rust) | Memória de pico — Fase 1 (JS) |
|---|---|---|
| carga_quase_ordem_1000.txt | 1,95 MB | 18,67 MB |
| carga_quase_ordem_5000.txt | 4,31 MB | 27,55 MB |
| carga_quase_ordem_20000.txt | 12,85 MB | 70,71 MB |
| carga_embaralhada_1000.txt | 1,89 MB | 18,37 MB |
| carga_embaralhada_5000.txt | 4,32 MB | 37,67 MB |
| carga_embaralhada_20000.txt | 12,82 MB | 85,80 MB |

A diferença de memória mistura dois fatores: a estrutura de dados (AVL +
índice auxiliar vs. BST pura) e o runtime da linguagem (um binário Rust
compilado vs. a máquina virtual V8 do Node, que já parte de uma base de
memória maior mesmo para entradas pequenas). Não dá para atribuir toda a
diferença só à árvore.

## Comparação lado a lado — Fase 1 × Fase 3 (mesma máquina)

| Arquivo de carga | N | Tempo F1 (ms) | Tempo F3 (ms) | Razão F1/F3 | Altura F1 | Altura F3 | Memória F1 | Memória F3 |
|---|---|---|---|---|---|---|---|---|
| carga_quase_ordem_1000.txt | 1000 | 47.564 | 0.947 | 50,2× | 889 | 11 | 18,67 MB | 1,95 MB |
| carga_quase_ordem_5000.txt | 5000 | 840.360 | 9.331 | 90,1× | 4458 | 13 | 27,55 MB | 4,31 MB |
| carga_quase_ordem_20000.txt | 20000 | 13063.304 | 94.688 | 138,0× | 17750 | 15 | 70,71 MB | 12,85 MB |
| carga_embaralhada_1000.txt | 1000 | 24.010 | 0.909 | 26,4× | 22 | 12 | 18,37 MB | 1,89 MB |
| carga_embaralhada_5000.txt | 5000 | 1261.364 | 9.717 | 129,8× | 30 | 15 | 37,67 MB | 4,32 MB |
| carga_embaralhada_20000.txt | 20000 | 91063.671 | 111.016 | 820,0× | 32 | 17 | 85,80 MB | 12,82 MB |

(Tempos da Fase 1 nesta mesma máquina, via `node testes/medir.js 1` — os
valores da tabela média de três computadores em `medicao_funcionamento.md`
são de outra medição, com outras máquinas; por isso não são os mesmos
números usados aqui.)

Como a razão F1/F3 mistura a troca de algoritmo (BST sem balanceamento →
AVL) com a troca de linguagem (JavaScript → Rust), ela não isola "quanto
veio da árvore"; serve como ordem de grandeza, não como medida pura do
ganho da estrutura.

## Reproduzir em mais computadores

A Fase 2 comparou três computadores do grupo. Para repetir isso aqui, em
cada computador, da raiz do repositório:

```
cargo build --release --manifest-path "3 parte/Cargo.toml"
node testes/medir.js 3
```

e colar a tabela de saída numa nova seção "Computador N" neste arquivo,
igual ao que foi feito em `medicao_funcionamento.md` para a Fase 2.

## Discussão (completar)

As perguntas abaixo são do enunciado e pedem a leitura do grupo sobre os
números acima — não foram respondidas aqui de propósito, para ficarem de
autoria do grupo:

- **(3.1)** quanto custa a forma de remoção escolhida para a AVL, e do que o
  grupo abre mão ao escolher essa forma?
- **(3.2)** quanto custam R4 e R8 com a estrutura auxiliar, esse custo
  cresce com o tempo, e o que precisa ser atualizado a cada `REG`/`REM` para
  as duas estruturas não ficarem desatualizadas entre si?
- **(i)** leitura da tabela comparativa: o que a razão de tempo, a
  diferença de altura e a diferença de memória mostram, e o que não
  mostram?
- **(j)** a verificação do limite de altura (seção acima) se sustenta nos
  seis arquivos — o que isso diz sobre a garantia da AVL mesmo em carga
  quase ordenada, onde a Fase 1 degenerava?
