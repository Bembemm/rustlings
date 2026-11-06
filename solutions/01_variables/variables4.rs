fn main() {
    // Em Rust, as variáveis são imutáveis por padrão.
    // Adicionar `mut` depois de `let` torna a variável declarada mutável.
    let mut x = 3;
    println!("Número {x}");

    x = 5;
    println!("Número {x}");
}
