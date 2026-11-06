// Durante a compilação, Rust precisa saber quanto espaço cada tipo ocupa.
// Isso é um problema para tipos recursivos, nos quais um valor contém outro
// valor do mesmo tipo. Uma solução é usar `Box`, um ponteiro inteligente que
// armazena dados na heap e permite envolver tipos recursivos.
//
// Neste exercício, implementaremos uma "cons list", estrutura comum em
// linguagens funcionais. Cada item contém o valor atual e o próximo item.
// O último elemento é representado por `Nil`.

// TODO: Use `Box` na definição da enum para que o código compile.
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, List),
    Nil,
}

// TODO: Crie uma cons list vazia.
fn create_empty_list() -> List {
    todo!()
}

// TODO: Crie uma cons list não vazia.
fn create_non_empty_list() -> List {
    todo!()
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
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
