fn main() {
    // A forma mais simples de corrigir o erro do compilador é inicializar a
    // variável `x`. Ao atribuir um inteiro, Rust infere o tipo
    // como `i32`, que é o tipo inteiro padrão.
    let x = 42;

    // Também podemos forçar um tipo diferente do `i32` padrão adicionando
    // uma anotação de tipo:
    // let x: u8 = 42;

    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
