// Clippy é uma coleção de lints que analisa seu código para encontrar erros
// comuns e sugerir melhorias.
//
// Nestes exercícios, avisos do Clippy farão o código falhar. Leia as sugestões
// exibidas no terminal para descobrir como corrigir cada problema.

fn main() {
    // TODO: Corrija o lint do Clippy nesta linha.
    let pi = 3.14;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("A área de um círculo de raio {radius:.2} é {area:.5}");
}
