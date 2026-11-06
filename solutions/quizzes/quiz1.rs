// Mary está comprando maçãs. O preço funciona assim:
// - Cada maçã custa 2 rustbucks.
// - Porém, se Mary comprar mais de 40 maçãs, o preço de cada maçã em todo
// o pedido cai para apenas 1 rustbuck!

fn calculate_price_of_apples(n_apples: u64) -> u64 {
    if n_apples > 40 {
        n_apples
    } else {
        2 * n_apples
    }
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

// Não altere os testes!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
