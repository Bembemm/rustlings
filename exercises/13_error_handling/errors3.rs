// Este programa tenta usar uma versão concluída da função `total_cost` do
// exercício anterior, mas algo não funciona. Por quê? O que precisa ser
// alterado para corrigir o problema?

use std::num::ParseIntError;

// Não altere esta função.
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// TODO: Corrija o erro de compilação alterando a assinatura e o corpo da
// função `main`.
fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // Não altere esta linha.
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("Você não tem fichas suficientes para comprar tudo isso!");
    } else {
        tokens -= cost;
        println!("Agora você tem {tokens} fichas.");
    }
}
