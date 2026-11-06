// Testes são importantes para verificar se o código realmente faz aquilo
// que você espera.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    // TODO: Importe `is_even`. Você pode usar um curinga para importar tudo do
    // módulo externo.

    #[test]
    fn you_can_assert() {
        // TODO: Teste a função `is_even` com alguns valores.
        assert!();
        assert!();
    }
}
