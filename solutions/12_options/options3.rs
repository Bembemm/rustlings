#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // Solução 1: fazer o `match` sobre `Option` (não `&Option`) sem mover
    // o valor para fora da variante `Some`.
    match optional_point {
        Some(ref p) => println!("As coordenadas são {},{}", p.x, p.y),
        //   ^^^ adicionado
        _ => panic!("Nenhuma correspondência!"),
    }

    // Solução 2: fazer o `match` sobre uma referência (`&Option`) adicionando `&` antes de
    // `optional_point`.
    match &optional_point {
        //^ adicionado
        Some(p) => println!("As coordenadas são {},{}", p.x, p.y),
        _ => panic!("Nenhuma correspondência!"),
    }

    println!("{optional_point:?}");
}
