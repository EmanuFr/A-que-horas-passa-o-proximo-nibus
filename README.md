# Problema 2 – A que horas passa o próximo ônibus?

_UnDF – Universidade do Distrito Federal Professor Jorge Amaury Maia Nunes_

Os estudantes que dependem de ônibus para chegar à universidade criaram um grupo de mensagens para avisar uns aos outros quando o ônibus chega: "o 136.1 acabou de chegar à parada da UnDF". O grupo cresceu, as mensagens se perdem no meio da conversa e ninguém consegue responder a perguntas simples, como "o 136.1 já passou depois das 18h30?" ou "quanto tempo esperou quem estava no ponto desde as 18h30?". O aplicativo oficial do GDF mostra uma previsão que muitas vezes não se confirma.

O coletivo de estudantes decidiu solicitar à turma de ciência da computação que construa um sistema que guarde cada chegada informada e responda a essas perguntas com base no que de fato aconteceu. Com semanas de registros acumulados, o sistema também permite ver se uma linha atrasa mais em certos dias da semana e se o serviço melhora ou piora ao longo do tempo. Com esses números, a reclamação de cada um pode virar uma cobrança coletiva, apoiada em dados, junto à Ouvidoria do DF e à Secretaria de Mobilidade do GDF.

Cada chegada informada tem a linha do ônibus, a data e o horário em que ele chegou à parada. Normalmente, o estudante avisa no grupo assim que entra no ônibus, e por isso a maior parte das chegadas entra no sistema em ordem de data e horário. Às vezes, porém, alguém esquece e só manda o aviso horas depois, e essa chegada entra no meio de outras que já foram registradas. Dois ônibus de linhas diferentes podem chegar no mesmo instante, mas a mesma linha não chega duas vezes no mesmo instante. Por isso, uma chegada é identificada pela data, pelo horário e pela linha. Às vezes alguém informa uma chegada por engano, e ela precisa ser apagada. São requisitos mínimos do sistema:

- **R1:** `registrar_chegada(data, horario, linha)`; Registra uma chegada. Exige tratamento quando a chegada já foi registrada.
- **R2:** `buscar_chegada(data, horario, linha)`; Informa se a chegada está registrada.
- **R3:** `proximo(data, horario)`; Retorna a primeira chegada registrada a partir daquele instante, de qualquer linha. Exige tratamento quando não há chegada a partir dele.
- **R4:** `proximo_da_linha(data, horario, linha)`; Retorna a data e o horário da primeira chegada da linha a partir daquele instante. É assim que se responde se o 136.1 já passou depois das 18h30 e quanto esperou quem estava no ponto desde esse horário. Exige tratamento quando não há chegada daquela linha a partir dele.
- **R5:** `entre(data1, horario1, data2, horario2)`; Retorna, em ordem de data e horário, todas as chegadas registradas entre os dois instantes, inclusive. É a resposta para "o que passou enquanto eu esperava?".
- **R6:** `remover_chegada(data, horario, linha)`; Apaga uma chegada informada por engano. Exige tratamento quando a chegada não está registrada.
- **R7:** `altura()`; Informa a altura da árvore que guarda as chegadas. Essa operação existe para que vocês acompanhem o que acontece com a árvore à medida que as chegadas são registradas.
- **R8:** `espera_semanal(linha, data, horario, k)`; Para a data informada e para as semanas anteriores, até completar k semanas, sempre no mesmo dia da semana e no mesmo horário, informa quanto esperou por aquela linha quem estava no ponto desde aquele horário. É assim que se vê se o 136.1 das 18h30 de segunda-feira atrasa toda semana. Exige tratamento quando a linha não passou mais naquele dia.

> _<u>Pergunta para o relatório:</u> Quais estruturas de dados sustentam essas operações de forma que o sistema continue rápido depois de meses de registros, sabendo que a maioria dos avisos chega em ordem de data e horário, mas parte chega com atraso? Quanto custa cada resposta, em tempo e em memória, e como vocês podem comprovar isso?_

## Formato de entrada e saída (obrigatório)

Neste problema, todos os grupos executam os mesmos arquivos de teste, e eu comparo os resultados de todos. Para isso, o programa lê um arquivo de operações, com uma operação por linha, e escreve as respostas exatamente no formato abaixo, qualquer que seja a linguagem. A data é escrita como `AAAA-MM-DD`, o horário como `HH:MM:SS` e a linha sem espaços, com até 8 caracteres. Linhas em branco e linhas iniciadas por `#` são ignoradas.

