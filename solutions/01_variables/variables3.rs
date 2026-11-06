#![allow(clippy::needless_late_init)]

fn main() {
    // Rust não permite ler variáveis não inicializadas!
    // Portanto, primeiro precisamos atribuir um valor.
    let x: i32 = 42;

    println!("Number {x}");

    // É possível declarar uma variável e inicializá-la depois.
    // Mas ela não pode ser usada antes da inicialização.
    let y: i32;
    y = 42;
    println!("Number {y}");
}
