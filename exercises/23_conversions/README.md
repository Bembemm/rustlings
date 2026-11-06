# Conversões de tipos

Rust oferece várias maneiras de converter valores de um tipo para outro.

A forma mais simples é uma conversão explícita com o operador `as`. Por exemplo, `println!("{}", 1 + 1.0);` não compila porque `1` é inteiro e `1.0` é ponto flutuante. Já `println!("{}", 1 as f32 + 1.0)` compila. O exercício [`conversions1`](conversions1.rs) aborda esse caso.

Rust também possui traits para conversões:

- `From` e `Into` em [`conversions2`](conversions2.rs);
- `TryFrom` e `TryInto` em [`conversions4`](conversions4.rs);
- `AsRef` e `AsMut` em [`conversions5`](conversions5.rs).

Além disso, `std::str` fornece a trait [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html), que permite converter strings em outros tipos usando `parse`.

Essas são as principais formas de conversão disponíveis na biblioteca padrão.

## Para saber mais

- [Módulo `convert`](https://doc.rust-lang.org/std/convert/index.html)
- [Trait `FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html)
