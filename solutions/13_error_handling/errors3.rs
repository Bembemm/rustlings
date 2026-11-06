// Este programa tenta usar uma versão concluída da
// função `total_cost` do exercício anterior, mas algo não funciona!
// Por quê? O que devemos fazer para corrigir?

use std::num::ParseIntError;

// Não altere esta função.
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

fn main() -> Result<(), ParseIntError> {
    //    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ adicionado
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {tokens} tokens.");
    }

    // Esta linha foi adicionada para retornar a variante `Ok` do `Result` esperado.
    Ok(())
}
