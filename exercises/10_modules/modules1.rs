// TODO: Corrija o erro de compilação causado pela chamada a uma função privada.
mod sausage_factory {
    // Não permita que ninguém de fora deste módulo veja isto!
    fn get_secret_recipe() -> String {
        String::from("Gengibre")
    }

    fn make_sausage() {
        get_secret_recipe();
        println!("salsicha!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
