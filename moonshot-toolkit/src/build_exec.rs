//! Task #14 — actual seL4 cross-compile execution.
//!
//! Executes a [`BuildPlan`] against a [`SystemSpec`]'s `[build]`
//! configuration, invoking the Microkit 2.2.0 SDK toolchain:
//!
//! 1. Detect architecture from `{sdk}/board/{board}/{config}/include/kernel/gen_config.h`
//! 2. Generate Microkit XML from the spec
//! 3. For each `CompilePd` step: compile C source + link with `-lmicrokit -Tmicrokit.ld`
//! 4. For `AssembleImage`: invoke `microkit` to produce `loader.img`
//!
//! Toolchain selection:
//! - `aarch64` → `aarch64-linux-gnu-gcc` / `aarch64-linux-gnu-ld`
//! - `x86_64`  → `x86_64-linux-gnu-gcc`  / `x86_64-linux-gnu-ld`

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::plan::{BuildCommand, BuildPlan};
use crate::spec::SystemSpec;

/// Execute the build plan produced from `spec`. `spec_dir` is the
/// directory containing the `system-spec.toml` — all relative paths
/// in the spec are resolved against it.
pub fn run_build(
    spec: &SystemSpec,
    plan: &BuildPlan,
    spec_dir: &Path,
) -> Result<(), String> {
    let bc = &spec.build;

    if bc.board.is_empty() {
        return Err(
            "system-spec.toml is missing [build] section or board is empty".to_string(),
        );
    }

    let sdk = Path::new(&bc.sdk);
    let board_dir = sdk
        .join("board")
        .join(&bc.board)
        .join(&bc.config);
    let output_dir = spec_dir.join(&bc.output_dir);
    let microkit_bin = sdk.join("bin/microkit");

    // Sanity-check the SDK and board directory exist.
    if !sdk.exists() {
        return Err(format!("Microkit SDK not found: {}", sdk.display()));
    }
    if !board_dir.exists() {
        return Err(format!(
            "board directory not found: {} (check board/config in [build])",
            board_dir.display()
        ));
    }

    // 1. Detect target architecture.
    let arch = detect_arch(&board_dir)?;
    eprintln!(
        "moonshot-toolkit: arch={arch}  board={}  config={}  plan_hash={}",
        bc.board,
        bc.config,
        hex_short(&plan.plan_hash)
    );

    // 2. Create output directory.
    fs::create_dir_all(&output_dir)
        .map_err(|e| format!("create {}: {e}", output_dir.display()))?;

    // 3. Write Microkit XML system description.
    let xml_path = output_dir.join("system.xml");
    fs::write(&xml_path, spec.to_microkit_xml())
        .map_err(|e| format!("write {}: {e}", xml_path.display()))?;
    eprintln!("  wrote: {}", xml_path.display());

    // 4. Execute each plan step.
    for step in &plan.steps {
        eprintln!("  step: {}", step.name);
        match &step.command {
            BuildCommand::CompilePd {
                pd_name,
                source_path,
                binary_target,
            } => {
                let source = spec_dir.join(source_path);
                let obj = output_dir.join(format!("{pd_name}.o"));
                let elf = spec_dir.join(binary_target);
                compile_pd(&arch, &board_dir, &source, &obj, &elf)?;
                eprintln!("    compiled: {pd_name} → {}", elf.display());
            }
            BuildCommand::AssembleImage { output_image, .. } => {
                let image_path = spec_dir.join(output_image);
                let report_path = output_dir.join("report.txt");
                assemble_image(
                    &microkit_bin,
                    &xml_path,
                    &output_dir,
                    &bc.board,
                    &bc.config,
                    &image_path,
                    &report_path,
                )?;
                if let Ok(txt) = fs::read_to_string(&report_path) {
                    eprintln!("\nMicrokit report:\n{txt}");
                }
                eprintln!("  image: {}", image_path.display());
                // Print the output path on stdout for toolchain consumers.
                println!("{}", image_path.display());
            }
        }
    }

    Ok(())
}

fn detect_arch(board_dir: &Path) -> Result<String, String> {
    let gen_config = board_dir.join("include/kernel/gen_config.h");
    let content = fs::read_to_string(&gen_config)
        .map_err(|e| format!("read {}: {e}", gen_config.display()))?;
    for line in content.lines() {
        if line.trim_start().starts_with("#define CONFIG_SEL4_ARCH") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                return Ok(parts[2].to_string());
            }
        }
    }
    Err(format!(
        "CONFIG_SEL4_ARCH not found in {}",
        gen_config.display()
    ))
}

struct Toolchain {
    cc: &'static str,
    ld: &'static str,
    arch_cflags: Vec<&'static str>,
}

fn toolchain_for(arch: &str) -> Result<Toolchain, String> {
    match arch {
        "aarch64" => Ok(Toolchain {
            cc: "aarch64-linux-gnu-gcc",
            ld: "aarch64-linux-gnu-ld",
            arch_cflags: vec!["-mstrict-align"],
        }),
        "x86_64" => Ok(Toolchain {
            cc: "x86_64-linux-gnu-gcc",
            ld: "x86_64-linux-gnu-ld",
            arch_cflags: vec!["-march=x86-64", "-mtune=generic"],
        }),
        other => Err(format!("unsupported architecture: {other}")),
    }
}

fn compile_pd(
    arch: &str,
    board_dir: &Path,
    source: &Path,
    obj: &PathBuf,
    elf: &PathBuf,
) -> Result<(), String> {
    let tc = toolchain_for(arch)?;
    let board_include = board_dir.join("include");
    let board_lib = board_dir.join("lib");

    // Compile: source → object
    let mut compile = Command::new(tc.cc);
    compile
        .args(["-nostdlib", "-ffreestanding", "-g", "-O2", "-Wall",
               "-Wno-unused-function", "-Werror"])
        .args(&tc.arch_cflags)
        .arg(format!("-I{}", board_include.display()))
        .arg("-c")
        .arg(source)
        .arg("-o")
        .arg(obj);
    run_cmd(&mut compile, "compile")?;

    // Link: object → ELF with Microkit linker script
    let mut link = Command::new(tc.ld);
    link.arg(format!("-L{}", board_lib.display()))
        .arg("-lmicrokit")
        .arg("-Tmicrokit.ld")
        .arg(obj)
        .arg("-o")
        .arg(elf);
    run_cmd(&mut link, "link")?;

    Ok(())
}

fn assemble_image(
    microkit_bin: &Path,
    xml_path: &Path,
    search_dir: &Path,
    board: &str,
    config: &str,
    output: &PathBuf,
    report: &PathBuf,
) -> Result<(), String> {
    let mut cmd = Command::new(microkit_bin);
    cmd.arg(xml_path)
        .arg("--search-path")
        .arg(search_dir)
        .arg("--board")
        .arg(board)
        .arg("--config")
        .arg(config)
        .arg("-o")
        .arg(output)
        .arg("-r")
        .arg(report);
    run_cmd(&mut cmd, "microkit assemble")?;
    Ok(())
}

fn run_cmd(cmd: &mut Command, label: &str) -> Result<(), String> {
    let status = cmd
        .status()
        .map_err(|e| format!("{label}: failed to exec {:?}: {e}", cmd.get_program()))?;
    if !status.success() {
        return Err(format!(
            "{label}: {:?} exited with {}",
            cmd.get_program(),
            status
        ));
    }
    Ok(())
}

fn hex_short(hash: &[u8; 32]) -> String {
    hash[..8]
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>()
        + "…"
}
