#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    // Exemplo: 42 / 0
    DivideByZero,
    // Único caso para `i64`: `i64::MIN / -1`, pois o resultado seria `i64::MAX + 1`
    IntegerOverflow,
    // Exemplo: 5 / 2 = 2,5
    NotDivisible,
}

// TODO: Calcule `a / b` quando `a` for divisível exatamente por `b`.
// Caso contrário, retorne um erro apropriado.
fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    todo!();
}

// TODO: Adicione o tipo de retorno correto e complete o corpo da função.
// Saída desejada: `Ok([1, 11, 1426, 3])`
fn result_with_list() {
    let numbers = [27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
}

// TODO: Adicione o tipo de retorno correto e complete o corpo da função.
// Saída desejada: `[Ok(1), Ok(11), Ok(1426), Ok(3)]`
fn list_of_results() {
    let numbers = [27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
        assert_eq!(divide(81, -1), Ok(-81));
        assert_eq!(divide(i64::MIN, i64::MIN), Ok(1));
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_integer_overflow() {
        assert_eq!(divide(i64::MIN, -1), Err(DivisionError::IntegerOverflow));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(divide(81, 6), Err(DivisionError::NotDivisible));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(result_with_list().unwrap(), [1, 11, 1426, 3]);
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(list_of_results(), [Ok(1), Ok(11), Ok(1426), Ok(3)]);
    }
}
