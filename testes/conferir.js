// Confere a saída do programa com a resposta esperada da professora, linha por linha.
// Uso, da raiz do repositório: node testes/conferir.js [1|3]   (sem número = Fase 1)
const fs = require('fs');
const { spawnSync } = require('child_process');

const fase = process.argv[2] === '3' ? 3 : 1;
const linhas = (texto) => texto.trimEnd().split(/\r?\n/);

// executar.txt: 1ª linha compila, 2ª roda a Fase 1, 3ª roda a Fase 3
const executar = fs.readFileSync('executar.txt', 'utf8').split(/\r?\n/);
const comando = executar[fase === 1 ? 1 : 2].replace('{arquivo}', 'testes/pequeno.txt');

const resultado = spawnSync(comando, { shell: true, encoding: 'utf8' });
if (resultado.status !== 0) {
  console.log(`O programa terminou com erro (código ${resultado.status}):`);
  console.log(resultado.stderr);
  process.exit(1);
}

const saida = linhas(resultado.stdout);
const esperado = linhas(fs.readFileSync(`testes/pequeno_esperado_fase${fase}.txt`, 'utf8'));

// Na Fase 3, as respostas de ALT depois da primeira remoção bem-sucedida não são comparadas.
const ignorar = fase === 3 ? altsDepoisDaPrimeiraRemocao() : new Set();

let diferencas = 0;
for (let i = 0; i < Math.max(saida.length, esperado.length); i++) {
  if (ignorar.has(i)) continue;
  if (saida[i] !== esperado[i]) {
    diferencas++;
    console.log(`linha ${i + 1}: esperado "${esperado[i]}", saiu "${saida[i]}"`);
  }
}
console.log(`Fase ${fase}: ${diferencas} diferença(s) em ${esperado.length} linhas esperadas`);
process.exitCode = diferencas === 0 ? 0 : 1;

// Anda pelos comandos do pequeno.txt junto com a resposta esperada para saber
// em que linha cai a resposta de cada ALT.
function altsDepoisDaPrimeiraRemocao() {
  const comandos = fs.readFileSync('testes/pequeno.txt', 'utf8').split(/\r?\n/)
    .filter((l) => l.trim() !== '' && !l.startsWith('#'))
    .map((l) => l.trim().split(/\s+/));
  const posicoes = new Set();
  let i = 0;
  let removeu = false;
  for (const [op, ...args] of comandos) {
    if (op === 'ENTRE') i += 1 + Number(esperado[i]); // a contagem e depois k chegadas
    else if (op === 'SEMANA') i += Number(args[3]); // k linhas, uma por semana
    else {
      if (op === 'ALT' && removeu) posicoes.add(i);
      if (op === 'REM' && esperado[i] === 'OK') removeu = true;
      i += 1;
    }
  }
  return posicoes;
}