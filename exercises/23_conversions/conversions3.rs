// Neste exercício, implementaremos `FromStr` para converter dados armazenados
// em uma string para um tipo estruturado. Diferentemente de `From`, essa
// conversão pode falhar e, por isso, retorna um `Result`. Depois de implementar
// `FromStr`, você poderá usar `parse` em strings para criar valores do tipo.
// Documentação: https://doc.rust-lang.org/std/str/trait.FromStr.html

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

// Usaremos este tipo de erro na implementação de `FromStr`.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Número incorreto de campos
    BadLen,
    // Campo de nome vazio
    NoName,
    // Erro recebido de `parse::<u8>()`
    ParseInt(ParseIntError),
}

// TODO: Complete esta implementação de `FromStr` para converter uma string
// no formato "Mark,20" em uma `Person`.
// Para converter a idade em `u8`, você pode usar algo como `"4".parse::<u8>()`.
//
// Passos:
// 1. Separe a string pelas vírgulas.
// 2. Se houver menos ou mais de 2 elementos, retorne `ParsePersonError::BadLen`.
// 3. Use o primeiro elemento como nome.
// 4. Se o nome estiver vazio, retorne `ParsePersonError::NoName`.
// 5. Converta o segundo elemento para `u8` como idade.
// 6. Se a conversão falhar, retorne `ParsePersonError::ParseInt`.
impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {}
}

fn main() {
    let p = "Mark,20".parse::<Person>();
    println!("{p:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ParsePersonError::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!("John,".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!("John,twenty".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(",".parse::<Person>(), Err(NoName | ParseInt(_))));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(NoName | ParseInt(_)),
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!("John,32,man".parse::<Person>(), Err(BadLen));
    }
}
