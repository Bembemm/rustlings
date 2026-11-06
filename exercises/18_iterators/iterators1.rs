// Iteradores são essenciais ao realizar operações sobre elementos de uma
// coleção. Este exercício apresenta a estrutura básica de um iterador e como
// percorrer os elementos de uma coleção iterável.

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterators() {
        let my_fav_fruits = &["banana", "custard apple", "avocado", "peach", "raspberry"];

        // TODO: Crie um iterador sobre o slice.
        let mut fav_fruits_iterator = todo!();

        assert_eq!(fav_fruits_iterator.next(), Some(&"banana"));
        assert_eq!(fav_fruits_iterator.next(), todo!()); // TODO: Substitua `todo!()`
        assert_eq!(fav_fruits_iterator.next(), Some(&"avocado"));
        assert_eq!(fav_fruits_iterator.next(), todo!()); // TODO: Substitua `todo!()`
        assert_eq!(fav_fruits_iterator.next(), Some(&"raspberry"));
        assert_eq!(fav_fruits_iterator.next(), todo!()); // TODO: Substitua `todo!()`
    }
}
