// Este wrapper armazena atualmente um inteiro positivo.
// TODO: Reescreva-o usando genéricos para que possa armazenar QUALQUER tipo.
struct Wrapper {
    value: u32,
}

// TODO: Adapte a implementação da struct para ser genérica sobre o valor armazenado.
impl Wrapper {
    fn new(value: u32) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
