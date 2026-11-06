// Foi adicionado o atributo `macro_use`.
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Veja minha macro!");
        };
    }
}

fn main() {
    my_macro!();
}
