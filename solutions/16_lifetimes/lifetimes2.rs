fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("uma string longa é longa");
    // Solução 1: você pode mover `strings2` para fora do bloco interno para que ela
    // não seja descartada antes da instrução de impressão.
    let string2 = String::from("xyz");
    let result;
    {
        result = longest(&string1, &string2);
    }
    println!("A string mais longa é '{result}'");
    // `string2` é descartada no fim da função.

    // =========================================================================

    let string1 = String::from("uma string longa é longa");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        // Solução 2: você pode mover a instrução de impressão para dentro do bloco interno para
        // que ela seja executada antes de `string2` ser descartada.
        println!("A string mais longa é '{result}'");
        // `string2` é descartada aqui (fim do escopo interno).
    }
}
