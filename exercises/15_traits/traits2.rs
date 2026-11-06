trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implemente a trait `AppendBar` para um vetor de strings.
// `append_bar` deve adicionar a string "Bar" ao vetor.

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
