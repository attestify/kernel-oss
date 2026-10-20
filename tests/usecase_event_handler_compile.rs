//! Compiler evidence against the actual library, with a separate Domain crate.
//!
//! Generated manifests and build output stay under this repository's `target/`.
//! No additional test dependency or copied Kernel implementation is used.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

struct FixtureWorkspace(PathBuf);

impl Drop for FixtureWorkspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn quoted_path(path: &Path) -> String {
    // TOML literal strings preserve backslashes and spaces in filesystem paths.
    let value = path.to_str().expect("fixture paths must be valid UTF-8");
    assert!(!value.contains('\''), "fixture path cannot contain a quote");
    format!("'{value}'")
}

fn cargo(workspace: &FixtureWorkspace, operation: &str, name: &str) -> Output {
    Command::new(env!("CARGO"))
        .args([operation, "--offline", "--quiet", "--bin", name])
        .arg("--manifest-path")
        .arg(workspace.0.join("Cargo.toml"))
        .arg("--target-dir")
        .arg(workspace.0.join("build"))
        .current_dir(&workspace.0)
        .env("CARGO_TERM_COLOR", "never")
        .output()
        .expect("Cargo must run for the compiler contract checks")
}

/// Positive controls run; each negative fails with its intended Rust error code.
#[test]
fn public_contract_and_foreign_marker_compiler_boundaries() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let fixtures = root.join("tests/fixtures/usecase_event_handler");
    let workspace = FixtureWorkspace(
        root.join("target")
            .join(format!("handler-contract-{}", std::process::id())),
    );
    // A stale directory from a terminated prior run is not evidence for this run.
    if workspace.0.exists() {
        fs::remove_dir_all(&workspace.0).expect("remove stale compiler fixture output");
    }
    fs::create_dir_all(workspace.0.join("domain")).expect("create compiler fixture workspace");
    fs::write(
        workspace.0.join("domain/Cargo.toml"),
        format!(
            "[package]\nname = \"handler_domain\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\
             [lib]\npath = {}\n\
             [dependencies]\nkernel_oss = {{ path = {} }}\n",
            quoted_path(&fixtures.join("domain.rs")),
            quoted_path(root),
        ),
    )
    .expect("write Domain fixture manifest");
    let cases = [
        ("foreign_binding", "E0117"),
        ("wrong_input", "E0308"),
        ("missing_event", "E0046"),
        ("missing_base", "E0277"),
        ("wrong_marker", "E0308"),
        ("non_send_constructor", "E0599"),
        ("non_sync_default", "E0599"),
        ("non_sync_handler", "E0277"),
    ];
    let mut manifest = format!(
        "[workspace]\nmembers = [\"domain\"]\n\
         [package]\nname = \"handler_consumer\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\
         [dependencies]\nkernel_oss = {{ path = {} }}\n\
         handler_domain = {{ path = \"domain\" }}\n",
        quoted_path(root),
    );
    for name in std::iter::once("positive").chain(cases.iter().map(|(name, _)| *name)) {
        manifest.push_str(&format!(
            "[[bin]]\nname = \"{name}\"\npath = {}\n",
            quoted_path(&fixtures.join(format!("{name}.rs"))),
        ));
    }
    fs::write(workspace.0.join("Cargo.toml"), manifest).expect("write consumer fixture manifest");

    let positive = cargo(&workspace, "run", "positive");
    assert!(
        positive.status.success(),
        "positive fixture must compile and run: {}",
        String::from_utf8_lossy(&positive.stderr)
    );
    for (name, error_code) in cases {
        let output = cargo(&workspace, "check", name);
        let diagnostic = String::from_utf8_lossy(&output.stderr);
        assert!(!output.status.success(), "{name} unexpectedly compiled");
        assert!(
            diagnostic.contains(&format!("error[{error_code}]")),
            "{name} must fail with {error_code}, not an unrelated failure:\n{diagnostic}"
        );
    }
}
