// Calcula uma potência de 2 usando deslocamento de bits.
// `1 << n` equivale a "2 elevado a n".
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // TODO: Teste a função `power_of_2` com alguns valores.
        assert_eq!();
        assert_eq!();
        assert_eq!();
        assert_eq!();
    }
}
