fn factorial(num: u64) -> u64 {
    // TODO: Complete esta função para retornar o fatorial de `num`, definido
    // como `1 * 2 * 3 * … * num`.
    // https://en.wikipedia.org/wiki/Factorial
    //
    // Não use:
    // - retornos antecipados (usando explicitamente a palavra-chave `return`)
    // Tente não usar:
    // - laços imperativos (`for`/`while`)
    // - variáveis adicionais
    // Para um desafio extra, não use:
    // - recursão
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