- `REG data hora linha` (R1): responde `OK` ou `DUPLICADO`.
- `BUSCA data hora linha` (R2): responde `ACHOU` ou `NAO`.
- `PROX data hora` (R3): responde "`data hora linha`" ou `NENHUM`.
- `PROXL data hora linha` (R4): responde "`data hora`" ou `NENHUM`.
- `ENTRE data1 hora1 data2 hora2` (R5): responde a quantidade k de chegadas e, em seguida, k linhas no formato "`data hora linha`", em ordem. Se o primeiro instante for posterior ao segundo, responde `0`.
- `REM data hora linha` (R6): responde `OK` ou `NAO`. Uma chegada apagada pode ser registrada de novo; nesse caso, `REG` responde `OK`.
- `ALT` (R7): responde um número inteiro. A árvore vazia tem altura 0, e uma árvore com um único nó tem altura 1. Na Fase 3, as respostas de `ALT` dadas depois da primeira remoção bem-sucedida podem variar conforme a forma de remoção escolhida pelo grupo e, por isso, não são comparadas com a resposta esperada.
- `SEMANA linha data hora k` (R8): responde k linhas, uma por semana, da mais antiga até a data informada, no formato "`data espera`", com a espera em segundos, ou "`data NAOPASSOU`" quando a linha não passou mais naquele dia a partir daquele horário. Por exemplo, `SEMANA 136.1 2026-10-26 18:30:00 4` responde pelas segundas-feiras 05/10, 12/10, 19/10 e 26/10.

Uma chegada vem antes de outra quando ocorreu antes; se ocorreram no mesmo instante, vem antes a de código de linha menor, comparando os códigos caractere por caractere, pelo código de cada caractere. Na árvore, cada chegada é guardada por uma chave que precisa respeitar essa ordem. Como montar essa chave é uma decisão do grupo, e o relatório precisa justificá-la. Para converter data e horário, vocês podem usar a biblioteca de datas da linguagem escolhida, inclusive no núcleo do código.

O programa lê o arquivo inteiro antes de executar as operações e só escreve as respostas no final, na saída padrão, no formato descrito acima. Ele mede o tempo gasto apenas com as operações e escreve esse valor, em milissegundos, com três casas decimais e ponto como separador, na saída de erro, em uma linha no formato `TEMPO_MS valor` (por exemplo, `TEMPO_MS 12.437`).

Na raiz do repositório deve ficar um arquivo `executar.txt` com três linhas: na primeira, o comando de compilação (ou um hífen, se a linguagem não precisar); na segunda, o comando que executa a Fase 1; na terceira, o comando que executa a Fase 3. Na segunda e na terceira linhas, `{arquivo}` marca o lugar do nome do arquivo de operações (por exemplo, `python fase3/sistema.py {arquivo}`). Os comandos são executados no Windows, a partir da raiz do repositório, com a versão da linguagem indicada no README. Na comparação com as respostas esperadas, a diferença entre as quebras de linha do Windows e do Linux é ignorada. Vou baixar o repositório e executar esses comandos; se o programa não rodar assim, o critério de execução será penalizado.

Na primeira sessão PBL vocês recebem um arquivo pequeno com alguns casos, junto com as respostas esperadas para a Fase 1 e para a Fase 3, e os arquivos de carga da Fase 2, sem as respostas. Na correção, usarei também outros arquivos no mesmo formato, com dados diferentes.

## As três fases (ordem obrigatória; todas são entregues)

### Fase 1: Faça funcionar

Implementem R1 a R8 com uma árvore binária de busca construída pelo grupo, do zero, seguindo o formato de entrada e saída. Ao remover um nó que tem dois filhos, substituam-no pelo sucessor, isto é, pelo menor nó da subárvore da direita. Para R4 e R8, nesta fase, basta a solução mais simples que funcione sobre a própria árvore. Testem com o arquivo pequeno e com os arquivos de carga embaralhados.

### Fase 2: Meça o funcionamento real

