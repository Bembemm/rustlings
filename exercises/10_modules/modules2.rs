// Você pode importar caminhos de módulos para o escopo e atribuir novos nomes
// a eles usando as palavras-chave `use` e `as`.

mod delicious_snacks {
    // TODO: Corrija e adicione as duas instruções `use` a seguir.
    // use self::fruits::PEAR as ???;
    // use self::veggies::CUCUMBER as ???;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "petiscos favoritos: {} e {}",
        delicious_snacks::FRUIT,
        delicious_snacks::VEGGIE,
    );
}
