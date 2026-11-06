// Este quiz revisa os seguintes assuntos:
// - Variáveis
// - Funções
// - `if`
//
// Mary está comprando maçãs. O preço funciona assim:
// - Cada maçã custa 2 rustbucks.
// - Se Mary comprar mais de 40 maçãs, cada maçã de todo o pedido passa
// a custar apenas 1 rustbuck.

// TODO: Escreva uma função que calcule o preço total de um pedido de maçãs
// com base na quantidade comprada.
// fn calculate_price_of_apples(???) -> ??? { ??? }

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