Executem a árvore da Fase 1 com os arquivos de carga de N = 1.000, 5.000 e 20.000 comandos `REG`, em duas versões: quase em ordem, que é como as chegadas são informadas no dia a dia, e embaralhadas, como se tivessem sido recuperadas de um backup fora de ordem. Executem cada arquivo cinco vezes, registrem em uma tabela a mediana dos tempos e a altura da árvore ao final da execução e respondam por escrito:

- **(a)** qual é a relação entre a altura da árvore e os tempos medidos? Usem a razão entre os tempos de N = 1.000 e de N = 20.000 nas duas versões.
- **(b)** desenhem, à mão ou no computador, a árvore que se forma ao inserir sete chegadas em ordem de data e horário. Qual é o formato dela e qual é a complexidade da busca nesse caso?

### Fase 3: Troque a estrutura e meça a diferença

Substituam a árvore da Fase 1 por uma árvore AVL implementada pelo grupo, do zero: fator de balanceamento, rotações simples e duplas e rebalanceamento na inserção.

- **(3.1)** A remoção (R6) na AVL é decisão do grupo (há mais de uma forma de remover). Qualquer uma é aceita, desde que o relatório diga quanto ela custa e do que o grupo abre mão ao escolhê-la. Em todos os casos, as respostas de R2, R3, R4, R5 e R8 consideram apenas as chegadas que não foram apagadas.
- **(3.2)** O próximo ônibus de uma linha (R4) e a espera semanal (R8) merecem atenção separada. Com uma única árvore ordenada por data e horário, quanto custam R4 e R8, e esse custo cresce à medida que os meses de registros se acumulam? Para a operação que precisar de outra estrutura, a solução é do grupo. Para a que não precisar, o grupo deve mostrar isso com medições. Em qualquer caso, o relatório deve informar quanto custam R4 e R8, quanta memória a solução ocupa e o que precisa ser feito a cada registro e a cada remoção para que nenhuma estrutura fique desatualizada. A árvore principal, que guarda todas as chegadas e responde a R7, continua sendo uma só.
- **(3.3)** Executem a AVL com os mesmos arquivos de carga da Fase 2, confiram se a altura final respeita o limite da AVL, altura ≤ 1,45 × log₂(n + 2), em que n é o número de nós da árvore ao final da execução, e apresentem a comparação lado a lado com a Fase 1, em tempo, altura e memória.

**Relatório:** Toda tabela e todo número do relatório devem poder ser reproduzidos com os arquivos fornecidos e com os comandos indicados no README.

## Regras de implementação (bibliotecas)

- O núcleo do sistema é a árvore principal (a árvore binária de busca da Fase 1 e a AVL da Fase 3) e as estruturas criadas para R4 e R8. Não é necessária interface gráfica.
- **Proibido no núcleo:** qualquer estrutura de dados pronta da linguagem ou de biblioteca externa. Em Python: `dict`, `set`, `collections`, `bisect`, `sorted()` e `.sort()`. Em C: `qsort()`, `bsearch()` e qualquer biblioteca de estruturas de dados. Em outras linguagens, vale o mesmo: mapas, conjuntos, árvores e funções de ordenação prontos (por exemplo, `HashMap`, `BTreeMap`, `TreeMap`, `std::map` e `sort`). Os nós, as classes e as rotações do núcleo devem ser de vocês. O vetor simples da linguagem é permitido, assim como tupla ou struct usada apenas para guardar os dados de uma chegada.
- A tabela hash construída por qualquer integrante no Problema 1 pode ser reaproveitada no núcleo.
- Liberado fora do núcleo: medição de tempo e de memória, leitura e escrita de arquivos e as estruturas prontas da linguagem apenas em códigos de teste.
- Só é avaliado o que estiver na branch principal do repositório.
- Violação da regra no núcleo zera o critério de corretude daquela entrega. Na dúvida se algo é núcleo, perguntem antes de usar.

**Uso de IA generativa:** vale o acordo pedagógico da disciplina. Não pode ser usado para criação de material que deveria ser de autoria do aluno (código ou relatório).

**Marcos semanais:** serão definidos pelo coordenador do grupo. As sessões PBL do Problema 2 acontecem em 15/10, 22/10 e 29/10, e a entrega é em 05/11. No Problema 2 não haverá rotação de papéis: cada grupo define, na primeira sessão, quem será o coordenador e o redator, e esses papéis valem até a entrega. A única entrega semanal é o registro do redator. Ele precisa ser detalhado o suficiente para que eu acompanhe o que o grupo pretende ter pronto no próximo encontro e quem é responsável por cada atividade em cada semana.

