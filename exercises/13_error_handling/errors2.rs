// Imagine um jogo em que itens podem ser comprados com fichas. Cada item custa
// 5 fichas e toda compra possui uma taxa de processamento de 1 ficha. O jogador
// informa quantos itens deseja comprar e `total_cost` calcula o custo total.
// Como a quantidade digitada chega como uma string, ela pode conter qualquer
// coisa — não apenas números!
//
// Atualmente, esta função não trata o caso de erro. Se `total_cost` receber
// uma string que não representa um número, a conversão produzirá um
// `ParseIntError`. Nesse caso, queremos retornar imediatamente esse erro, sem
// tentar realizar a multiplicação ou a soma.
//
// Há pelo menos duas formas corretas de implementar isso, mas uma delas é
// bem mais curta!

use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // TODO: Trate o caso de erro conforme descrito acima.
    let qty = item_quantity.parse::<i32>();

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
