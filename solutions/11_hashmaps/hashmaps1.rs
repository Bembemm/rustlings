// Você precisa definir uma cesta de frutas usando um hash map. A chave
// representa o nome da fruta e o valor representa quantas unidades dela
// existem na cesta. Coloque pelo menos 3 tipos diferentes de frutas
// (por exemplo, maçã, banana e manga), e o total de frutas
// deve ser de pelo menos 5.

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // Declare o hash map.
    let mut basket = HashMap::new();

    // Duas bananas já foram adicionadas para você :)
    basket.insert(String::from("banana"), 2);

    // Adicione mais frutas à cesta.
    basket.insert(String::from("apple"), 3);
    basket.insert(String::from("mango"), 1);

    basket
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
