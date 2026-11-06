// Testes são importantes para garantir que o código realmente faça o que você
// espera.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    // Ao escrever testes unitários, é comum importar tudo do módulo externo
    // (`super`) usando um curinga.
    use super::*;

    #[test]
    fn you_can_assert() {
        assert!(is_even(0));
        assert!(!is_even(-1));
        //      ^ Você pode verificar `false` usando o operador de negação `!`.
    }
}
