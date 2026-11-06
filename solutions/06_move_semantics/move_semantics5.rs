#![allow(clippy::ptr_arg)]

// Empresta em vez de assumir a propriedade.
// O recomendado aqui seria usar `&str` em vez de `&String`. Mas isto
// é suficiente por enquanto, pois ainda não estudamos strings.
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Assume a propriedade em vez de emprestar.
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust é ótimo!".to_string();

    get_char(&data);

    string_uppercase(data);
}
