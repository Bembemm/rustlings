macro_rules! my_macro {
    () => {
        println!("Veja minha macro!");
    };
}

fn main() {
    my_macro!();
    //      ^
}
