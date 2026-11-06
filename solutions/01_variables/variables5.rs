fn main() {
    let number = "T-R-E-S";
    println!("Escreva um número: {number}");

    // Usando sombreamento de variável (*shadowing*)
    // https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing
    let number = 3;
    println!("O número mais dois é: {}", number + 2);
}
