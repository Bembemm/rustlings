fn current_favorite_color() -> String {
    // Equivalente a `String::from("azul")`
    "azul".to_string()
}

fn main() {
    let answer = current_favorite_color();
    println!("Minha cor favorita no momento é {answer}");
}
