// TODO: Corrija o erro de compilação em `main` sem alterar esta função.
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "verde" || attempt == "azul" || attempt == "vermelho"
}

fn main() {
    let word = String::from("verde"); // Não altere esta linha.

    if is_a_color_word(word) {
        println!("Essa é uma cor que eu conheço!");
    } else {
        println!("Essa não é uma cor que eu conheço.");
    }
}
