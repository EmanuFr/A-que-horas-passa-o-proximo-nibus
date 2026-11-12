// Aritmética de calendário própria (sem biblioteca de datas), usada só para
// R8 (SEMANA), que precisa recuar de 7 em 7 dias a partir de uma data,
// atravessando virada de mês e de ano.

fn bissexto(ano: i32) -> bool {
    (ano % 4 == 0 && ano % 100 != 0) || ano % 400 == 0
}

fn dias_no_mes(ano: i32, mes: u32) -> u32 {
    match mes {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if bissexto(ano) {
                29
            } else {
                28
            }
        }
        _ => unreachable!("mês inválido: {mes}"),
    }
}

fn formatar(ano: i32, mes: u32, dia: u32) -> String {
    format!("{ano:04}-{mes:02}-{dia:02}")
}

// Recebe "AAAA-MM-DD" e devolve a data `dias` dias antes, no mesmo formato.
pub fn subtrair_dias(data: &str, dias: u32) -> String {
    let mut partes = data.split('-');
    let mut ano: i32 = partes.next().unwrap().parse().unwrap();
    let mut mes: u32 = partes.next().unwrap().parse().unwrap();
    let mut dia: u32 = partes.next().unwrap().parse().unwrap();

    for _ in 0..dias {
        if dia > 1 {
            dia -= 1;
        } else {
            mes = if mes > 1 { mes - 1 } else { 12 };
            if mes == 12 {
                ano -= 1;
            }
            dia = dias_no_mes(ano, mes);
        }
    }

    formatar(ano, mes, dia)
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn uma_semana_antes() {
        assert_eq!(subtrair_dias("2026-10-26", 7), "2026-10-19");
    }

    #[test]
    fn atravessa_mes() {
        assert_eq!(subtrair_dias("2026-10-05", 7), "2026-09-28");
    }

    #[test]
    fn atravessa_ano() {
        assert_eq!(subtrair_dias("2026-01-03", 7), "2025-12-27");
    }

    #[test]
    fn atravessa_fevereiro_bissexto() {
        assert_eq!(subtrair_dias("2028-03-02", 2), "2028-02-29");
    }
}
