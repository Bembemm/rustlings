// Não altere esta função.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // TODO: Corrija o erro de compilação movendo apenas uma linha.

    let string1 = String::from("uma string longa é longa");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
    }
    println!("A string mais longa é '{result}'");
}
