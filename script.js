const registros = [];
const chave = (r) => `${r.data} ${r.horario}`;

const registrarChegada = (data, horario, linha) => {
  const chegada = { data, horario, linha };

  const existente = registros.find(
    (r) =>
      r.data === data &&
      r.horario === horario &&
      r.linha === linha,
  );

  if (existente) {
    return false;
  }

  registros.push(chegada);
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

const inserirNaArvore = function (no, chegada) {
  if (no === null) {
    return {
      chegada: chegada,
      esquerda: null,
      direita: null,
    };
  }

  const comparacao = compararChegadas(chegada, no.chegada);

  if (comparacao < 0) {
    no.esquerda = inserirNaArvore(no.esquerda, chegada);
  } else if (comparacao > 0) {
    no.direita = inserirNaArvore(no.direita, chegada);
  }

  return no;
};

const calcularAltura = function (no) {
  if (no === null) {
    return 0;
  }

  const esquerda = calcularAltura(no.esquerda);
  const direita = calcularAltura(no.direita);

  return 1 + Math.max(esquerda, direita);
};

const removerDaArvore = function (no, chegada) {
  if (no === null) {
    return null;
  }

  const comparacao = compararChegadas(chegada, no.chegada);

  if (comparacao < 0) {
    no.esquerda = removerDaArvore(no.esquerda, chegada);
  } else if (comparacao > 0) {
    no.direita = removerDaArvore(no.direita, chegada);
  } else {
    if (no.esquerda === null) {
      return no.direita;
    }

    if (no.direita === null) {
      return no.esquerda;
    }

    let sucessor = no.direita;

    while (sucessor.esquerda !== null) {
      sucessor = sucessor.esquerda;
    }

    no.chegada = sucessor.chegada;
    no.direita = removerDaArvore(no.direita, sucessor.chegada);
  }

  return no;
};

const buscarChegada = (data, horario, linha) => {
  const encontrado = registros.find(
    (r) => r.data === data && r.horario === horario && r.linha === linha,
  );
  console.log(encontrado ?? "Não encontrado");
  return encontrado;
};

const proximo = (data, horario) => {
  const referencia = `${data} ${horario}`;

  const proximo = registros
    .filter((r) => `${r.data} ${r.horario}` >= referencia)
    .sort((a, b) =>
      `${a.data} ${a.horario}`.localeCompare(`${b.data} ${b.horario}`),
    )[0];

  console.log(proximo ?? "Nenhuma chegada futura");
  return proximo;
};

const proximo_da_linha = function (data, horario, linha) {
  const referencia = `${data} ${horario}`;

  const resultado = registros
    .filter((r) => r.linha === linha && chave(r) >= referencia)
    .sort((a, b) => chave(a).localeCompare(chave(b)))[0];

  console.log(resultado ?? "Nenhuma chegada futura para a linha");
  return resultado;
};

const entre = function (data1, horario1, data2, horario2) {
  const inicio = `${data1} ${horario1}`;
  const fim = `${data2} ${horario2}`;

  const resultado = registros
    .filter((r) => chave(r) >= inicio && chave(r) <= fim)
    .sort((a, b) => chave(a).localeCompare(chave(b)));

  console.log(resultado.length ? resultado : "Nenhuma chegada no intervalo");
  return resultado;
};

const remover_chegada = function (data, horario, linha) {
  const indice = registros.findIndex(
    (r) =>
      r.data === data &&
      r.horario === horario &&
      r.linha === linha,
  );

  if (indice === -1) {
    console.log("Chegada não encontrada");
    return false;
  }

  const chegada = registros[indice];

  raiz = removerDaArvore(raiz, chegada);
  registros.splice(indice, 1);

  console.log("Chegada removida com sucesso");
  return true;
};

const altura = function () {
  const resultado = calcularAltura(raiz);

  console.log(resultado);
  return resultado;
};

const espera_semana = function (linha, data, horario, k) {
  const resultados = [];

  for (let i = 0; i < k; i++) {
    const dia = new Date(`${data}T12:00:00`);
    dia.setDate(dia.getDate() - i * 7);

    const dataConsulta = `${dia.getFullYear()}-${String(dia.getMonth() + 1).padStart(2, "0")}-${String(dia.getDate()).padStart(2, "0")}`;

    const chegadas = registros
      .filter(
        (r) =>
          r.linha === linha && r.data === dataConsulta && r.horario >= horario,
      )
      .sort((a, b) => a.horario.localeCompare(b.horario));

    if (chegadas.length === 0) {
      console.log(`${dataConsulta}: A linha não passou mais nesse dia`);

      resultados.push({
        data: dataConsulta,
        espera: null,
      });

      continue;
    }

    const chegada = chegadas[0];

    const [horaInicio, minutoInicio] = horario.split(":").map(Number);
    const [horaChegada, minutoChegada] = chegada.horario.split(":").map(Number);

    const inicio = horaInicio * 60 + minutoInicio;
    const fim = horaChegada * 60 + minutoChegada;

    const espera = fim - inicio;

    console.log(`${dataConsulta}: Espera de ${espera} minutos`);

    resultados.push({
      data: dataConsulta,
      espera: espera,
    });
  }

  return resultados;
};

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

  switch ((cmd || "").toUpperCase()) {
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
    case "SAIR":
      rl.close();
      return;
    case "":
      break;
    default:
      console.log("Comando inválido", linha);
  }
  rl.prompt();
});
