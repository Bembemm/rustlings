// Este quiz revisa os seguintes assuntos:
// - Strings
// - Vetores (`Vec`)
// - Semântica de movimentação
// - Módulos
// - Enumerações
//
// Vamos construir uma pequena máquina em forma de função. A entrada será uma
// lista de strings e comandos. Cada comando define uma transformação:
// - Converter a string para maiúsculas;
// - Remover espaços nas extremidades;
// - Adicionar "bar" ao final uma determinada quantidade de vezes.
//
// A entrada será um vetor de tuplas com dois elementos: a string e o comando.
// A saída será um vetor de strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete a função conforme descrito acima.
    // pub fn transformer(input: ???) -> ??? { ??? }
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    // TODO: O que precisamos importar para que `transformer` fique no escopo?
    // use ???;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
