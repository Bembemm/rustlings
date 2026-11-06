# Valores opcionais (`Option`)

O tipo `Option` representa um valor que **pode existir ou não**. Todo `Option` é uma destas duas variantes: `Some(valor)`, quando há um valor, ou `None`, quando não há.

Esse tipo aparece com frequência no Rust e pode ser utilizado para:

- Representar valores inicialmente ausentes;
- Retornar resultados de funções que nem sempre produzem um valor;
- Indicar a ausência de um resultado sem precisar lançar um erro;
- Definir campos opcionais em estruturas;
- Emprestar ou retirar valores de campos de estruturas;
- Criar argumentos opcionais;
- Representar ponteiros que podem ser nulos;
- Substituir temporariamente valores em operações mais complexas.

## Para saber mais

- [Definições de enumerações com `Option`](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)
- [Documentação do módulo `Option`](https://doc.rust-lang.org/std/option/)
- [Documentação da enumeração `Option`](https://doc.rust-lang.org/std/option/enum.Option.html)
- [`if let`](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [`while let`](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)
