// TODO: Corrija o erro de compilação adicionando um ou dois caracteres.
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Veja minha macro!");
    }
    ($val:expr) => {
        println!("Veja esta outra macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
