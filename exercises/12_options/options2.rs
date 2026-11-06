fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Transforme isto em uma instrução `if let` que corresponda a `Some`.
        word = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Transforme isto em uma instrução `while let`. Lembre-se de que
        // `Vec::pop()` adiciona outra camada de `Option`. Você pode usar
        // correspondência de padrões aninhados em `if let` e `while let`.
        integer = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
