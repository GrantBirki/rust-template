use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rust-template"))
        .args(args)
        .output()
        .expect("failed to run rust-template binary")
}

fn stdout(output: &Output) -> String {
    assert!(
        output.status.success(),
        "command failed with status {:?}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout.clone()).expect("stdout should be valid UTF-8")
}

#[test]
fn greets_default_name() {
    assert_eq!(stdout(&run(&[])), "Hello, world!\n");
}

#[test]
fn greets_named_user() {
    assert_eq!(stdout(&run(&["--name", "Grant"])), "Hello, Grant!\n");
}

#[test]
fn repeats_and_shouts_greeting() {
    assert_eq!(
        stdout(&run(&["--name", "codex", "--times", "2", "--shout"])),
        "HELLO, CODEX!\nHELLO, CODEX!\n"
    );
}

#[test]
fn adds_numbers() {
    assert_eq!(stdout(&run(&["add", "2", "3"])), "5\n");
}

#[test]
fn subtracts_numbers() {
    assert_eq!(stdout(&run(&["sub", "10", "4"])), "6\n");
}

#[test]
fn prints_extended_version_metadata() {
    let output = stdout(&run(&["version"]));

    assert!(output.contains(&format!("rust-template {}", env!("CARGO_PKG_VERSION"))));
    assert!(output.contains(&format!("build: {}", env!("CARGO_PKG_VERSION"))));
    assert!(output.contains("commit: unknown"));
    assert!(output.contains("built: unknown"));
}

#[test]
fn emits_shell_completions() {
    for shell in ["bash", "zsh", "fish", "powershell"] {
        let output = stdout(&run(&["completions", shell]));
        assert!(
            !output.is_empty(),
            "{shell} completions should not be empty"
        );
        assert!(
            output.contains("rust-template"),
            "{shell} completions should reference the binary name"
        );
    }
}

#[test]
fn emits_man_page() {
    let output = stdout(&run(&["man"]));

    assert!(output.contains(".TH rust-template"));
    assert!(output.contains("Minimal hello\\-world CLI template"));
    assert!(output.contains("rust\\-template\\-add(1)"));
}
