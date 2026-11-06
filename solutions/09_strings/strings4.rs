fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

fn main() {
    string_slice("azul");

    string("vermelho".to_string());

    string(String::from("oi"));

    string("rust é divertido!".to_owned());

    string(format!("Interpolação {}", "Estação"));

    // ATENÇÃO: esta indexação é por bytes, não por caracteres.
    // Para acessar caracteres, você pode usar `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]);

    string_slice("  olá ".trim());

    string("Feliz segunda-feira!".replace("segunda", "terça"));

    string("mInHa TeClA sHiFt EsTá PrEsA".to_lowercase());
}
