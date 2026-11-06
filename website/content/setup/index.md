+++
title = "Configuração"
+++

## Instalando o Rust

Antes de instalar o Rustlings, você precisa ter a **versão mais recente do Rust** instalada.
Visite [www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) para ver as instruções.
Isso também instalará o _Cargo_, gerenciador de pacotes e projetos do Rust.

> 🐧 Se você usa **Linux**, certifique-se de ter o `gcc` instalado (_para atuar como linker_).
>
> Debian: `sudo apt install gcc`\
> Fedora: `sudo dnf install gcc`
>
> 🍎 Se você usa **macOS**, certifique-se de ter o _Xcode e suas ferramentas de desenvolvimento_ instalados: `xcode-select --install`

## Instalando o Rustlings

O comando abaixo baixa e compila o Rustlings:

```bash
cargo install rustlings
```

{% <details summary="Se a instalação falhar…"> %}

- Confirme que você está usando a versão mais recente do Rust executando `rustup update`
- Tente adicionar a flag `--locked`: `cargo install rustlings --locked`
- Se ainda falhar, [relate o problema](https://github.com/rust-lang/rustlings/issues/new)

{% </details> %}

## Inicialização

Depois de instalar o Rustlings, execute o comando abaixo para criar o diretório `rustlings/`:

```bash
rustlings init
```

{% <details summary="Se o comando <code>rustlings</code> não for encontrado…"> %}

Provavelmente você está no Linux e instalou o Rust usando o gerenciador de pacotes da distribuição.

O Cargo instala executáveis no diretório `~/.cargo/bin`.
Infelizmente, alguns gerenciadores de pacotes não adicionam `~/.cargo/bin` à variável de ambiente `PATH`.

- Adicione `~/.cargo/bin` manualmente ao `PATH`
- Ou desinstale o Rust instalado pelo gerenciador de pacotes e [instale-o pelo método oficial usando `rustup`](https://www.rust-lang.org/tools/install)

{% </details> %}

Agora entre no diretório recém-criado e execute o Rustlings para receber as próximas instruções:

```bash
cd rustlings/
rustlings
```

## Ambiente de trabalho

### Editor

Nossa recomendação geral é o [VS Code](https://code.visualstudio.com/) com a extensão [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
Mas qualquer editor compatível com [rust-analyzer](https://rust-analyzer.github.io/) deve funcionar bem para os exercícios.

### Terminal

Ao usar o Rustlings, prefira um terminal moderno para ter a melhor experiência.
O terminal padrão do Linux e do macOS normalmente é suficiente.
No Windows, recomendamos o [Windows Terminal](https://aka.ms/terminal).

### Documentação offline

Se estiver usando o Rustlings sem internet, você pode abrir cópias locais do livro ou da documentação da biblioteca padrão com `rustup doc --book` ou `rustup doc --std`.

## Uso

Depois da configuração, visite a página de [**uso**](@/usage/index.md) para aprender como trabalhar com o Rustlings 🚀
