// Imagine um jogo em que você compra itens com fichas. Todos os itens custam
// 5 fichas, e cada compra cobra uma taxa de processamento de 1
// ficha. O jogador informa quantos itens deseja comprar, e
// a função `total_cost` calcula o custo total. Como
// a quantidade digitada chega como string, ela pode conter
// qualquer coisa, não apenas números!
//
// No momento, esta função não trata o caso de erro. Queremos
// que, se `total_cost` receber uma string que não seja um
// número, a função retorne um `ParseIntError`. Nesse caso, queremos
// retornar o erro imediatamente, sem tentar multiplicar nem
// somar.
//
// Há pelo menos duas formas corretas de implementar isso, mas uma
// é bem mais curta!

use std::num::ParseIntError;

#[allow(unused_variables, clippy::question_mark)]
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // Foi adicionado `?` para propagar o erro.
    let qty = item_quantity.parse::<i32>()?;
    //                                    ^ adicionado

    // Equivalente a esta versão mais explícita:
    let qty = match item_quantity.parse::<i32>() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}
