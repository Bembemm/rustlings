#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: Corrija o erro de compilação adicionando algo a esta expressão `match`.
    match optional_point {
        Some(p) => println!("As coordenadas são {},{}", p.x, p.y),
        _ => panic!("Nenhuma correspondência!"),
    }

    println!("{optional_point:?}"); // Não altere esta linha.
}
