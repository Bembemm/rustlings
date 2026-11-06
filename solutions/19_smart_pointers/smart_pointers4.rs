// Este exercício explora o ponteiro inteligente `Cow` (*Clone-On-Write*). Ele pode
// armazenar dados emprestados, fornecer acesso imutável e clonar os dados
// apenas quando uma mutação ou a propriedade for necessária. O tipo foi projetado para funcionar
// com dados emprestados de forma genérica por meio da trait `Borrow`.

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            // Clona para um vetor caso os dados ainda não sejam possuídos.
            input.to_mut()[ind] = -value;
        }
    }
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() {
        // O clone ocorre porque `input` precisa ser modificado.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn reference_no_mutation() {
        // Nenhum clone ocorre porque `input` não precisa ser modificado.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Borrowed(_)));
        //                      ^^^^^^^^^^^^^^^^
    }

    #[test]
    fn owned_no_mutation() {
        // Também podemos passar `vec` sem `&`, fazendo com que `Cow` possua os dados diretamente. Nesse
        // caso, não ocorre mutação (todos os números já são absolutos) e portanto
        // também não há clone. Mesmo assim, o resultado continua sendo possuído porque nunca foi
        // emprestado nem modificado.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
        //                      ^^^^^^^^^^^^^
    }

    #[test]
    fn owned_mutation() {
        // Isso também vale quando uma mutação ocorre (nem todos
        // os números são absolutos). Nesse caso, a chamada a `to_mut()` em
        // `abs_all` retorna uma referência para os mesmos dados de antes.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
        //                      ^^^^^^^^^^^^^
    }
}
