// Fase 2: roda cada arquivo de carga 5 vezes e monta a tabela com a mediana
// do TEMPO_MS e a altura da árvore ao final da execução.
// Uso, da raiz do repositório: node testes/medir.js [1|3]   (sem número = Fase 1)
const fs = require('fs');
const { spawnSync } = require('child_process');

const fase = process.argv[2] === '3' ? 3 : 1;
const EXECUCOES = 5;

// executar.txt: 1ª linha compila, 2ª roda a Fase 1, 3ª roda a Fase 3
const modelo = fs.readFileSync('executar.txt', 'utf8').split(/\r?\n/)[fase === 1 ? 1 : 2];

const cargas = [
  { arquivo: 'carga_quase_ordem_1000.txt', n: 1000 },
  { arquivo: 'carga_quase_ordem_5000.txt', n: 5000 },
  { arquivo: 'carga_quase_ordem_20000.txt', n: 20000 },
  { arquivo: 'carga_embaralhada_1000.txt', n: 1000 },
  { arquivo: 'carga_embaralhada_5000.txt', n: 5000 },
  { arquivo: 'carga_embaralhada_20000.txt', n: 20000 },
];

console.log('| Arquivo de carga | N | Mediana do tempo (ms) | Altura final |');
console.log('|---|---|---|---|');

for (const carga of cargas) {
  const comando = modelo.replace('{arquivo}', `testes/${carga.arquivo}`);
  const tempos = [];
  let altura;

  for (let execucao = 1; execucao <= EXECUCOES; execucao++) {
    // a saída das cargas grandes passa de 1 MB, o limite padrão do spawnSync
    const resultado = spawnSync(comando, { shell: true, encoding: 'utf8', maxBuffer: 64 * 1024 * 1024 });
    const tempo = (resultado.stderr || '').match(/TEMPO_MS (\d+\.\d+)/);
    if (resultado.status !== 0 || !tempo) {
      const motivo = resultado.error ? resultado.error.message
        : resultado.status !== 0 ? resultado.stderr
        : 'não achei "TEMPO_MS valor" na saída de erro';
      console.error(`Falhou em ${carga.arquivo}, execução ${execucao}: ${motivo}`);
      process.exit(1);
    }
    tempos.push(parseFloat(tempo[1]));

    // toda carga termina com ALT, então a altura final é a última linha da saída
    const saida = resultado.stdout.trimEnd().split(/\r?\n/);
    altura = saida[saida.length - 1];
  }

  // sem a função, o sort compara número como texto e "10" vem antes de "9"
  tempos.sort((a, b) => a - b);
  const mediana = tempos[Math.floor(EXECUCOES / 2)];
  console.log(`| ${carga.arquivo} | ${carga.n} | ${mediana.toFixed(3)} | ${altura} |`);
}