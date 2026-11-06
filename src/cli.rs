use clap::{Parser, Subcommand};

use crate::dev::DevCommand;

/// Rustlings é uma coleção de pequenos exercícios para praticar leitura e escrita de código Rust
#[derive(Parser)]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
    /// Desativa a abertura automática do exercício atual no VS Code ou Zellij.
    /// Ignora `--edit-cmd`
    #[arg(long)]
    pub no_editor: bool,
    /// Abre o exercício atual executando `EDIT_CMD EXERCISE_PATH`.
    /// O comando não pode bloquear o processo (por exemplo, `vim`).
    /// Ele deve se comunicar com um editor executado em outro processo.
    /// `EDIT_CMD` pode conter argumentos, como `--edit-cmd "PROGRAM -x --arg1"`.
    /// O caminho do exercício atual é acrescentado pelo Rustlings como último argumento.
    /// `--edit-cmd` é ignorado no VS Code.
    ///
    /// Exemplo: `--edit-cmd "code"` (comportamento padrão em um terminal do VS Code)
    #[arg(long)]
    pub edit_cmd: Option<String>,
    /// Executa manualmente o exercício atual com `r` no modo de acompanhamento.
    /// Use apenas se o Rustlings não conseguir detectar alterações nos arquivos
    #[arg(long)]
    pub manual_run: bool,
}

#[derive(Subcommand)]
pub enum Command {
    /// Inicializa os exercícios oficiais do Rustlings
    Init,
    /// Executa um único exercício.
    /// Se nenhum nome for informado, executa o próximo exercício pendente
    Run {
        /// Nome do exercício
        name: Option<String>,
    },
    /// Verifica todos os exercícios e os marca como concluídos ou pendentes
    CheckAll,
    /// Restaura um único exercício
    Reset {
        /// Nome do exercício
        name: String,
    },
    /// Exibe uma dica.
    /// Se nenhum nome for informado, mostra a dica do próximo exercício pendente
    Hint {
        /// Nome do exercício
        name: Option<String>,
    },
    /// Comandos para desenvolver exercícios comunitários do Rustlings
    #[command(subcommand)]
    Dev(DevCommand),
}
