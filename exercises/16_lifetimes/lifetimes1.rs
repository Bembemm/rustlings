// O compilador Rust precisa verificar se as referências recebidas continuam
// válidas, para avisar quando uma referência pode sair de escopo antes de ser
// utilizada. Lembre-se: referências são empréstimos e não possuem os dados.
// O que acontece se o dono desses dados sair de escopo?

// TODO: Corrija o erro de compilação atualizando a assinatura da função.
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}
