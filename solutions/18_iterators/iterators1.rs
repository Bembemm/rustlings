// Ao realizar operações sobre elementos de uma coleção, iteradores são
// essenciais. Este módulo ajuda você a se familiarizar com a estrutura de uso de um
// iterador e com a forma de percorrer os elementos de uma coleção iterável.

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterators() {
        let my_fav_fruits = &["banana", "custard apple", "avocado", "peach", "raspberry"];

        // Crie um iterador sobre o slice.
        let mut fav_fruits_iterator = my_fav_fruits.iter();

        assert_eq!(fav_fruits_iterator.next(), Some(&"banana"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"custard apple"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"avocado"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"peach"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"raspberry"));
        assert_eq!(fav_fruits_iterator.next(), None);
        //                                     ^^^^ chegou ao fim
    }
}
