use anyhow::{Context, Result, bail};
use crossterm::{
    QueueableCommand,
    style::{Attribute, Color, ResetColor, SetAttribute, SetForegroundColor},
};
use std::{
    env::{current_dir, set_current_dir},
    fs::{self, create_dir},
    io::{self, Write},
    path::Path,
    process::{Command, Stdio},
};

use crate::{
    cargo_toml::updated_cargo_toml, embedded::EMBEDDED_FILES, exercise::RunnableExercise,
    info_file::InfoFile, term::press_enter_prompt,
};

pub fn init() -> Result<()> {
    let rustlings_dir = Path::new("rustlings");
    if rustlings_dir.exists() {
        bail!(RUSTLINGS_DIR_ALREADY_EXISTS_ERR);
    }

    let mut locate_project_output = Command::new("cargo")
        .arg("locate-project")
        .arg("-q")
        .arg("--workspace")
        .arg("--message-format=plain")
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .context(
            "Falha ao executar `cargo locate-project …`\n\
             O Rust já está instalado?\n\
             Execute `cargo --version` para diagnosticar o problema.",
        )?;

    if !Command::new("cargo")
        .arg("clippy")
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("Falha ao executar `cargo clippy --version`")?
        .success()
    {
        bail!(
            "O Clippy, linter oficial do Rust, não foi encontrado.\n\
             Instale-o antes de inicializar o Rustlings."
        )
    }

    let mut stdout = io::stdout().lock();
    let mut init_git = true;

    if locate_project_output.status.success() {
        if Path::new("exercises").exists() && Path::new("solutions").exists() {
            bail!(IN_INITIALIZED_DIR_ERR);
        }

        // Remove newline
        locate_project_output.stdout.pop();

        let workspace_manifest = String::try_from(locate_project_output.stdout)
            .context("Falha ao converter a saída de `cargo locate-project …` em uma string")?;

        let workspace_manifest_content = fs::read_to_string(&workspace_manifest)
            .with_context(|| format!("Falha ao ler o arquivo {workspace_manifest}"))?;
        if !workspace_manifest_content.contains("[workspace]")
            && !workspace_manifest_content.contains("workspace.")
        {
            bail!(
                "O diretório atual já faz parte de um projeto Cargo.\n\
                 Inicialize o Rustlings em outro diretório"
            );
        }

        stdout.write_all("Este comando criará a pasta `rustlings/` como membro deste workspace Cargo.\nPressione ENTER para continuar ".as_bytes())?;
        press_enter_prompt(&mut stdout)?;

        // Make sure "rustlings" is added to `workspace.members` by making
        // Cargo initialize a new project.
        let status = Command::new("cargo")
            .arg("new")
            .arg("-q")
            .arg("--vcs")
            .arg("none")
            .arg("rustlings")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .status()?;
        if !status.success() {
            bail!(
                "Falha ao inicializar um novo membro do workspace Cargo.\n\
                 Inicialize o Rustlings em outro diretório"
            );
        }

        stdout.write_all("A pasta `rustlings` foi adicionada a `workspace.members` no `Cargo.toml` deste workspace.\n".as_bytes())?;
        fs::remove_dir_all(rustlings_dir)
            .context("Falha ao remover o diretório temporário `rustlings/`")?;
        init_git = false;
    } else {
        stdout.write_all("Este comando criará a pasta `rustlings/`, que conterá os exercícios.\nPressione ENTER para continuar ".as_bytes())?;
        press_enter_prompt(&mut stdout)?;
    }

    create_dir(rustlings_dir).context("Falha ao criar o diretório `rustlings/`")?;
    set_current_dir(rustlings_dir)
        .context("Falha ao mudar o diretório atual para `rustlings/`")?;

    let info_file = InfoFile::parse()?;
    EMBEDDED_FILES
        .init_exercises_dir(&info_file.exercises)
        .context("Falha ao inicializar o diretório `rustlings/exercises`")?;

    create_dir("solutions").context("Falha ao criar o diretório `solutions/`")?;
    fs::write(
        "solutions/README.md",
        include_bytes!("../solutions/README.md"),
    )
    .context("Falha ao criar o arquivo rustlings/solutions/README.md")?;
    for dir in EMBEDDED_FILES.exercise_dirs {
        let mut dir_path = String::with_capacity(10 + dir.name.len());
        dir_path.push_str("solutions/");
        dir_path.push_str(dir.name);
        create_dir(&dir_path)
            .with_context(|| format!("Falha ao criar o diretório {dir_path}"))?;
    }
    for exercise_info in &info_file.exercises {
        let solution_path = exercise_info.sol_path();
        fs::write(&solution_path, INIT_SOLUTION_FILE)
            .with_context(|| format!("Falha ao criar o arquivo {solution_path}"))?;
    }

    let current_cargo_toml = include_str!("../dev-Cargo.toml");
    // Skip the first line (comment).
    let newline_ind = current_cargo_toml
        .as_bytes()
        .iter()
        .position(|c| *c == b'\n')
        .context("O `Cargo.toml` incorporado está vazio ou contém apenas uma linha")?;
    let current_cargo_toml = current_cargo_toml
        .get(newline_ind + 1..)
        .context("O `Cargo.toml` incorporado contém apenas uma linha")?;
    let updated_cargo_toml = updated_cargo_toml(&info_file.exercises, current_cargo_toml, b"")
        .context("Falha ao gerar `Cargo.toml`")?;
    fs::write("Cargo.toml", updated_cargo_toml)
        .context("Falha ao criar o arquivo `rustlings/Cargo.toml`")?;

    fs::write("rust-analyzer.toml", RUST_ANALYZER_TOML)
        .context("Falha ao criar o arquivo `rustlings/rust-analyzer.toml`")?;

    fs::write(".gitignore", GITIGNORE)
        .context("Falha ao criar o arquivo `rustlings/.gitignore`")?;

    fs::write("README.md", README).context("Falha ao criar o arquivo `rustlings/README.md`")?;

    create_dir(".vscode").context("Falha ao criar o diretório `rustlings/.vscode`")?;
    fs::write(".vscode/extensions.json", VS_CODE_EXTENSIONS_JSON)
        .context("Falha ao criar o arquivo `rustlings/.vscode/extensions.json`")?;

    if init_git && let Ok(dir) = current_dir() {
        let mut dir = dir.as_path();

        loop {
            if dir.join(".git").exists() || dir.join(".jj").exists() {
                break;
            }

            if let Some(parent) = dir.parent() {
                dir = parent;
            } else {
                // Ignore any Git error because Git initialization is not required.
                let _ = Command::new("git")
                    .arg("init")
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();
                break;
            }
        }
    }

    stdout.queue(SetForegroundColor(Color::Green))?;
    stdout.write_all("Inicialização concluída ✓".as_bytes())?;
    stdout.queue(ResetColor)?;
    stdout.write_all(b"\n\n")?;

    stdout.queue(SetAttribute(Attribute::Bold))?;
    stdout.write_all(POST_INIT_MSG.as_bytes())?;
    stdout.queue(ResetColor)?;

    Ok(())
}

