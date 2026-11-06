// Esta função informa quanto sorvete ainda resta na geladeira.
// Antes das 22h (relógio de 24 horas), restam 5 bolas de sorvete. Às 22h,
// alguém come tudo, e a quantidade passa a ser 0.
// Retorne `None` se `hour_of_day` for maior que 23.
fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    // TODO: Complete o corpo da função.
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // TODO: Corrija este teste. Como você obtém o valor contido em um
        // `Option`?
        let ice_creams = maybe_ice_cream(12);

        assert_eq!(ice_creams, 5); // Não altere esta linha.
    }

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(0), Some(5));
        assert_eq!(maybe_ice_cream(9), Some(5));
        assert_eq!(maybe_ice_cream(18), Some(5));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(24), None);
        assert_eq!(maybe_ice_cream(25), None);
    }
}
