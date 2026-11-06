# Programação assíncrona

Programação assíncrona é um modelo em que tarefas são delegadas a um *runtime*, que pode executá-las de forma concorrente.

Esse modelo é especialmente eficiente para aplicações que realizam muitas operações independentes de entrada e saída, como servidores web.

Rust fornece os recursos necessários para escrever código assíncrono, mas a biblioteca padrão não inclui um runtime. Nestes exercícios, utilizaremos o runtime `tokio`.

## Para saber mais

- [Fundamentos de programação assíncrona](https://doc.rust-lang.org/book/ch17-00-async-await.html)
- [Documentação do Tokio](https://docs.rs/tokio/latest/tokio/)
