// Durante a compilação, Rust precisa saber quanto espaço um tipo ocupa. Isso
// se torna um problema para tipos recursivos, nos quais um valor pode conter
// outro valor do mesmo tipo. Para contornar isso, podemos usar
// `Box` — um ponteiro inteligente usado para armazenar dados na heap — que também permite
// envolver um tipo recursivo.
//
// O tipo recursivo implementado neste exercício é a "cons list", uma
// estrutura de dados comum em linguagens funcionais. Cada
// item da lista contém dois elementos: o valor atual e
// o próximo item. O último item é representado por `Nil`.

#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn create_empty_list() -> List {
    List::Nil
}

fn create_non_empty_list() -> List {
    List::Cons(42, Box::new(List::Nil))
}

fn main() {
    println!("Esta é uma cons list vazia: {:?}", create_empty_list());
    println!(
        "Esta é uma cons list não vazia: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