const INIT_SOLUTION_FILE: &str = "fn main() {\n    // NÃO EDITE ESTE ARQUIVO DE SOLUÇÃO!\n    // Ele será preenchido automaticamente após você concluir o exercício.\n}\n";

pub const RUST_ANALYZER_TOML: &[u8] = br#"check.command = "clippy"
check.extraArgs = ["--profile", "test"]
cargo.targetDir = true
"#;

const GITIGNORE: &[u8] = b"Cargo.lock
target/
.vscode/
";

const README: &str = "# Rustlings\n\nEste é o seu espaço para resolver os exercícios do Rustlings.\nExecute `rustlings` neste diretório para começar.\nSaiba mais sobre o uso do Rustlings em:\n<https://rustlings.rust-lang.org/usage/>\n";

pub const VS_CODE_EXTENSIONS_JSON: &[u8] = br#"{"recommendations":["rust-lang.rust-analyzer"]}"#;

const IN_INITIALIZED_DIR_ERR: &str = "Parece que o Rustlings já foi inicializado neste diretório.\n\nSe você já o inicializou, execute `rustlings` para começar os exercícios.\nCaso contrário, execute `rustlings init` novamente em outro diretório.";

const RUSTLINGS_DIR_ALREADY_EXISTS_ERR: &str =
    "Já existe uma pasta chamada `rustlings` no diretório atual.\nProvavelmente o Rustlings já foi inicializado.\nExecute `cd rustlings`\nDepois execute `rustlings` novamente";

const POST_INIT_MSG: &str = "Execute `cd rustlings` para entrar no diretório criado.\nDepois execute `rustlings` para começar.\n";
