// TODO: Corrija o erro de compilação sem retirar a definição da macro deste
// módulo.
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
