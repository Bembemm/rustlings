mod delicious_snacks {
    // Foi adicionado `pub` e usado o alias esperado depois de `as`.
    pub use self::fruits::PEAR as FRUIT;
    pub use self::veggies::CUCUMBER as VEGGIE;

    mod fruits {
        pub const PEAR: &str = "Pera";
        pub const APPLE: &str = "Maçã";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Pepino";
        pub const CARROT: &str = "Cenoura";
    }
}

fn main() {
    println!(
        "petiscos favoritos: {} e {}",
        delicious_snacks::FRUIT,
        delicious_snacks::VEGGIE,
    );
}
