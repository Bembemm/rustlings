// Caracteres (`char`)

fn main() {
    // Observe as aspas _simples_: elas são diferentes das aspas duplas
    // que você viu nos exemplos anteriores.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("É uma letra!");
    } else if my_first_initial.is_numeric() {
        println!("É um número!");
    } else {
        println!("Não é letra nem número!");
    }

    // TODO: Seguindo o exemplo anterior, declare abaixo uma variável chamada
    // `your_character` com seu caractere favorito.
    // Experimente uma letra, um dígito (entre aspas simples), um símbolo,
    // um caractere de outro idioma ou até um emoji 😉
    // let your_character = '';

    if your_character.is_alphabetic() {
        println!("É uma letra!");
    } else if your_character.is_numeric() {
        println!("É um número!");
    } else {
        println!("Não é letra nem número!");
    }
}
