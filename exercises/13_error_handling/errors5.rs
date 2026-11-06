// Este exercício é uma versão modificada de `errors4`. Ele utiliza alguns
// conceitos que aparecem mais adiante, como `Box` e a trait `From`. Não é
// necessário entendê-los em detalhes agora. Por enquanto, pense em
// `Box<dyn ???>` como "quero qualquer tipo que faça ???".
//
// Neste caso, usamos `Box` quando queremos possuir um valor, mas só nos importa
// que o tipo desse valor implemente uma determinada trait. Para isso, declaramos
// `Box<dyn Trait>`, onde `Trait` é o comportamento exigido pelo compilador.
// Aqui, estamos falando dos diferentes erros que podem ser retornados em um
// `Result`.

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
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
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

// TODO: Adicione o tipo de retorno correto: `Result<(), Box<dyn ???>>`.
// O que pode representar os dois tipos de erro? Existe uma trait implementada
// por ambos?
fn main() {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}
