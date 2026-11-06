// A definição da macro foi movida para antes da chamada.
macro_rules! my_macro {
    () => {
        println!("Veja minha macro!");
    };
}

fn main() {
    my_macro!();
}
