use anyhow::{Context, Result, bail};
use app_state::StateFileStatus;
use clap::Parser;
use std::{
    env,
    io::{self, IsTerminal, Write},
    path::Path,
    process::ExitCode,
};
use term::{clear_terminal, press_enter_prompt};

use crate::{
    app_state::AppState,
    cli::{Args, Command},
    editor::Editor,
    info_file::InfoFile,
};

mod app_state;
mod cargo_toml;
mod cli;
mod cmd;
mod dev;
mod editor;
mod embedded;
mod exercise;
mod info_file;
mod init;
mod list;
mod run;
mod term;
mod watch;

const CURRENT_FORMAT_VERSION: u8 = 1;

fn main() -> Result<ExitCode> {
    let args = Args::parse();

    if cfg!(not(debug_assertions)) && Path::new("dev/rustlings-repo.txt").exists() {
        bail!("{OLD_METHOD_ERR}");
    }

    'priority_cmd: {
        match args.command {
            Some(Command::Init) => init::init().context("Falha ao inicializar")?,
            Some(Command::Dev(dev_command)) => dev_command.run()?,
            _ => break 'priority_cmd,
        }

        return Ok(ExitCode::SUCCESS);
    }

    if !Path::new("exercises").is_dir() {
        println!("{PRE_INIT_MSG}");
        return Ok(ExitCode::FAILURE);
    }

    let info_file = InfoFile::parse()?;

    if info_file.format_version > CURRENT_FORMAT_VERSION {
        bail!(FORMAT_VERSION_HIGHER_ERR);
    }

    let vs_code_term = env::var_os("TERM_PROGRAM").is_some_and(|v| v == "vscode");
    let editor = if args.no_editor {
        None
    } else {
        Editor::new(args.edit_cmd, vs_code_term)?
    };

    let (mut app_state, state_file_status) = AppState::new(
        info_file.exercises,
        info_file.final_message,
        editor,
        vs_code_term,
    )?;

    // Show the welcome message if the state file doesn't exist yet.
    let welcome_message = info_file.welcome_message.as_bytes().trim_ascii();
    if !welcome_message.is_empty() {
        match state_file_status {
            StateFileStatus::NotRead => {
                let mut stdout = io::stdout().lock();
                clear_terminal(&mut stdout)?;

                let welcome_message = welcome_message.trim_ascii();
                stdout.write_all(welcome_message)?;
                stdout.write_all("\n\nPressione ENTER para continuar ".as_bytes())?;
                press_enter_prompt(&mut stdout)?;
                clear_terminal(&mut stdout)?;
                // Flush to be able to show errors occurring before printing a newline to stdout.
                stdout.flush()?;
            }
            StateFileStatus::Read => (),
        }
    }

    match args.command {
        None => {
            if !io::stdout().is_terminal() {
                bail!("Terminal/TTY ausente ou não suportado");
            }

            let notify_exercise_names = if args.manual_run {
                None
            } else {
                // LEAKING: For the notify event handler thread. The slice is used until the end of the program.
                Some(
                    &*app_state
                        .exercises()
                        .iter()
                        .map(|exercise| exercise.name.as_bytes())
                        .collect::<Vec<_>>()
                        .leak(),
                )
            };

            watch::watch(&mut app_state, notify_exercise_names)?;
            app_state.close_editor()?;
        }
        Some(Command::Run { name }) => {
            if let Some(name) = name {
                app_state.set_current_exercise_by_name(&name)?;
            }
            return run::run(&mut app_state);
        }
        Some(Command::CheckAll) => {
            let mut stdout = io::stdout().lock();
            if let Some(first_pending_exercise_ind) = app_state.check_all_exercises(&mut stdout)? {
                if app_state.current_exercise().done {
                    app_state.set_current_exercise_ind(first_pending_exercise_ind)?;
                }

                stdout.write_all(b"\n\n")?;
                let pending = app_state.n_pending();
                if pending == 1 {
                    stdout.write_all("Um exercício pendente: ".as_bytes())?;
                } else {
                    write!(
                        stdout,
                        "{pending}/{} exercícios pendentes. O primeiro: ",
                        app_state.exercises().len(),
                    )?;
                }
                app_state
                    .current_exercise()
                    .terminal_file_link(&mut stdout, app_state.emit_file_links())?;
                stdout.write_all(b"\n")?;

                return Ok(ExitCode::FAILURE);
            }

            app_state.render_final_message(&mut stdout)?;
        }
        Some(Command::Reset { name }) => {
            app_state.set_current_exercise_by_name(&name)?;
            app_state.reset_current_exercise()?;

            let current_exercise = app_state.current_exercise();
            let mut stdout = io::stdout().lock();
            stdout.write_all("O exercício ".as_bytes())?;
            current_exercise.terminal_file_link(&mut stdout, app_state.emit_file_links())?;
            stdout.write_all(" foi restaurado\n".as_bytes())?;
        }
        Some(Command::Hint { name }) => {
            if let Some(name) = name {
                app_state.set_current_exercise_by_name(&name)?;
            }

            let current_exercise = app_state.current_exercise();
            let mut stdout = io::stdout().lock();
            stdout.write_all("Exercício atual: ".as_bytes())?;
            current_exercise.terminal_file_link(&mut stdout, app_state.emit_file_links())?;

            stdout.write_all("\n\nDica:\n".as_bytes())?;
            stdout.write_all(current_exercise.hint.as_bytes().trim_ascii())?;
            stdout.write_all(b"\n")?;
        }
        // Handled in an earlier match.
        Some(Command::Init | Command::Dev(_)) => (),
    }

    Ok(ExitCode::SUCCESS)
}

const OLD_METHOD_ERR: &str =
    "Você está tentando executar o Rustlings usando o método antigo, anterior à versão 6.\nO método atual não exige clonar o repositório do Rustlings.\nSiga as instruções no `README.md`:\nhttps://github.com/rust-lang/rustlings#getting-started";

const FORMAT_VERSION_HIGHER_ERR: &str =
    "A versão de formato indicada em `info.toml` é mais nova que a suportada por esta versão.\nÉ possível que seu Rustlings esteja desatualizado.\nInstale a versão mais recente do Rustlings e tente novamente.";

const PRE_INIT_MSG: &str = r"
       Bem-vindo ao...
                 _   _ _
  _ __ _   _ ___| |_| (_)_ __   __ _ ___
 | '__| | | / __| __| | | '_ \ / _` / __|
 | |  | |_| \__ \ |_| | | | | | (_| \__ \
 |_|   \__,_|___/\__|_|_|_| |_|\__, |___/
                               |___/

A pasta `exercises/` não foi encontrada no diretório atual.\nSe você está começando agora, execute `rustlings init` para inicializar os exercícios.";
