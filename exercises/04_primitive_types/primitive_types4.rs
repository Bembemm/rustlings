fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Obtenha um slice chamado `nice_slice` a partir do array `a` para que o teste passe.
        // let nice_slice = ???

        assert_eq!([2, 3, 4], nice_slice);
    }
}
