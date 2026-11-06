fn main() {
    my_macro!();
}

// TODO: Corrija o erro de compilação movendo a definição inteira desta macro.
macro_rules! my_macro {
    () => {
        println!("Veja minha macro!");
    };
}
