fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: Use o índice da tupla para acessar o segundo elemento de `numbers`
        // e atribuí-lo a uma variável chamada `second`.
        // let second = ???;

        assert_eq!(second, 2, "Este não é o segundo número da tupla!");
    }
}
