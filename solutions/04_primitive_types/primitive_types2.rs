fn main() {
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("É uma letra!");
    } else if my_first_initial.is_numeric() {
        println!("É um número!");
    } else {
        println!("Não é letra nem número!");
    }

    // Exemplo usando um emoji.
    let your_character = '🦀';

    if your_character.is_alphabetic() {
        println!("É uma letra!");
    } else if your_character.is_numeric() {
        println!("É um número!");
    } else {
        println!("Não é letra nem número!");
    }
}
