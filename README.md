# A que horas passa o próximo ônibus? — Problema 2

Sistema que registra chegadas de ônibus (data, horário e linha) e responde
perguntas sobre elas: se uma chegada já aconteceu, qual a próxima de uma
linha, o que passou entre dois instantes, e quanto se esperou por uma linha
em semanas anteriores. Três fases, cada uma trocando a estrutura de dados
que sustenta essas respostas.

## Linguagens e versões

| Fase | Pasta | Linguagem | Versão usada |
|---|---|---|---|
| 1 | `1 parte/` | JavaScript | Node.js v22.22.3 |
| 3 | `3 parte/` | Rust | rustc/cargo 1.94.0 |

## Estrutura do repositório

- `1 parte/` — Fase 1: árvore binária de busca pura (`script.js`).
- `3 parte/` — Fase 3: AVL própria + índice auxiliar (`Cargo.toml` + `src/`). Detalhes da arquitetura em `3 parte/README.md`.
- `testes/` — compartilhada pelas duas fases: arquivo pequeno com respostas esperadas, arquivos de carga e os scripts `conferir.js` (confere a saída) e `medir.js` (mede tempo e altura).
- `executar.txt` — comandos de compilação e execução exigidos pelo enunciado.
- `medicao_funcionamento.md` — medições e discussão da Fase 2 (árvore da Fase 1 sob carga).
- `medicao_fase3.md` — medições e discussão da Fase 3 (AVL sob carga, limite de altura, comparação com a Fase 1).

## Decisões tomadas

- **Fase 1**: árvore binária de busca sem balanceamento, do zero. Remoção de nó com dois filhos substitui pelo sucessor (menor da subárvore direita). Chave de ordenação: `(data, horario, linha)`, comparada como string — o formato de largura fixa (`AAAA-MM-DD`, `HH:MM:SS`) já dá a ordem cronológica certa sem precisar converter para outro tipo.
- **Fase 3 — árvore principal**: AVL própria, com fator de balanceamento, rotações simples/duplas e rebalanceamento na inserção, como o enunciado exige.
- **Fase 3 — remoção (3.1)**: é lógica (flag "removida"), sem tirar o nó da árvore nem rebalancear. O enunciado só exige rebalanceamento na *inserção*; em troca, a altura nunca diminui depois de uma remoção (por isso o próprio enunciado não compara `ALT` depois da primeira remoção).
- **Fase 3 — R4 e R8 (3.2)**: estrutura auxiliar própria, fora da AVL principal: uma árvore scapegoat de linhas, em que cada nó aponta para outra árvore scapegoat com os instantes daquela linha. Evita varrer todas as chegadas para achar a próxima de uma linha específica. A AVL continua sendo a única árvore que responde `ALT`.
- **Fase 3 — datas**: a subtração de semanas (R8) usa aritmética de calendário escrita à mão (ano bissexto, dias por mês), sem nenhuma biblioteca externa — zero dependências no `Cargo.toml`, para a compilação não depender de internet no computador de correção.
- **Organização em pastas**: cada fase em sua própria pasta, isolada por linguagem; `testes/` fica fora das duas porque os mesmos arquivos de carga e scripts de verificação valem para ambas, incluindo arquivos novos que forem adicionados depois.

## Como rodar

Da raiz do repositório:

```
cargo build --release --manifest-path "3 parte/Cargo.toml"
node "1 parte/script.js" caminho/do/arquivo.txt
"3 parte/target/release/fase3.exe" caminho/do/arquivo.txt
```

(No Windows o binário sai com `.exe`, como acima; em macOS/Linux é
`"3 parte/target/release/fase3"`, sem extensão.)

Ou exatamente como a correção roda, usando o `executar.txt`:

```
node testes/conferir.js 1
node testes/conferir.js 3
```

## Comandos que produzem cada tabela do relatório

| Tabela | Comando |
|---|---|
| Conferir a saída contra o arquivo pequeno | `node testes/conferir.js 1` / `node testes/conferir.js 3` |
| Tempo e altura da Fase 2 (`medicao_funcionamento.md`) | `node testes/medir.js 1` |
| Tempo e altura da Fase 3 (`medicao_fase3.md`) | `node testes/medir.js 3` |
| Memória de pico por carga | ver comando específico em `medicao_fase3.md` |
