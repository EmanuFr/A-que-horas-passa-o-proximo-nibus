const registros = [];

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

const proximo_da_linha = function (data, horario, linha) {};

const entre = function (data1, horario1, data2, horario2) {};

const remover_chegada = function (data, horario, linha) {};

const altura = function () {};

const espera_semana = function (linha, data, horario, k) {};
