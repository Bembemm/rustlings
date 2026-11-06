// Vamos construir uma pequena máquina na forma de uma função. Como entrada, vamos
// fornecer uma lista de strings e comandos. Esses comandos determinam qual ação
// será aplicada à string. Pode ser:
// - Converter a string para maiúsculas
// - Remover espaços nas extremidades
// - Adicionar "bar" à string uma quantidade especificada de vezes
//
// O formato será:
// - A entrada será um vetor de tuplas com 2 elementos,
//   o primeiro é a string e o segundo é o comando.
// - A saída será um vetor de strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // Esta é a solução com um laço. Veja `transformer_iter` para uma versão
    // usando iteradores.
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output = Vec::new();

        for (string, command) in input {
            // Crie a nova string.
            let new_string = match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(n) => string + &"bar".repeat(n),
            };

            // Adicione a nova string ao vetor de saída.
            output.push(new_string);
        }

        output
    }

    // Equivalente a `transform`, mas usa um iterador em vez de um laço para
    // comparação. Não se preocupe: praticaremos iteradores mais adiante ;)
    pub fn transformer_iter(input: Vec<(String, Command)>) -> Vec<String> {
        input
            .into_iter()
            .map(|(string, command)| match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(n) => string + &"bar".repeat(n),
            })
            .collect()
    }
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    // Importe `transformer`.
    use super::my_module::transformer;

    use super::Command;
    use super::my_module::transformer_iter;

    #[test]
    fn it_works() {
        for transformer in [transformer, transformer_iter] {
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
}
