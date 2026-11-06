// Este exercício explora o ponteiro inteligente `Cow` (*Clone-On-Write*).
// Ele pode fornecer acesso imutável a dados emprestados e cloná-los somente
// quando uma mutação ou a posse dos dados for necessária. O tipo funciona com
// dados emprestados de forma genérica por meio da trait `Borrow`.

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
        // TODO: Substitua `todo!()` por `Cow::Owned(_)` ou `Cow::Borrowed(_)`.
        assert!(matches!(input, todo!()));
    }

    #[test]
    fn owned_no_mutation() {
        // Também podemos passar `vec` sem `&`, fazendo com que `Cow` possua os dados
        // diretamente. Neste caso não há mutação e também não há clone, mas o
        // resultado continua sendo `Owned`, pois os dados nunca foram emprestados.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: Substitua `todo!()` por `Cow::Owned(_)` ou `Cow::Borrowed(_)`.
        assert!(matches!(input, todo!()));
    }

    #[test]
    fn owned_mutation() {
        // Isso também vale quando ocorre uma mutação. Como os dados já pertencem ao
        // `Cow`, `to_mut()` pode retornar uma referência mutável para os mesmos
        // dados, sem precisar cloná-los.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: Substitua `todo!()` por `Cow::Owned(_)` ou `Cow::Borrowed(_)`.
        assert!(matches!(input, todo!()));
    }
}
