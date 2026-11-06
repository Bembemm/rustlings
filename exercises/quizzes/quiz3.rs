// Este quiz revisa:
// - Genéricos
// - Traits
//
// Uma escola de magia imaginária criou em Rust um sistema de boletins.
// Atualmente, ele aceita apenas notas numéricas (por exemplo, 1.0 -> 5.5),
// mas a escola também utiliza notas com letras (A+ -> F-).
//
// Faça as alterações necessárias em `ReportCard` e no bloco `impl` para
// aceitar os dois tipos de nota.

// TODO: Adapte a struct conforme descrito acima.
struct ReportCard {
    grade: f32,
    student_name: String,
    student_age: u8,
}

// TODO: Adapte o bloco `impl` conforme descrito acima.
impl ReportCard {
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
