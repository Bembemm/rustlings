# Lifetimes

*Lifetimes* ajudam o compilador a verificar se referências continuam válidas pelo tempo necessário.

Por exemplo, uma anotação de lifetime pode expressar algo como: "garanta que a referência `'a` permaneça válida por tempo suficiente para que o valor retornado continue seguro".

Lifetimes são relevantes para empréstimos, isto é, referências. Valores copiados ou movidos passam a pertencer ao novo escopo e seguem regras diferentes.

As anotações de lifetime permitem ao compilador verificar se os argumentos usados por uma função permanecem válidos durante o uso das referências retornadas.

Se quiser praticar ainda mais, o projeto [lifetimekata](https://tfpk.github.io/lifetimekata/) possui exercícios no estilo do Rustlings focados especificamente em lifetimes.

## Para saber mais

- [Lifetimes no Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [Validando referências com lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
