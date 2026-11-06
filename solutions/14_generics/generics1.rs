// `Vec<T>` é genérico sobre o tipo `T`. Na maioria dos casos, o compilador consegue
// inferir `T`, por exemplo depois de inserir no vetor um valor de tipo concreto.
// Neste exercício, porém, o compilador precisa de ajuda por meio de uma anotação de tipo.

fn main() {
    // Tanto `u8` quanto `i8` podem ser convertidos para `i16`.
    let mut numbers: Vec<i16> = Vec::new();
    //             ^^^^^^^^^^ adicionado

    // Não altere as linhas abaixo.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
