fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];
        //       0  1  2  3  4  <- índices
        //          -------
        //             |
        //             +--- slice

        // Observe que o índice superior 4 não é incluído.
        let nice_slice = &a[1..4];
        assert_eq!([2, 3, 4], nice_slice);

        // O índice superior pode ser incluído usando a sintaxe `..=` (com o sinal `=`).
        let nice_slice = &a[1..=3];
        assert_eq!([2, 3, 4], nice_slice);
    }
}
