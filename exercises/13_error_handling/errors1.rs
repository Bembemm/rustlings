// TODO: Esta função se recusa a gerar o texto de um crachá quando recebe
// uma string vazia. Seria melhor explicar qual foi o problema em vez de
// simplesmente retornar `None`. Rust possui uma construção semelhante a
// `Option` que também representa erros. Altere a assinatura e o corpo da
// função para retornar `Result<String, String>` em vez de `Option<String>`.
fn generate_nametag_text(name: String) -> Option<String> {
    if name.is_empty() {
        // Nomes vazios não são permitidos
        None
    } else {
        Some(format!("Hi! My name is {name}"))
    }
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Empty names aren't allowed"),
        );
    }
}
