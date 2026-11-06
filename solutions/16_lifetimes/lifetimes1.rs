// O compilador Rust precisa saber como verificar se as referências recebidas são
// válidas, para poder avisar quando uma referência corre o risco de
// sair de escopo antes de ser usada. Lembre-se: referências são empréstimos e
// não possuem os próprios dados. O que acontece se o dono sair de escopo?

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //    ^^^^     ^^          ^^          ^^
    if x.len() > y.len() { x } else { y }
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
