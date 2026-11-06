// Uma escola de magia imaginária possui um novo sistema de boletins escrito
// em Rust! Atualmente, o sistema só cria boletins em que
// a nota do aluno é numérica (por exemplo, 1.0 -> 5.5). Porém, a
// escola também usa notas por letras (A+ -> F-) e precisa
// exibir os dois tipos de boletim!
//
// Faça as alterações necessárias na struct `ReportCard` e no bloco `impl`
// para aceitar notas por letras além das numéricas.

use std::fmt::Display;

// Torne a struct genérica sobre `T`.
struct ReportCard<T> {
    //           ^^^
    grade: T,
    //     ^
    student_name: String,
    student_age: u8,
}

// Para poder exibir a nota, o tipo precisa implementar a trait `Display`.
impl<T: Display> ReportCard<T> {
    //  ^^^^^^^ exige que `T` implemente `Display`.
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            self.student_name, self.student_age, self.grade,
        )
    }
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}
