fn square(num: i32) -> i32 {
    // O ponto e vírgula `;` foi removido do fim da linha abaixo para retornar o resultado implicitamente.
    num * num
}

fn main() {
    let answer = square(3);
    println!("O quadrado de 3 é {answer}");
}
