# Rustlings em português brasileiro 🦀🇧🇷

Esta é uma tradução comunitária **não oficial** do [Rustlings](https://github.com/rust-lang/rustlings), projeto de exercícios para aprender a linguagem Rust. Recomendamos estudar em paralelo ao [Rust Book](https://doc.rust-lang.org/book/).

O projeto original e seus autores mantêm os créditos; este fork preserva a [licença MIT](LICENSE).

## Progresso da tradução

| Módulo | Situação |
| --- | --- |
| 00 — Introdução | Traduzido |
| 01 — Variáveis | Traduzido |
| 02 — Funções | Traduzido |
| 03 — Condicionais | Traduzido |
| 04 — Tipos primitivos | Traduzido |
| 05 — Vetores | Traduzido |
| 06 — Semântica de movimentação | Traduzido |
| 07 — Estruturas | Traduzido |
| 08 — Enumerações | Traduzido |
| 09 — Strings | Traduzido |
| 10 — Módulos | Traduzido |
| 11 — HashMap | Traduzido |
| 12 — Option | Traduzido |
| 13 — Tratamento de erros | Traduzido |
| 14 — Genéricos | Traduzido |
| 15 — Traits | Traduzido |
| 16 — Lifetimes | Traduzido |
| 17 — Testes | Traduzido |
| 18 — Iteradores | Traduzido |
| 19 — Ponteiros inteligentes | Traduzido |
| 20 — Threads | Traduzido |
| 21 — Macros | Traduzido |
| 22 — Clippy | Traduzido |
| 23 — Conversões | Traduzido |
| 24 — Async | Traduzido |
| Quizzes | Traduzido |
| Dicas (`hint`) | Traduzido |
| Interface principal do terminal | Traduzido |
| Inicialização e lista de exercícios | Traduzido |
| Ferramentas internas de desenvolvimento e site | Ainda parcialmente em inglês |

**Tradução em andamento.** Os módulos 00 a 24 e os quizzes já têm comentários e documentação em português. As mensagens usadas como dados pelos testes foram mantidas em inglês para preservar os resultados esperados. Os erros intencionais dos exercícios foram preservados. Não traduzimos palavras-chave da linguagem, nomes de arquivos ou identificadores essenciais para resolver os desafios.

## Como começar

Clone o fork e execute o Rustlings usando Rust e Cargo:

```sh
git clone https://github.com/Bembemm/rustlings.git
cd rustlings
cargo run
```

Leia os arquivos `README.md` de cada módulo e resolva os desafios `.rs`, sem consultar as soluções antes de tentar. Confira também a [correspondência com os capítulos do livro](exercises/README.md).

**Observação:** os exercícios, READMEs dos módulos, quizzes, mensagens de boas-vindas, 95 dicas do comando `hint` e a interface principal do terminal já estão em português. Partes voltadas a desenvolvimento interno do próprio Rustlings e o site do projeto ainda podem conter textos em inglês.

## Links

- [Site oficial](https://rustlings.rust-lang.org/)
- [Repositório original](https://github.com/rust-lang/rustlings)
- [Livro oficial do Rust](https://doc.rust-lang.org/book/)

## Validação

As alterações preservam palavras-chave, nomes de exercícios e valores usados pelos testes. No momento, o GitHub não registrou checks de CI para os commits desta tradução, então uma validação completa com `cargo test`/`cargo clippy` ainda deve ser executada localmente antes de considerar o fork totalmente validado.
