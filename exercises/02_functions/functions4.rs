// Esta loja está fazendo uma promoção: se o preço for par, você recebe
// 10 Rustbucks de desconto. Se for ímpar, recebe 3 Rustbucks de desconto.
// Não se preocupe com o corpo das funções por enquanto:
// nosso foco está apenas nas assinaturas.

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO: Corrija a assinatura da função.
fn sale_price(price: i64) -> {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("O preço com desconto é {}", sale_price(original_price));
}
