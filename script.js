const registros = [];
const chave = (r) => `${r.data} ${r.horario}`;

const registrarChegada = (data, horario, linha) => {
  registros.push({ data, horario, linha });
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
    (r) => r.data === data && r.horario === horario && r.linha === linha,
  );

  if (indice === -1) {
    console.log("Chegada não encontrada");
    return false;
  }

  registros.splice(indice, 1);
  console.log("Chegada removida com sucesso");
  return true;
};

const altura = function () {}; // precisa implementar a estrutura de arvore para fazer essa função

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
  "Comandos: REG d h l | BUSCA d h l | PROX d h | PROXL d h l | ENTRE d1 h1 d2 h2 | REM d h l | ESPERA l d h k | SAIR",
);
rl.prompt();

rl.on("line", (linha) => {
  const [cmd, ...a] = linha.trim().split(/\s+/);

  switch ((cmd || "").toUpperCase()) {
    case "REG":
      registrarChegada(a[0], a[1], a[2]);
      console.log("OK");
      break;
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
    case "ESPERA":
      espera_semana(a[0], a[1], a[2], Number(a[3]));
      break;
    case "SAIR":
      rl.close();
      return;
    case "":
      break;
    default:
      console.log("Comando inválido");
  }
  rl.prompt();
});
