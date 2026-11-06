// Clippy é uma coleção de lints que analisa seu código para
// encontrar erros comuns e melhorar seu código Rust.
//
// Nestes exercícios, o código falhará quando houver avisos do Clippy.
// Leia as sugestões do Clippy na saída para resolver o exercício.

use std::f32::consts::PI;

fn main() {
    // Use a constante `PI`, que é mais precisa.
    let pi = PI;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("A área de um círculo de raio {radius:.2} é {area:.5}");
}
