// Usar tipos genéricos de erro como `Box<dyn Error>` não é recomendado em
// código de biblioteca quando quem chama a função pode querer tomar decisões com base no
// conteúdo do erro, em vez de apenas imprimi-lo ou propagá-lo. Aqui, definimos
// um tipo de erro próprio para permitir que o chamador decida o que fazer
// quando a função retornar um erro.

use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// Tipo de erro personalizado usado em `PositiveNonzeroInteger::parse`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

// Como solução alternativa, implementar a trait `From` permite
// converter automaticamente um `ParseIntError` em `ParsePosNonzeroError`
// usando o operador `?`, sem precisar chamar `map_err`.
//
// ```
// let x: i64 = s.parse()?;
// ```
//
// Traits como `From` serão estudadas em exercícios posteriores.
impl From<ParseIntError> for ParsePosNonzeroError {
    fn from(err: ParseIntError) -> Self {
        ParsePosNonzeroError::ParseInt(err)
    }
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(Self(x as u64)),
        }
    }

    fn parse(s: &str) -> Result<Self, ParsePosNonzeroError> {
        // Retorne um erro apropriado em vez de causar `panic` quando `parse()`
        // retornar um erro.
        let x: i64 = s.parse().map_err(ParsePosNonzeroError::ParseInt)?;
        //                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        Self::new(x).map_err(ParsePosNonzeroError::Creation)
    }
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert!(matches!(
            PositiveNonzeroInteger::parse("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_)),
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            PositiveNonzeroInteger::parse("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative)),
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            PositiveNonzeroInteger::parse("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero)),
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42).unwrap();
        assert_eq!(x.0, 42);
        assert_eq!(PositiveNonzeroInteger::parse("42"), Ok(x));
    }
}
