// São apresentadas 3 soluções possíveis.

// Com um laço `for` e uma variável mutável.
fn factorial_for(num: u64) -> u64 {
    let mut result = 1;

    for x in 2..=num {
        result *= x;
    }

    result
}

// Equivalente a `factorial_for`, mas mais curta e sem um laço `for` nem
// variáveis mutáveis.
fn factorial_fold(num: u64) -> u64 {
    // Caso num==0: o iterador 2..=0 está vazio
    //              -> o valor inicial de `fold`, que é 1, é retornado.
    // Caso num==1: o iterador 2..=1 também está vazio
    //              -> o valor inicial 1 é retornado.
    // Caso num==2: o iterador 2..=2 contém um elemento
    //              -> o valor inicial 1 é multiplicado por 2 e o resultado
    //                 é retornado.
    // Caso num==3: o iterador 2..=3 contém 2 elementos
    //              -> calcula-se 1 * 2; depois, o resultado 2 é multiplicado pelo
    //                 segundo elemento, 3, e o resultado 6 é retornado.
    // E assim por diante…
    #[allow(clippy::unnecessary_fold)]
    (2..=num).fold(1, |acc, x| acc * x)
}

// Equivalente a `factorial_fold`, mas usando um método nativo sugerido
// pelo Clippy.
fn factorial_product(num: u64) -> u64 {
    (2..=num).product()
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial_for(0), 1);
        assert_eq!(factorial_fold(0), 1);
        assert_eq!(factorial_product(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial_for(1), 1);
        assert_eq!(factorial_fold(1), 1);
        assert_eq!(factorial_product(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial_for(2), 2);
        assert_eq!(factorial_fold(2), 2);
        assert_eq!(factorial_product(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial_for(4), 24);
        assert_eq!(factorial_fold(4), 24);
        assert_eq!(factorial_product(4), 24);
    }
}
