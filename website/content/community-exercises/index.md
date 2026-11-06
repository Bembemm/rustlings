+++
title = "Exercícios da comunidade"
+++

## Lista de exercícios da comunidade

- 🇯🇵 [Rustlings em japonês](https://github.com/sotanengel/rustlings-jp): tradução japonesa dos exercícios do Rustlings.
- 🇨🇳 [Rustlings em chinês simplificado](https://github.com/SandmeyerX/rustlings-zh-cn): tradução para chinês simplificado.
- 🇺🇦 [Rustlings em ucraniano](https://github.com/noroutine/rustlings-ua): tradução ucraniana dos exercícios.
- 🇰🇷 [Rustlings em coreano](https://github.com/eoncheole/rustlings-kr): tradução coreana dos exercícios.

> Você pode usar o mesmo programa `rustlings`, instalado com `cargo install rustlings`, para executar exercícios da comunidade.

## Criando exercícios da comunidade

O suporte do Rustlings a exercícios da comunidade permite criar exercícios próprios focados em assuntos específicos.
Você também pode oferecer uma tradução dos exercícios oficiais no formato de exercícios da comunidade.

### Começando

Para criar exercícios da comunidade, instale o Rustlings e execute `rustlings dev new PROJECT_NAME`.
De forma semelhante a `cargo new PROJECT_NAME`, esse comando cria o diretório-modelo `PROJECT_NAME` com os arquivos necessários.

_Leia os comentários_ do arquivo `info.toml` gerado para entender o formato.
Nele você pode definir mensagens personalizadas de boas-vindas e conclusão, além dos metadados de cada exercício.

### Criando um exercício

Exemplo de metadados de um exercício:

```toml
[[exercises]]
name = "intro1"
hint = """
Para concluir este exercício, você precisa …
Estes links podem ajudar …"""
```

Depois de adicionar isso ao `info.toml`, crie o arquivo `intro1.rs` no diretório `exercises/`.
O exercício precisa ter uma função `main`, que pode estar vazia.
Adicionar testes é recomendado.
Use os exercícios oficiais do Rustlings como referência.

Opcionalmente, você pode adicionar um arquivo de solução `intro1.rs` ao diretório `solutions/`.

Depois execute `rustlings dev check`.
O comando informará qualquer problema encontrado nos exercícios.
Por exemplo, pode solicitar `rustlings dev update` para atualizar o `Cargo.toml` e incluir o novo exercício `intro1`.

`rustlings dev check` também executa as soluções, quando existirem, para verificar se funcionam corretamente.

Pronto!
Você concluiu seu primeiro exercício 🎉

### Cargo.toml

Com exceção da lista `bin`, você pode alterar o arquivo `Cargo.toml` como quiser.

> A lista `bin` é atualizada automaticamente ao executar `rustlings dev update`

- Você pode adicionar dependências na tabela `[dependencies]`.
- Também pode [configurar lints](https://doc.rust-lang.org/cargo/reference/manifest.html#the-lints-section) para todos os exercícios usando as tabelas `[lints.rust]` e `[lints.clippy]`.

### Publicação

Adicione mais exercícios e publique o projeto como um repositório Git.

Os usuários só precisam clonar esse repositório e executar `rustlings` dentro dele para começar, assim como nos exercícios oficiais.

Uma diferença é que, nos exercícios da comunidade, os arquivos de solução não ficam ocultos até a conclusão do exercício.
Mas você pode confiar que os alunos não vão abrir a solução antes da hora 😉

### Compartilhamento

Depois de publicar seus exercícios da comunidade, abra uma issue ou pull request no [repositório oficial do Rustlings](https://github.com/rust-lang/rustlings) para adicionar seu projeto à [lista de exercícios da comunidade](#lista-de-exercicios-da-comunidade) 😃
