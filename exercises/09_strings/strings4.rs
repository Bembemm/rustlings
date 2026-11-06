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
    placeholder("azul");

    placeholder("vermelho".to_string());

    placeholder(String::from("oi"));

    placeholder("rust é divertido!".to_owned());

    placeholder(format!("Interpolação {}", "Estação"));

    // ATENÇÃO: Este índice é baseado em bytes, não em caracteres.
    // Para acessar caracteres, você pode usar `s.chars().nth(INDEX)`.
    placeholder(&String::from("abc")[0..1]);

    placeholder("  olá ".trim());

    placeholder("Feliz segunda-feira!".replace("segunda", "terça"));

    placeholder("mInHa TeClA sHiFt EsTá PrEsA".to_lowercase());
}
