#![allow(clippy::ptr_arg)]

// TODO: Corrija os erros de compilação sem fazer nenhuma alteração além
// de adicionar ou remover referências (o caractere `&`).

// Não deve assumir a propriedade (*ownership*) do valor
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Deve assumir a propriedade (*ownership*) do valor
fn string_uppercase(mut data: &String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(data);

    string_uppercase(&data);
}
