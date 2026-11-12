// Uma chegada registrada: linha de ônibus que chegou numa data e horário.
// A chave que ordena as chegadas na árvore é (data, horario, linha), nessa
// ordem, comparando cada campo como string. Como data e horário sempre vêm
// no formato de largura fixa AAAA-MM-DD e HH:MM:SS, a ordem lexicográfica de
// string já é a ordem cronológica certa.
#[derive(Clone)]
pub struct Chegada {
    pub data: String,
    pub horario: String,
    pub linha: String,
    pub removida: bool,
}

// Chave usada só para comparar (não é guardada em lugar nenhum): três
// referências de string, comparadas campo a campo, na ordem data, horario,
// linha. `derive(Ord)` numa struct compara os campos nessa mesma ordem, então
// isso equivale a escrever a comparação à mão.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Chave<'a> {
    pub data: &'a str,
    pub horario: &'a str,
    pub linha: &'a str,
}

impl Chegada {
    pub fn chave(&self) -> Chave<'_> {
        Chave {
            data: &self.data,
            horario: &self.horario,
            linha: &self.linha,
        }
    }
}

// Converte "HH:MM:SS" em segundos desde meia-noite, para calcular a espera
// em segundos entre dois horários (usado em R8).
pub fn horario_em_segundos(horario: &str) -> i64 {
    let mut partes = horario.split(':');
    let h: i64 = partes.next().unwrap_or("0").parse().unwrap_or(0);
    let m: i64 = partes.next().unwrap_or("0").parse().unwrap_or(0);
    let s: i64 = partes.next().unwrap_or("0").parse().unwrap_or(0);
    h * 3600 + m * 60 + s
}
