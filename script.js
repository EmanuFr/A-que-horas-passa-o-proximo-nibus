let registros = [];

const registrar_chegada = function (data, horario, linha) {
  let registro = {
    data: data,
    horario: horario,
    linha: linha,
  };

  registros[registros.length] = registro;
};

const buscar_chegada = function (data, horario, linha) {
  for (busca of registros) {
    if (
      busca.data == data &&
      busca.horario == horario &&
      busca.linha == linha
    ) {
      return console.log(busca);
    }
  }
  console.log("Nao encontrado");
};

const proximo = function (data, horario) {};

const proximo_da_linha = function (data, horario, linha) {};

const entre = function (data1, horario1, data2, horario2) {};

const remover_chegada = function (data, horario, linha) {};

const altura = function () {};

const espera_semana = function (linha, data, horario, k) {};
