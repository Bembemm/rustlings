trait Licensed {
    // TODO: Adicione uma implementação padrão para `licensing_info`, permitindo
    // que tipos como as duas structs abaixo compartilhem esse comportamento
    // sem repetir a função.
    // A informação padrão da licença deve ser a string "Default license".
    fn licensing_info(&self) -> String;
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // Não altere esta linha.
impl Licensed for OtherSoftware {} // Não altere esta linha.

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = "Default license";
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
