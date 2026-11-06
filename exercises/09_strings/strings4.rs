// Substitua as chamadas desta função por chamadas a `string_slice` ou `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Abaixo há vários valores: alguns são `String` e outros são `&str`.
// Substitua cada chamada a `placeholder(…)` por `string_slice(…)`
// ou `string(…)`, de acordo com o tipo de cada valor.
fn main() {
    placeholder("blue");

    placeholder("red".to_string());

    placeholder(String::from("hi"));

    placeholder("rust is fun!".to_owned());

    placeholder(format!("Interpolation {}", "Station"));

    // ATENÇÃO: Este índice é baseado em bytes, não em caracteres.
    // Para acessar caracteres, você pode usar `s.chars().nth(INDEX)`.
    placeholder(&String::from("abc")[0..1]);

    placeholder("  hello there ".trim());

    placeholder("Happy Monday!".replace("Mon", "Tues"));

    placeholder("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
