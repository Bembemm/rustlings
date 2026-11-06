fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Complete esta função para retornar o maior número!
    // Se os números forem iguais, você pode retornar qualquer um deles.
    // Não utilize:
    // - chamadas a outras funções
    // - variáveis adicionais
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

// Não se preocupe com esta parte por enquanto :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
