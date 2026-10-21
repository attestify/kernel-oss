//! Verifies the real Logger contract across compiler and subprocess boundaries.
//! Contract: kernel-oss-operational-logger@0.1.0
//! Role: test; Targets: LOG-T02–LOG-T04, LOG-T07–LOG-T09; Layout: F07.
//! Test size: MD. A positive binary builds first and runs silently; deliberately
//! invalid implementations must fail for their named obligations. No sink
//! conformance, allocation measurement or panic immunity is claimed.

use std::fs;
use std::panic::{catch_unwind, resume_unwind};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// Encodes a path as a TOML basic string, including quotes and control characters.
fn quoted_path(path: &Path) -> String {
    let mut quoted = String::from("\"");
    for character in path
        .to_str()
        .expect("fixture paths must be valid UTF-8")
        .chars()
    {
        match character {
            '\\' => quoted.push_str("\\\\"),
            '"' => quoted.push_str("\\\""),
            c if c.is_control() => quoted.push_str(&format!("\\u{:04X}", u32::from(c))),
            c => quoted.push(c),
        }
    }
    quoted.push('"');
    quoted
}

/// Runs the current Cargo offline against the isolated workspace and actual library.
fn cargo(workspace: &Path, operation: &str, name: &str) -> Output {
    Command::new(env!("CARGO"))
        .args([operation, "--offline", "--quiet", "--bin", name])
        .arg("--manifest-path")
        .arg(workspace.join("Cargo.toml"))
        .arg("--target-dir")
        .arg(workspace.join("build"))
        .current_dir(workspace)
        .env("CARGO_TERM_COLOR", "never")
        .output()
        .expect("Cargo must run to verify the Logger compiler boundary")
}

/// Creates deterministic fixture metadata; nested resolution never edits the root lock.
fn prepare_workspace(root: &Path, workspace: &Path, names: &[&str]) {
    fs::create_dir_all(workspace).expect("create isolated Logger compiler workspace");
    let mut manifest = format!(
        "[workspace]\n[package]\nname = \"logging_consumer\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\
         [dependencies]\nkernel_oss = {{ path = {} }}\n",
        quoted_path(root),
    );
    for name in names {
        manifest.push_str(&format!(
            "[[bin]]\nname = \"{name}\"\npath = {}\n",
            quoted_path(
                &root
                    .join("tests/fixtures/logging")
                    .join(format!("{name}.rs"))
            )
        ));
    }
    fs::write(workspace.join("Cargo.toml"), manifest)
        .expect("write Logger fixture manifest pointing to actual library and fixtures");
}

/// Builds/runs a silent actual no-op, then rejects only the intended trait violations.
/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test; Targets: LOG-T02–LOG-T04, LOG-T07–LOG-T09; Layout: F07.
/// Test size: MD. One bounded compiler contract suite shares a positive control;
/// each negative has its own status, code and obligation assertions. Stream
/// silence is observed separately from Cargo diagnostics. Other I/O is review-only.
#[test]
fn public_logger_compiler_and_silent_no_op_success() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace: PathBuf = root
        .join("target")
        .join(format!("logging-contract-{}", std::process::id()));
    if workspace.exists() {
        fs::remove_dir_all(&workspace).expect("remove stale Logger compiler output");
    }
    // Catch harness assertions only so fixture output is cleaned before failure
    // propagates. Logger code executes in a subprocess; its panic is never caught.
    let outcome = catch_unwind(|| {
        let cases = [
            ("missing_method", "E0046", "`warn`"),
            ("non_send", "E0277", "cannot be sent between threads safely"),
            (
                "non_sync",
                "E0277",
                "cannot be shared between threads safely",
            ),
            (
                "extra_initialize",
                "E0407",
                "method `initialize` is not a member of trait `Logger`",
            ),
            (
                "result_return",
                "E0053",
                "method `info` has an incompatible type for trait",
            ),
        ];
        let names: Vec<_> = std::iter::once("positive")
            .chain(cases.iter().map(|(name, _, _)| *name))
            .collect();
        prepare_workspace(root, &workspace, &names);
        let positive = cargo(&workspace, "build", "positive");
        assert!(
            positive.status.success(),
            "positive fixture must compile: {}",
            String::from_utf8_lossy(&positive.stderr)
        );
        let run = Command::new(
            workspace
                .join("build/debug")
                .join(format!("positive{}", std::env::consts::EXE_SUFFIX)),
        )
        .output()
        .expect("execute the actual no-op positive fixture binary");
        assert!(
            run.status.success(),
            "positive no-op fixture must return normally: {}",
            String::from_utf8_lossy(&run.stderr)
        );
        assert!(run.stdout.is_empty(), "no-op must emit no stdout");
        assert!(run.stderr.is_empty(), "no-op must emit no stderr");
        for (name, error_code, obligation) in cases {
            let output = cargo(&workspace, "check", name);
            let diagnostic = String::from_utf8_lossy(&output.stderr);
            assert!(!output.status.success(), "{name} unexpectedly compiled");
            let marker = format!("error[{error_code}]");
            assert!(
                diagnostic.contains(&marker),
                "{name} must fail with {error_code}:\n{diagnostic}"
            );
            assert!(
                diagnostic.contains(obligation),
                "{name} must identify {obligation}:\n{diagnostic}"
            );
            assert!(
                diagnostic
                    .lines()
                    .filter(|line| line.starts_with("error["))
                    .all(|line| line.starts_with(&marker)),
                "{name} must not hide an unrelated compiler error:\n{diagnostic}"
            );
        }
    });
    let cleanup = if workspace.exists() {
        fs::remove_dir_all(&workspace)
    } else {
        Ok(())
    };
    if let Err(panic) = outcome {
        if let Err(error) = cleanup {
            eprintln!("Logger fixture cleanup also failed: {error}");
        }
        resume_unwind(panic);
    }
    cleanup.expect("remove Logger fixture manifests, lockfile and build output");
}
