// `Vec<T>` é genérico sobre o tipo `T`. Na maioria dos casos, o compilador
// consegue inferir `T`, por exemplo depois que um valor de tipo conhecido é
// inserido no vetor. Neste exercício, porém, será necessária uma anotação de tipo.

fn main() {
    // TODO: Corrija o erro de compilação anotando o tipo do vetor `Vec<T>`.
    // Escolha para `T` algum tipo inteiro que possa ser criado a partir de
    // valores `u8` e `i8`.
    let mut numbers = Vec::new();

    // Não altere as linhas abaixo.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
