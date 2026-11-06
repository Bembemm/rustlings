mod sausage_factory {
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // Foi adicionado `pub` antes de `fn` para tornar a função acessível fora do módulo.
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
