// Estamos juntando frutas para fazer um bolo delicioso. A cesta será um
// hash map: a chave representa o tipo de fruta e o valor indica quantas
// unidades foram coletadas. Já temos maçãs (4), mangas (2) e lichias (5).
// Adicione frutas para que haja pelo menos uma unidade de cada tipo e
// mais de 11 frutas ao todo — há muita gente para alimentar!
// Você não pode aumentar as quantidades das frutas que já estão na cesta
// (maçã, manga e lichia).

use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = [
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
        // TODO: Insira as frutas que ainda não estiverem na cesta.
        // Atenção: você não pode adicionar mais unidades de tipos já presentes!
    }
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;

    // Não altere esta função!
    fn get_fruit_basket() -> HashMap<Fruit, u32> {
        let content = [(Fruit::Apple, 4), (Fruit::Mango, 2), (Fruit::Lychee, 5)];
        HashMap::from_iter(content)
    }

    #[test]
    fn test_given_fruits_are_not_modified() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
        assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
        assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
    }

    #[test]
    fn at_least_five_types_of_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count_fruit_kinds = basket.len();
        assert!(count_fruit_kinds >= 5);
    }

    #[test]
    fn greater_than_eleven_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count = basket.values().sum::<u32>();
        assert!(count > 11);
    }

    #[test]
    fn all_fruit_types_in_basket() {
        let fruit_kinds = [
            Fruit::Apple,
            Fruit::Banana,
            Fruit::Mango,
            Fruit::Lychee,
            Fruit::Pineapple,
        ];

        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);

        for fruit_kind in fruit_kinds {
            let Some(amount) = basket.get(&fruit_kind) else {
                panic!("A fruta do tipo {fruit_kind:?} não foi encontrada na cesta");
            };
            assert!(*amount > 0);
        }
    }
}
