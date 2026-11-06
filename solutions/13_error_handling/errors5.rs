// Este exercício é uma versão modificada de `errors4`. Ele usa alguns
// conceitos que só veremos mais adiante, como `Box` e a
// trait `From`. Não é necessário entendê-los em detalhes agora, mas
// você pode adiantar a leitura se quiser. Por enquanto, pense em `Box<dyn ???>` como
// um tipo "quero qualquer coisa que faça ???".
//
// Resumindo, este uso de `Box` serve quando você quer possuir um
// valor e só importa que o tipo implemente uma determinada
// trait. Para isso, declaramos `Box<dyn Trait>`, em que
// `Trait` é a trait que o compilador exige dos valores usados nesse
// contexto. Neste exercício, esse contexto são os possíveis erros que
// podem ser retornados em um `Result`.

use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// Isto é necessário para que `CreationError` possa implementar `Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "número negativo",
            CreationError::Zero => "número igual a zero",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("saída={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}
