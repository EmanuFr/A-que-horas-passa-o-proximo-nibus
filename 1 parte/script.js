const chave = (r) => `${r.data} ${r.horario}`;

const registrarChegada = (data, horario, linha) => {
  const chegada = { data, horario, linha };

    if (buscarNaArvore(chegada) !== null) {
    return false;
  }

  raiz = inserirNaArvore(raiz, chegada);

  return true;
};

let raiz = null;

const compararChegadas = function (a, b) {
  const instanteA = `${a.data} ${a.horario}`;
  const instanteB = `${b.data} ${b.horario}`;

  if (instanteA < instanteB) return -1;
  if (instanteA > instanteB) return 1;

  if (a.linha < b.linha) return -1;
  if (a.linha > b.linha) return 1;

  return 0;
};

// Versão iterativa (sem recursão): com chegadas quase em ordem a árvore
// degenera e fica com altura próxima de n, o que faria uma versão recursiva
// estourar a pilha de chamadas do Node para n grande (ver carga_quase_ordem).
const inserirNaArvore = function (no, chegada) {
  if (no === null) {
    return {
      chegada: chegada,
      esquerda: null,
      direita: null,
    };
  }

  let atual = no;

  while (true) {
    const comparacao = compararChegadas(chegada, atual.chegada);

    if (comparacao < 0) {
      if (atual.esquerda === null) {
        atual.esquerda = { chegada: chegada, esquerda: null, direita: null };
        break;
      }
      atual = atual.esquerda;
    } else if (comparacao > 0) {
      if (atual.direita === null) {
        atual.direita = { chegada: chegada, esquerda: null, direita: null };
        break;
      }
      atual = atual.direita;
    } else {
      break;
    }
  }

  return no;
};

const calcularAltura = function (no) {
  if (no === null) {
    return 0;
  }

  let nivel = [no];
  let altura = 0;

  while (nivel.length > 0) {
    altura++;
    const proximoNivel = [];

    for (const atual of nivel) {
      if (atual.esquerda !== null) proximoNivel.push(atual.esquerda);
      if (atual.direita !== null) proximoNivel.push(atual.direita);
    }

    nivel = proximoNivel;
  }

  return altura;
};

// Busca uma chegada descendo pela árvore. Devolve o nó ou null.
const buscarNaArvore = function (chegada) {
  let atual = raiz;

  while (atual !== null) {
    const comparacao = compararChegadas(chegada, atual.chegada);

    if (comparacao === 0) return atual;

    atual = comparacao < 0 ? atual.esquerda : atual.direita;
  }

  return null;
};

// Percorre a árvore em ordem, começando pela primeira chegada a partir
// de "referencia" ("AAAA-MM-DD HH:MM:SS"). Para quando visitar devolve true.
const percorrerAPartirDe = function (referencia, visitar) {
  const pilha = [];
  let atual = raiz;

  while (atual !== null || pilha.length > 0) {
    if (atual !== null) {
      if (chave(atual.chegada) >= referencia) {
        pilha.push(atual);
        atual = atual.esquerda;
      } else {
        atual = atual.direita;
      }
    } else {
      const no = pilha.pop();
      if (visitar(no.chegada)) return;
      atual = no.direita;
    }
  }
};

const removerDaArvore = function (no, chegada) {
  let atual = no;
  let pai = null;

  while (atual !== null) {
    const comparacao = compararChegadas(chegada, atual.chegada);

    if (comparacao < 0) {
      pai = atual;
      atual = atual.esquerda;
    } else if (comparacao > 0) {
      pai = atual;
      atual = atual.direita;
    } else {
      break;
    }
  }

  if (atual === null) {
    return no;
  }

  if (atual.esquerda !== null && atual.direita !== null) {
    let paiSucessor = atual;
    let sucessor = atual.direita;

    while (sucessor.esquerda !== null) {
      paiSucessor = sucessor;
      sucessor = sucessor.esquerda;
    }

    atual.chegada = sucessor.chegada;

    if (paiSucessor === atual) {
      paiSucessor.direita = sucessor.direita;
    } else {
      paiSucessor.esquerda = sucessor.direita;
    }

    return no;
  }

  const filho = atual.esquerda !== null ? atual.esquerda : atual.direita;

  if (pai === null) {
    return filho;
  }

  if (pai.esquerda === atual) {
    pai.esquerda = filho;
  } else {
    pai.direita = filho;
  }

  return no;
};

const buscarChegada = (data, horario, linha) => {
  const encontrado = buscarNaArvore({ data, horario, linha });
  console.log(encontrado !== null ? "ACHOU" : "NAO");
  return encontrado;
};

const proximo = (data, horario) => {
  const referencia = `${data} ${horario}`;

  let resultado = null;
  percorrerAPartirDe(referencia, (c) => {
    resultado = c;
    return true;
  });

  console.log(
    resultado
      ? `${resultado.data} ${resultado.horario} ${resultado.linha}`
      : "NENHUM",
  );
  return resultado;
};

const proximo_da_linha = function (data, horario, linha) {
  const referencia = `${data} ${horario}`;

  let resultado = null;
  percorrerAPartirDe(referencia, (c) => {
    if (c.linha !== linha) return false;
    resultado = c;
    return true;
  });

  console.log(resultado ? `${resultado.data} ${resultado.horario}` : "NENHUM");
  return resultado;
};

const entre = function (data1, horario1, data2, horario2) {
  const inicio = `${data1} ${horario1}`;
  const fim = `${data2} ${horario2}`;

  const resultado = [];
  percorrerAPartirDe(inicio, (c) => {
    if (chave(c) > fim) return true;
    resultado.push(c);
    return false;
  });

  console.log(resultado.length);
  for (const r of resultado) {
    console.log(`${r.data} ${r.horario} ${r.linha}`);
  }
  return resultado;
};

