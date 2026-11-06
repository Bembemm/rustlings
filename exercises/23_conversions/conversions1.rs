// Conversões explícitas de tipo em Rust podem ser feitas com o operador `as`.
// Observe que `as` também pode ser usado em outros contextos, como ao renomear
// importações.

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    // TODO: Faça uma conversão antes da divisão.
    total / values.len()
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
