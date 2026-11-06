# Traits

Uma `trait` é um conjunto de métodos que descreve um comportamento.

Tipos de dados podem implementar traits. Para isso, os métodos definidos pela trait são implementados para aquele tipo. Por exemplo, `String` implementa `From<&str>`, permitindo escrever `String::from("hello")`.

Nesse sentido, traits têm algumas semelhanças com interfaces em Java e classes abstratas em C++.

Algumas traits comuns em Rust incluem:

- `Clone` — fornece o método `clone`;
- `Display` — permite formatação com `{}`;
- `Debug` — permite formatação com `{:?}`.

Como traits descrevem comportamentos compartilhados entre tipos, elas são especialmente úteis junto com genéricos.

## Para saber mais

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
