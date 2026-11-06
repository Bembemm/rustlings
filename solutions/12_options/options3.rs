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
        Some(ref p) => println!("Coordinates are {},{}", p.x, p.y),
        //   ^^^ adicionado
        _ => panic!("No match!"),
    }

    // Solução 2: fazer o `match` sobre uma referência (`&Option`) adicionando `&` antes de
    // `optional_point`.
    match &optional_point {
        //^ adicionado
        Some(p) => println!("Coordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    }

    println!("{optional_point:?}");
}