## Apresentação (10 minutos, com slides)

A apresentação é feita para o coletivo de estudantes (representado pela turma). Ela tem quatro partes:

1. para que serve o sistema e que perguntas ele responde;
2. ao vivo: a professora entrega na hora um arquivo curto, já com registros e consultas, e o grupo executa o comando da Fase 3 diante de todos e explica cada resposta em linguagem que o coletivo entenda (por exemplo, "nessa segunda, quem chegou às 18h30 esperou 10 minutos pelo 136.1");
3. por que confiar nos números: como o grupo garante que as respostas estão certas e por que o sistema continua rápido depois de meses de registros, com o gráfico de tempo e altura da Fase 1 contra a Fase 3;
4. limites: o que esses dados ainda não provam e o que faltaria para usá-los em uma reclamação formal.

O código não é mostrado na apresentação; ele é assunto da arguição.

## Arguição individual

Os integrantes podem ser solicitados a explicar trechos do código, simular no quadro uma rotação da AVL, fazer uma alteração simples no código, ou explicar frases do relatório escolhidas pela professora.

## Entregáveis

_Link do repositório e relatório via AVA, um envio por grupo, até o início da sessão de 05/11._

- Código organizado em repositório. Linguagem livre. Fase 1 e Fase 3 executáveis pelo `executar.txt`. README na raiz do repositório, indicando a linguagem e sua versão e, para cada tabela do relatório, o comando que a produz. O histórico de commits faz parte da entrega. O trabalho é registrado no repositório à medida que é feito, ao longo das semanas, e cada integrante faz commits com o próprio usuário.
- Relatório Técnico em formato de artigo.
- Slides da apresentação.
- Registros do redator das quatro semanas.

## Estrutura do Relatório em formato de artigo

O relatório deve responder a todas as perguntas feitas neste enunciado. Deve conter Introdução, Materiais e Métodos, Resultados e Conclusão, nos moldes do que foi feito no Problema 1. Algumas considerações sobre os resultados:

- **a)** a tabela da Fase 2, com a mediana dos tempos e a altura da árvore em cada arquivo de carga. As alturas são as que o programa do grupo produz com o comando indicado no README;
- **b)** as respostas às perguntas (a) e (b) da Fase 2;
- **c)** se alguma execução falhou ou não terminou, o que aconteceu e como o grupo resolveu;
- **d)** duas ilustrações (desenho, foto ou representação gráfica) com as chegadas reais do arquivo pequeno, sendo uma a árvore da Fase 1 depois das sete primeiras chegadas e outra uma rotação que a AVL do grupo fez ao processar esse arquivo;
- **e)** as decisões do grupo. Para cada uma, informem a alternativa descartada, o motivo, a sessão em que foi tomada (conforme o registro do redator) e o commit em que foi implementada. As decisões são três: a chave que guarda as chegadas; a forma de remoção na AVL (3.1); a solução para R4 e R8 (3.2);
- **f)** o que aconteceria com R3 e R5 se a chave fosse o texto "19_10_66600";
- **g)** em que situações um vetor ordenado com busca binária serviria no lugar da árvore e em qual deixaria de servir;
- **h)** a complexidade de tempo de cada operação nas duas fases, com o tipo de garantia (pior caso, caso esperado ou amortizado);
- **i)** a tabela comparativa final, Fase 1 contra Fase 3, com os mesmos arquivos de carga, em tempo, altura e memória;
- **j)** a verificação de que a altura final da AVL respeita o limite 1,45 × log₂(n + 2) em cada arquivo (3.3);
- **k)** o erro que mais custou ao grupo, como foi descoberto e em qual commit foi corrigido.

O código fica no repositório; no relatório entram apenas trechos curtos, quando ajudam a explicar uma decisão. Toda tabela tem título, unidade e uma frase explicando o que ela mostra.

## Por onde começar a estudar (referência)

- **Árvores binárias de busca:** Cormen et al., _Algoritmos: Teoria e Prática_, capítulo de árvores binárias de busca.
- **Árvores AVL e rotações:** Goodrich, Tamassia e Goldwasser, seção de árvores AVL. Desenhem cada rotação no papel antes de programar; a arguição pode pedir o desenho.