const remover_chegada = function (data, horario, linha) {
  const no = buscarNaArvore({ data, horario, linha });

  if (no === null) {
    console.log("NAO");
    return false;
  }

  raiz = removerDaArvore(raiz, no.chegada);

  console.log("OK");
  return true;
};

const altura = function () {
  const resultado = calcularAltura(raiz);

  console.log(resultado);
  return resultado;
};

const espera_semana = function (linha, data, horario, k) {
  const resultados = [];

  for (let i = k - 1; i >= 0; i--) {
    const dia = new Date(`${data}T12:00:00`);
    dia.setDate(dia.getDate() - i * 7);

    const dataConsulta = `${dia.getFullYear()}-${String(dia.getMonth() + 1).padStart(2, "0")}-${String(dia.getDate()).padStart(2, "0")}`;

    let chegada = null;
    percorrerAPartirDe(`${dataConsulta} ${horario}`, (c) => {
      if (c.data !== dataConsulta) return true;
      if (c.linha !== linha) return false;
      chegada = c;
      return true;
    });

    if (chegada === null) {
      console.log(`${dataConsulta} NAOPASSOU`);

      resultados.push({
        data: dataConsulta,
        espera: null,
      });

      continue;
    }

    const [horaInicio, minutoInicio, segundoInicio] = horario
      .split(":")
      .map(Number);
    const [horaChegada, minutoChegada, segundoChegada] = chegada.horario
      .split(":")
      .map(Number);

    const inicio = horaInicio * 3600 + minutoInicio * 60 + (segundoInicio || 0);
    const fim = horaChegada * 3600 + minutoChegada * 60 + (segundoChegada || 0);

    const espera = fim - inicio;

    console.log(`${dataConsulta} ${espera}`);

    resultados.push({
      data: dataConsulta,
      espera: espera,
    });
  }

  return resultados;
};

// Executa um comando já separado em nome e argumentos. Usada tanto pelo
// REPL interativo quanto pelo modo em lote (processamento de arquivo).
const executarComando = function (cmd, a) {
  switch (cmd) {
    case "TITLE":
      break;
    case "REG": {
      const cadastrado = registrarChegada(a[0], a[1], a[2]);
      console.log(cadastrado ? "OK" : "DUPLICADO");
      break;
    }
    case "BUSCA":
      buscarChegada(a[0], a[1], a[2]);
      break;
    case "PROX":
      proximo(a[0], a[1]);
      break;
    case "PROXL":
      proximo_da_linha(a[0], a[1], a[2]);
      break;
    case "ENTRE":
      entre(a[0], a[1], a[2], a[3]);
      break;
    case "REM":
      remover_chegada(a[0], a[1], a[2]);
      break;
    case "ALT":
      altura();
      break;
    case "SEMANA":
    case "ESPERA":
      espera_semana(a[0], a[1], a[2], Number(a[3]));
      break;
    default:
      console.log("Comando inválido", cmd);
  }
};

// Lê o arquivo de operações inteiro, executa tudo e só então escreve as
// respostas na saída padrão, como exige o formato de entrada e saída do
// enunciado. Mede, por fora da leitura/escrita, o tempo de cada função (tipo
// de comando) e o tempo total gasto apenas com as operações, escrevendo os
// dois na saída de erro: "TEMPO_<COMANDO>_MS valor" por comando e, na última
// linha, "TEMPO_MS valor" com o total (formato exigido pelo enunciado).
const executarEmLote = function (caminhoArquivo) {
  const fs = require("fs");
  const util = require("util");
  const { performance } = require("perf_hooks");

  const comandos = fs
    .readFileSync(caminhoArquivo, "utf8")
    .split(/\r?\n/)
    .map((linha) => linha.trim())
    .filter((texto) => texto !== "" && !texto.startsWith("#"))
    .map((texto) => {
      const [cmd, ...a] = texto.split(/\s+/);
      return { cmd: (cmd || "").toUpperCase(), a };
    });

  const saida = [];
  const logOriginal = console.log;
  console.log = (...args) => saida.push(util.format(...args));

  const tempoPorComando = new Map();
  const inicioTotal = performance.now();

  for (const { cmd, a } of comandos) {
    const inicio = performance.now();
    executarComando(cmd, a);
    const duracao = performance.now() - inicio;
    tempoPorComando.set(cmd, (tempoPorComando.get(cmd) || 0) + duracao);
  }

  const tempoTotal = performance.now() - inicioTotal;
  console.log = logOriginal;

  process.stdout.write(saida.join("\n") + (saida.length ? "\n" : ""));

  for (const [cmd, tempo] of tempoPorComando) {
    process.stderr.write(`TEMPO_${cmd}_MS ${tempo.toFixed(3)}\n`);
  }
  process.stderr.write(`TEMPO_MS ${tempoTotal.toFixed(3)}\n`);
};

const caminhoArquivo = process.argv[2];

if (caminhoArquivo) {
  executarEmLote(caminhoArquivo);
} else {
  const readline = require("readline");

  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    prompt: "> ",
  });

  console.log(
    "Comandos: REG d h l | BUSCA d h l | PROX d h | PROXL d h l | ENTRE d1 h1 d2 h2 | REM d h l | ALT | ESPERA l d h k | SAIR",
  );
  rl.prompt();

  rl.on("line", (linha) => {
    const texto = linha.trim();
    if (texto === "" || texto.startsWith("#")) {
      return;
    }

    const [cmd, ...a] = texto.split(/\s+/);
    const comando = (cmd || "").toUpperCase();

    if (comando === "SAIR") {
      rl.close();
      return;
    }

    executarComando(comando, a);
    rl.prompt();
  });
}
