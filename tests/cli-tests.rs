use std::path::PathBuf;
use std::process::{Command, Output};

fn veyra_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_veyra"))
}

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn run_script(name: &str) -> Output {
    Command::new(veyra_binary())
        .arg(fixture(name))
        .output()
        .expect("Failed to execute Veyra")
}

fn assert_output(name: &str, expected: &str) {
    let output = run_script(name);

    assert!(
        output.status.success(),
        "Script '{}' failed:\n{}",
        name,
        String::from_utf8_lossy(&output.stderr)
    );

    let actual = String::from_utf8_lossy(&output.stdout);

    assert_eq!(
        actual.trim().replace("\r\n", "\n"),
        expected.trim(),
        "Unexpected output from '{}'",
        name
    );
}

#[test]
fn variables() {
    assert_output("variables.vey", "10\n20\n");
}

#[test]
fn functions() {
    assert_output("functions.vey", "42\n120\n");
}

#[test]
fn loops() {
    assert_output("loops.vey", "0\n1\n2\n10\n20\n30\n");
}

#[test]
fn lists() {
    assert_output("lists.vey", "99\n30\n3\n");
}

#[test]
fn jsondata() {
    assert_output("jsondata.vey", "Veyra\ntrue\njsondata\n");
}

#[test]
fn strings() {
    assert_output("strings.vey", "HELLO\nhello\ntrue\n");
}

#[test]
fn imports() {
    assert_output("imports.vey", "12\n");
}

#[test]
fn math() {
    assert_output("math.vey", "12\n256\n");
}

#[test]
fn while_dispatch() {
    assert_output("while_dispatch.vey", "0\n1\n2\n3\n4\n");
}

#[test]
fn multiline_strings() {
    assert_output("multiline_strings.vey", "Hello\nWorld\n");
}

#[test]
fn uncaught_throw() {
    let output = run_script("uncaught_throw.vey");

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stderr.contains("Expected error"),
        "Unexpected error: {}",
        stderr
    );
}

use std::io::Write;
use std::process::Stdio;

#[test]
fn list_slicing() {
    assert_output("list_slicing.vey", "20\n50\n30\n");
}

#[test]
fn indexed_assignment() {
    assert_output("indexed_assignment.vey", "99\n50\n");
}

#[test]
fn membership() {
    assert_output("membership.vey", "true\ntrue\ntrue\n");
}

#[test]
fn throw_catch() {
    assert_output("throw_catch.vey", "Caught\nFinished\n");
}

#[test]
fn collection_repeat() {
    assert_output("collection_repeat.vey", "10\n20\n30\n");
}

#[test]
fn list_methods() {
    assert_output("list_methods.vey", "3\n2\n0\n");
}

#[test]
fn get_input() {
    let mut child = Command::new(veyra_binary())
        .arg(fixture("get_input.vey"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start Veyra");

    {
        let stdin = child.stdin.as_mut().expect("Missing stdin");

        stdin.write_all(b"Veyra\n").expect("Failed to send input");
    }

    let output = child.wait_with_output().expect("Failed to wait for Veyra");

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "Name: Hello, Veyra"
    );
}

#[test]
fn error_location_with_source() {
    let output = run_script("error_location.vey");

    assert!(
        !output.status.success(),
        "Script should fail with a runtime error"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stderr.contains("error_location.vey:4: Division by zero"),
        "Error location or message is incorrect:\n{}",
        stderr
    );

    assert!(
        stderr.contains("4 | say x / y"),
        "Source line is missing from error output:\n{}",
        stderr
    );

    assert!(stderr.contains('^'), "Error marker is missing:\n{}", stderr);
}

#[test]
fn function_traceback() {
    let output = run_script("traceback.vey");

    assert!(!output.status.success(), "Script should fail");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stderr.contains("Division by zero"),
        "Missing original error:\n{}",
        stderr
    );

    assert!(
        stderr.contains("in divide()"),
        "Missing divide() traceback frame:\n{}",
        stderr
    );

    assert!(
        stderr.contains("in calculate()"),
        "Missing calculate() traceback frame:\n{}",
        stderr
    );
}

#[test]
fn string_interpolation() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_veyra"))
        .arg("--test")
        .arg("tests/fixtures/string_interpolation.vey")
        .output()
        .expect("Failed to execute Veyra");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Veyra exited with an error:\n{}\n{}",
        stdout,
        stderr
    );

    assert!(
        stdout.contains("13 passed, 0 failed"),
        "Unexpected test results:\n{}",
        stdout
    );
}

#[test]
fn functional_lists() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_veyra"))
        .arg("--test")
        .arg("tests/fixtures/functional_lists.vey")
        .output()
        .expect("Failed to execute Veyra");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Veyra exited with an error:\n{}\n{}",
        stdout,
        stderr
    );

    assert!(
        stdout.contains("15 passed, 0 failed"),
        "Unexpected test results:\n{}",
        stdout
    );
}

#[test]
fn jsondata_enhancements() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_veyra"))
        .arg("--test")
        .arg("tests/fixtures/jsondata_enhancements.vey")
        .output()
        .expect("Failed to execute Veyra");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Veyra exited with an error:\n{}\n{}",
        stdout,
        stderr
    );

    assert!(
        stdout.contains("15 passed, 0 failed"),
        "Unexpected test results:\n{}",
        stdout
    );
}

#[test]
fn compound_assignment() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_veyra"))
        .arg("--test")
        .arg("tests/fixtures/compound_assignment.vey")
        .output()
        .expect("Failed to execute Veyra");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Veyra exited with an error:\n{}\n{}",
        stdout,
        stderr
    );

    assert!(
        stdout.contains("13 passed, 0 failed"),
        "Unexpected test results:\n{}",
        stdout
    );
}

#[test]
fn short_circuit() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_veyra"))
        .arg("--test")
        .arg("tests/fixtures/short_circuit.vey")
        .output()
        .expect("Failed to execute Veyra");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Veyra exited with an error:\n{}\n{}",
        stdout,
        stderr
    );

    assert!(
        stdout.contains("22 passed, 0 failed"),
        "Unexpected test results:\n{}",
        stdout
    );
}

#[test]
fn interpolation_edge_cases() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_veyra"))
        .arg("--test")
        .arg("tests/fixtures/interpolation_edge_cases.vey")
        .output()
        .expect("Failed to execute Veyra");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Veyra exited with an error:\n{}\n{}",
        stdout,
        stderr
    );

    assert!(
        stdout.contains("22 passed, 0 failed"),
        "Unexpected test results:\n{}",
        stdout
    );
}