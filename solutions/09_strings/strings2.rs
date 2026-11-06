fn is_a_color_word(attempt: &str) -> bool {
    attempt == "verde" || attempt == "azul" || attempt == "vermelho"
}

fn main() {
    let word = String::from("verde");

    if is_a_color_word(&word) {
        //             ^ adicionado para obter `&String`, que é automaticamente
        //               convertido por coerção para `&str` pelo compilador.
        println!("Essa é uma cor que eu conheço!");
    } else {
        println!("Essa não é uma cor que eu conheço.");
    }
}
