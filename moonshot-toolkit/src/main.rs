//! moonshot-toolkit CLI — Rust-only build orchestrator for
//! Foundry's seL4 unikernel images.
//!
//! Per MEMO §7 and convention `system-substrate-doctrine.md` §6.
//! Three subcommands:
//!
//! - `validate <spec.toml>` — parse + invariant-check; exit 0 on
//!   valid, non-zero on parse/validation failure
//! - `plan <spec.toml>` — parse + generate + print BuildPlan
//! - `build <spec.toml>` — parse + plan + execute seL4 cross-compile
//!   via Microkit 2.2.0 SDK (task #14).

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

use moonshot_toolkit::plan::BuildPlan;
use moonshot_toolkit::spec::SystemSpec;

#[derive(Parser, Debug)]
#[command(
    name = "moonshot-toolkit",
    version,
    about = "Rust-only build orchestrator for Foundry seL4 unikernel images",
    long_about = "Per MEMO §7 (Microkit Python/CMake → moonshot-toolkit \
                  Rust-Only Toolchain) and convention \
                  system-substrate-doctrine.md §6 \
                  (Reproducible-Verification-On-Customer-Metal)."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Parse and validate a system-spec.toml without building.
    Validate {
        /// Path to system-spec.toml.
        spec_path: PathBuf,
    },
    /// Generate and print a BuildPlan from a system-spec.toml.
    Plan {
        /// Path to system-spec.toml.
        spec_path: PathBuf,
        /// Output format.
        #[arg(long, value_enum, default_value_t = PlanFormat::Json)]
        format: PlanFormat,
    },
    /// Parse, plan, and execute the seL4 cross-compile via Microkit 2.2.0 SDK.
    /// Requires [build] section in system-spec.toml (board, config, sdk).
    Build {
        /// Path to system-spec.toml.
        spec_path: PathBuf,
    },
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
enum PlanFormat {
    Json,
    PrettyJson,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match dispatch(cli.command) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn dispatch(command: Command) -> Result<(), String> {
    match command {
        Command::Validate { spec_path } => cmd_validate(&spec_path),
        Command::Plan { spec_path, format } => cmd_plan(&spec_path, format),
        Command::Build { spec_path } => cmd_build(&spec_path),
    }
}

fn read_spec(spec_path: &std::path::Path) -> Result<SystemSpec, String> {
    let text = std::fs::read_to_string(spec_path)
        .map_err(|e| format!("read {}: {e}", spec_path.display()))?;
    SystemSpec::from_toml_str(&text).map_err(|e| format!("parse {}: {e:?}", spec_path.display()))
}

fn cmd_validate(spec_path: &std::path::Path) -> Result<(), String> {
    let spec = read_spec(spec_path)?;
    println!(
        "✓ {} — {} protection_domain(s), {} channel(s), {} memory_region(s), {} irq_delivery",
        spec_path.display(),
        spec.protection_domains.len(),
        spec.channels.len(),
        spec.memory_regions.len(),
        spec.irq_delivery.len(),
    );
    Ok(())
}

fn cmd_plan(spec_path: &std::path::Path, format: PlanFormat) -> Result<(), String> {
    let spec = read_spec(spec_path)?;
    let plan = BuildPlan::from_spec(&spec).map_err(|e| format!("plan: {e:?}"))?;
    let rendered = match format {
        PlanFormat::Json => serde_json::to_string(&plan)
            .map_err(|e| format!("render plan: {e}"))?,
        PlanFormat::PrettyJson => serde_json::to_string_pretty(&plan)
            .map_err(|e| format!("render plan: {e}"))?,
    };
    println!("{rendered}");
    Ok(())
}

fn cmd_build(spec_path: &std::path::Path) -> Result<(), String> {
    let spec = read_spec(spec_path)?;
    let plan = BuildPlan::from_spec(&spec).map_err(|e| format!("plan: {e:?}"))?;
    let spec_dir = spec_path
        .parent()
        .ok_or_else(|| format!("{}: no parent directory", spec_path.display()))?;
    moonshot_toolkit::build_exec::run_build(&spec, &plan, spec_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn write_spec(text: &str) -> NamedTempFile {
        let mut f = NamedTempFile::new().unwrap();
        f.write_all(text.as_bytes()).unwrap();
        f.flush().unwrap();
        f
    }

    fn minimal_spec() -> &'static str {
        r#"
[[protection_domains]]
name = "hello"
binary = "src/hello.rs"
priority = 100
stack_bytes = 4096
"#
    }

    #[test]
    fn validate_command_accepts_minimal_spec() {
        let f = write_spec(minimal_spec());
        let r = cmd_validate(f.path());
        assert!(r.is_ok(), "validate should accept; got {r:?}");
    }

    #[test]
    fn validate_command_rejects_invalid_spec() {
        let f = write_spec("this is not [valid toml");
        let r = cmd_validate(f.path());
        assert!(r.is_err());
    }

    #[test]
    fn validate_command_rejects_missing_file() {
        let r = cmd_validate(std::path::Path::new("/tmp/does-not-exist-9f8a3c.toml"));
        assert!(r.is_err());
    }

    #[test]
    fn plan_command_emits_json() {
        let f = write_spec(minimal_spec());
        let r = cmd_plan(f.path(), PlanFormat::Json);
        assert!(r.is_ok(), "plan should succeed; got {r:?}");
    }

    #[test]
    fn plan_command_emits_pretty_json() {
        let f = write_spec(minimal_spec());
        let r = cmd_plan(f.path(), PlanFormat::PrettyJson);
        assert!(r.is_ok());
    }

    #[test]
    fn build_requires_board_in_spec() {
        // Spec with a PD but no [build] section → board is empty → Err.
        let f = write_spec(minimal_spec());
        let r = cmd_build(f.path());
        assert!(r.is_err(), "build without [build].board should fail; got {r:?}");
        let msg = r.unwrap_err();
        assert!(msg.contains("board"), "error should mention board; got: {msg}");
    }

    #[test]
    fn empty_spec_build_errors_at_plan_step() {
        // No protection_domains → plan generation refuses.
        let f = write_spec("");
        let r = cmd_build(f.path());
        assert!(r.is_err(), "empty spec should fail plan; got {r:?}");
    }
}
