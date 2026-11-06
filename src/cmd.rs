use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::{
    io::{Read, pipe},
    path::PathBuf,
    process::{Child, Command, Stdio},
    thread,
    time::Duration,
};
use wait_timeout::ChildExt;

const TIMEOUT_SECS: u64 = 30;

/// Run a command with a description for a possible error and append the merged stdout and stderr.
/// The boolean in the returned `Result` is true if the command's exit status is success.
fn run_cmd(
    mut cmd: Command,
    description: &str,
    cwd: Option<&str>,
    output: Option<&mut Vec<u8>>,
) -> Result<bool> {
    let spawn = |mut cmd: Command| {
        // The closure drops `cmd` which prevents a pipe deadlock.
        cmd.stdin(Stdio::null())
            .spawn()
            .with_context(|| format!("Falha ao executar `{description}`"))
    };
    let wait = |handle: &mut Child| {
        handle
            .wait_timeout(Duration::from_secs(TIMEOUT_SECS))
            .with_context(|| format!("Falha ao aguardar o término de `{description}`"))
    };
    if let Some(cwd) = cwd {
        cmd.current_dir(cwd);
    }

    let mut handle = if let Some(output) = output {
        let (mut reader, writer) =
            pipe().with_context(|| format!("Falha ao criar um pipe para executar `{description}``"))?;

        let writer_clone = writer
            .try_clone()
            .with_context(|| format!("Falha ao clonar o writer do pipe para `{description}`"))?;

        cmd.stdout(writer_clone).stderr(writer);
        let mut handle = spawn(cmd)?;

        let thread_handle = thread::Builder::new()
            .spawn(move || {
                let mut out = Vec::with_capacity(128);
                reader.read_to_end(&mut out).map(|_| out)
            })
            .context("Falha ao criar uma thread para coletar a saída do comando")?;

        if let Some(status) = wait(&mut handle)? {
            let out = thread_handle
                .join()
                .unwrap()
                .with_context(|| format!("Falha ao ler a saída de `{description}`"))?;
            output.extend_from_slice(&out);
            output.push(b'\n');
            return Ok(status.success());
        }

        handle
    } else {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
        let mut handle = spawn(cmd)?;

        if let Some(status) = wait(&mut handle)? {
            return Ok(status.success());
        }

        handle
    };

    handle
        .kill()
        .with_context(|| format!("Falha ao encerrar `{description}` após o tempo limite"))?;
    bail!("`{description}` excedeu o tempo limite de {TIMEOUT_SECS} segundos");
}

// Parses parts of the output of `cargo metadata`.
#[derive(Deserialize)]
struct CargoMetadata {
    target_directory: PathBuf,
}

pub struct CmdRunner {
    target_dir: PathBuf,
}

impl CmdRunner {
    pub fn build() -> Result<Self> {
        // Get the target directory from Cargo.
        let metadata_output = Command::new("cargo")
            .arg("metadata")
            .arg("-q")
            .arg("--format-version")
            .arg("1")
            .arg("--no-deps")
            .stdin(Stdio::null())
            .stderr(Stdio::inherit())
            .output()
            .context(CARGO_METADATA_ERR)?;

        if !metadata_output.status.success() {
            bail!("`cargo metadata …` falhou. Você está no diretório `rustlings/`?");
        }

        let metadata: CargoMetadata = serde_json::de::from_slice(&metadata_output.stdout).context(
            "Falha ao ler o campo `target_directory` da saída de `cargo metadata …`",
        )?;

        Ok(Self {
            target_dir: metadata.target_directory,
        })
    }

    pub fn cargo<'out>(
        &self,
        subcommand: &str,
        bin_name: &str,
        output: Option<&'out mut Vec<u8>>,
    ) -> CargoSubcommand<'out> {
        let mut cmd = Command::new("cargo");
        cmd.arg(subcommand).arg("-q").arg("--bin").arg(bin_name);

        // A hack to make `cargo run` work when developing Rustlings.
        #[cfg(debug_assertions)]
        cmd.arg("--manifest-path")
            .arg("dev/Cargo.toml")
            .arg("--target-dir")
            .arg(&self.target_dir);

        if output.is_some() {
            cmd.arg("--color").arg("always");
        }

        CargoSubcommand { cmd, output }
    }

    /// The boolean in the returned `Result` is true if the command's exit status is success.
    pub fn run_debug_bin(
        &self,
        bin_name: &str,
        cwd: &str,
        output: Option<&mut Vec<u8>>,
    ) -> Result<bool> {
        // 7 = "/debug/".len()
        let mut bin_path =
            PathBuf::with_capacity(self.target_dir.as_os_str().len() + 7 + bin_name.len());
        bin_path.push(&self.target_dir);
        bin_path.push("debug");
        bin_path.push(bin_name);

        run_cmd(Command::new(&bin_path), bin_name, Some(cwd), output)
    }
}

pub struct CargoSubcommand<'out> {
    cmd: Command,
    output: Option<&'out mut Vec<u8>>,
}

impl CargoSubcommand<'_> {
    pub fn args<'arg, I>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = &'arg str>,
    {
        self.cmd.args(args);
        self
    }

    /// The boolean in the returned `Result` is true if the command's exit status is success.
    pub fn run(self, description: &str) -> Result<bool> {
        run_cmd(self.cmd, description, None, self.output)
    }
}

const CARGO_METADATA_ERR: &str = "Falha ao executar `cargo metadata …`\nO Rust já está instalado?\nExecute `cargo --version` para diagnosticar o problema.";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_cmd() {
        let mut cmd = Command::new("echo");
        cmd.arg("Hello");

        let mut output = Vec::with_capacity(8);
        run_cmd(cmd, "echo …", None, Some(&mut output)).unwrap();

        assert_eq!(output, b"Hello\n\n");
    }
}
