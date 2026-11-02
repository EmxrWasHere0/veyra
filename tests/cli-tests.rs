use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

fn veyra_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_veyra"))
}

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn run_repl_in_directory(working_directory: &Path, input: &str) -> Output {
    let mut child = Command::new(veyra_binary())
        .current_dir(working_directory)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start Veyra REPL");

    let mut stdin = child.stdin.take().expect("Missing REPL stdin");
    stdin
        .write_all(input.as_bytes())
        .expect("Failed to send REPL input");
    drop(stdin);

    child
        .wait_with_output()
        .expect("Failed to wait for Veyra REPL")
}

fn run_script_in_directory(script: &Path, working_directory: &Path) -> Output {
    Command::new(veyra_binary())
        .arg(script)
        .current_dir(working_directory)
        .output()
        .expect("Failed to execute Veyra")
}

fn run_source_in_directory(
    project_directory: &Path,
    working_directory: &Path,
    source: &str,
) -> Output {
    let script = project_directory.join("main.vey");

    fs::write(&script, source).expect("Failed to write test script");

    run_script_in_directory(&script, working_directory)
}

fn create_vpm_package(
    project_directory: &Path,
    directory_name: &str,
    package_name: &str,
    module_name: &str,
    entry: &str,
    module_source: &str,
) -> PathBuf {
    let package_directory = project_directory
        .join("veyra_packages")
        .join(directory_name);

    fs::create_dir_all(&package_directory).expect("Failed to create test package directory");

    let entry_path = package_directory.join(entry);

    if let Some(parent) = entry_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create package entry directory");
    }

    fs::write(&entry_path, module_source).expect("Failed to write package entry");

    let manifest = format!(
        r#"{{
  "name": "{}",
  "module": "{}",
  "version": "0.1.0",
  "entry": "{}",
  "dependencies": {{}}
}}"#,
        package_name, module_name, entry
    );

    fs::write(package_directory.join("veyra.json"), manifest)
        .expect("Failed to write package manifest");

    package_directory
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
        stderr.contains("error_location.vey:4:7: Division by zero"),
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

#[test]
fn error_column_undefined_variable() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_veyra"))
        .arg("tests/fixtures/error_columns.vey")
        .output()
        .expect("Failed to execute Veyra");

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        combined.contains("error_columns.vey:1:10:"),
        "Incorrect undefined variable location:\n{}",
        combined
    );

    assert!(
        combined.contains("|          ^^^^^^^"),
        "Incorrect source marker:\n{}",
        combined
    );
}

#[test]
fn error_column_multiline() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_veyra"))
        .arg("tests/fixtures/error_columns_multiline.vey")
        .output()
        .expect("Failed to execute Veyra");

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        combined.contains("error_columns_multiline.vey:4:5:"),
        "Incorrect multiline error location:\n{}",
        combined
    );
}

#[test]
fn error_column_lambda() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_veyra"))
        .arg("tests/fixtures/lambda_error_location.vey")
        .output()
        .expect("Failed to execute Veyra");

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        combined.contains("lambda_error_location.vey:1:28:"),
        "Incorrect lambda error location:\n{}",
        combined
    );

    assert!(
        combined.contains("Division by zero"),
        "Missing division error:\n{}",
        combined
    );

    assert!(
        combined.contains("<lambda>()"),
        "Missing lambda traceback:\n{}",
        combined
    );
}

#[test]
fn error_column_nested_functions() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_veyra"))
        .arg("tests/fixtures/nested_error_location.vey")
        .output()
        .expect("Failed to execute Veyra");

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        combined.contains("nested_error_location.vey:2:15:"),
        "Incorrect nested function error location:\n{}",
        combined
    );

    assert!(
        combined.contains("divide()"),
        "Missing divide traceback:\n{}",
        combined
    );

    assert!(
        combined.contains("calculate()"),
        "Missing calculate traceback:\n{}",
        combined
    );
}

#[test]
fn short_circuit_parser_hardening() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_veyra"))
        .arg("--test")
        .arg("tests/fixtures/short_circuit_hardening.vey")
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
        stdout.contains("11 passed, 0 failed"),
        "Unexpected test results:\n{}",
        stdout
    );
}

#[test]
fn short_circuit_invalid_syntax() {
    let cases = [
        ("short_circuit_invalid_operator.vey", "Unexpected token"),
        ("short_circuit_invalid_arguments.vey", "Unexpected token"),
        ("short_circuit_invalid_group.vey", "Unexpected token"),
        ("short_circuit_invalid_lambda.vey", "Expected Id"),
        ("short_circuit_invalid_jsondata.vey", "Unexpected token"),
        ("short_circuit_invalid_slice.vey", "Expected RBracket"),
    ];

    for (filename, expected_error) in cases {
        let path = format!("tests/fixtures/{}", filename);

        let output = std::process::Command::new(env!("CARGO_BIN_EXE_veyra"))
            .arg(&path)
            .output()
            .expect("Failed to execute Veyra");

        let combined = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );

        assert!(
            combined.contains(expected_error),
            "Expected '{}' for '{}', got:\n{}",
            expected_error,
            filename,
            combined
        );
    }
}

#[test]
fn vpm_import_uses_manifest_module_and_entry() {
    let project = tempfile::tempdir().expect("Failed to create temporary project");

    create_vpm_package(
        project.path(),
        "example-package",
        "example-package",
        "example",
        "src/entry.veymod",
        r#"
set PACKAGE_VERSION = "0.1.0"

add greet(name)
    return f"Hello, {name}!"
end
"#,
    );

    let output = run_source_in_directory(
        project.path(),
        project.path(),
        r#"
mod example
say example.PACKAGE_VERSION
say example.greet("World")
"#,
    );

    assert!(
        output.status.success(),
        "Package import failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "0.1.0\nHello, World!"
    );
}

#[test]
fn vpm_nested_package_import_works_from_another_working_directory() {
    let project = tempfile::tempdir().expect("Failed to create temporary project");
    let other_directory = tempfile::tempdir().expect("Failed to create secondary directory");

    create_vpm_package(
        project.path(),
        "outer-package",
        "outer-package",
        "outer",
        "outer.veymod",
        r#"
mod helper
set RESULT = helper.VALUE
"#,
    );

    create_vpm_package(
        project.path(),
        "helper-package",
        "helper-package",
        "helper",
        "nested/helper.veymod",
        r#"set VALUE = "nested package found""#,
    );

    let output = run_source_in_directory(
        project.path(),
        other_directory.path(),
        "mod outer\nsay outer.RESULT\n",
    );

    assert!(
        output.status.success(),
        "Nested package import failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "nested package found"
    );
}

#[test]
fn regular_module_takes_precedence_over_vpm_package() {
    let project = tempfile::tempdir().expect("Failed to create temporary project");

    create_vpm_package(
        project.path(),
        "example-package",
        "example-package",
        "example",
        "entry.veymod",
        r#"set ORIGIN = "package""#,
    );

    fs::write(
        project.path().join("example.veymod"),
        r#"set ORIGIN = "regular module""#,
    )
    .expect("Failed to write regular module");

    let output = run_source_in_directory(
        project.path(),
        project.path(),
        "mod example\nsay example.ORIGIN\n",
    );

    assert!(
        output.status.success(),
        "Regular module import failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "regular module"
    );
}

#[test]
fn duplicate_vpm_module_names_are_reported() {
    let project = tempfile::tempdir().expect("Failed to create temporary project");

    create_vpm_package(
        project.path(),
        "first-package",
        "first-package",
        "shared",
        "entry.veymod",
        r#"set VALUE = "first""#,
    );

    create_vpm_package(
        project.path(),
        "second-package",
        "second-package",
        "shared",
        "entry.veymod",
        r#"set VALUE = "second""#,
    );

    let output = run_source_in_directory(project.path(), project.path(), "mod shared\n");

    assert!(!output.status.success());

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        combined.contains("Multiple VPM packages provide module 'shared'"),
        "Expected an ambiguity error, got:\n{}",
        combined
    );
}

#[test]
fn vpm_package_entry_rejects_parent_directory_components() {
    let project = tempfile::tempdir().expect("Failed to create temporary project");

    create_vpm_package(
        project.path(),
        "example-package",
        "example-package",
        "example",
        "../outside.veymod",
        r#"set VALUE = "outside""#,
    );

    let output = run_source_in_directory(project.path(), project.path(), "mod example\n");

    assert!(!output.status.success());

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        combined.contains("Invalid package entry path"),
        "Expected an invalid entry path error, got:\n{}",
        combined
    );
}

#[cfg(unix)]
#[test]
fn vpm_package_entry_symlink_cannot_escape_package_directory() {
    use std::os::unix::fs::symlink;

    let project = tempfile::tempdir().expect("Failed to create temporary project");

    let package = create_vpm_package(
        project.path(),
        "example-package",
        "example-package",
        "example",
        "entry.veymod",
        r#"set VALUE = "placeholder""#,
    );

    let outside_module = project.path().join("outside.veymod");
    fs::write(&outside_module, r#"set VALUE = "outside""#).expect("Failed to write outside module");

    let entry = package.join("entry.veymod");
    fs::remove_file(&entry).expect("Failed to remove placeholder entry");
    symlink(&outside_module, &entry).expect("Failed to create test symlink");

    let output = run_source_in_directory(project.path(), project.path(), "mod example\n");

    assert!(!output.status.success());

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        combined.contains("escapes its installation directory"),
        "Expected a package boundary error, got:\n{}",
        combined
    );
}

#[test]
fn malformed_module_import_restores_repl_state() {
    let project = tempfile::tempdir().expect("Failed to create temporary project");

    fs::write(
        project.path().join("broken_parse.veymod"),
        "add unfinished()\n    return 1\n",
    )
    .expect("Failed to write malformed module");

    fs::write(
        project.path().join("healthy.veymod"),
        r#"set VALUE = "healthy-import-ok""#,
    )
    .expect("Failed to write healthy module");

    let output = run_repl_in_directory(
        project.path(),
        concat!(
            "add caller_state = \"caller-survived\"\n",
            "mod broken_parse\n",
            "say caller_state\n",
            "mod broken_parse\n",
            "say caller_state\n",
            "mod healthy\n",
            "say healthy.VALUE\n",
            "exit\n",
        ),
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    assert!(
        combined.matches("broken_parse.veymod").count() >= 2,
        "The malformed module should fail on both import attempts:\n{}",
        combined
    );

    assert!(
        !combined.contains("Circular module import detected: 'broken_parse'"),
        "A failed import left the module marked as loading:\n{}",
        combined
    );

    assert!(
        stdout.matches("caller-survived").count() >= 2,
        "The caller's REPL state was not preserved:\n{}",
        combined
    );

    assert!(
        stdout.contains("healthy-import-ok"),
        "The REPL could not import a valid module after the failure:\n{}",
        combined
    );
}

#[test]
fn runtime_module_error_restores_repl_state() {
    let project = tempfile::tempdir().expect("Failed to create temporary project");

    fs::write(project.path().join("broken_runtime.veymod"), "say 1 / 0\n")
        .expect("Failed to write runtime-failing module");

    fs::write(
        project.path().join("healthy.veymod"),
        r#"set VALUE = "healthy-import-ok""#,
    )
    .expect("Failed to write healthy module");

    let output = run_repl_in_directory(
        project.path(),
        concat!(
            "add caller_state = \"caller-survived\"\n",
            "mod broken_runtime\n",
            "say caller_state\n",
            "mod broken_runtime\n",
            "say caller_state\n",
            "mod healthy\n",
            "say healthy.VALUE\n",
            "exit\n",
        ),
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    assert!(
        combined.matches("Division by zero").count() >= 2,
        "The failing module should execute and fail on both import attempts:\n{}",
        combined
    );

    assert!(
        !combined.contains("Circular module import detected: 'broken_runtime'"),
        "A failed import left the module marked as loading:\n{}",
        combined
    );

    assert!(
        stdout.matches("caller-survived").count() >= 2,
        "The caller's REPL state was not preserved:\n{}",
        combined
    );

    assert!(
        stdout.contains("healthy-import-ok"),
        "The REPL could not import a valid module after the failure:\n{}",
        combined
    );
}

#[test]
fn circular_module_import_is_reported() {
    let project = tempfile::tempdir().expect("Failed to create temporary project");

    fs::write(project.path().join("a.veymod"), "mod b\nset A = \"a\"\n")
        .expect("Failed to write module a");

    fs::write(project.path().join("b.veymod"), "mod a\nset B = \"b\"\n")
        .expect("Failed to write module b");

    let output = run_source_in_directory(project.path(), project.path(), "mod a\n");

    assert!(!output.status.success(), "Circular import should fail");

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        combined.contains("Circular module import detected: 'a'"),
        "Expected a circular import error, got:\n{}",
        combined
    );
}

#[cfg(unix)]
#[test]
fn vpm_package_root_symlink_is_rejected() {
    use std::os::unix::fs::symlink;

    let project = tempfile::tempdir().expect("Failed to create temporary project");
    let external = tempfile::tempdir().expect("Failed to create external directory");

    create_vpm_package(
        external.path(),
        "example-package",
        "example-package",
        "example",
        "entry.veymod",
        r#"set VALUE = "external""#,
    );

    let external_package_root = external.path().join("veyra_packages");
    let project_package_root = project.path().join("veyra_packages");

    symlink(&external_package_root, &project_package_root)
        .expect("Failed to create package-root symlink");

    let output = run_source_in_directory(project.path(), project.path(), "mod example\n");

    assert!(!output.status.success());

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        combined.contains("cannot be a symbolic link"),
        "Expected the package-root symlink to be rejected:\n{}",
        combined
    );
}

#[test]
fn vpm_package_star_import_exports_public_members_only() {
    let project = tempfile::tempdir().expect("Failed to create temporary project");

    create_vpm_package(
        project.path(),
        "example-package",
        "example-package",
        "example",
        "src/entry.veymod",
        r#"
set VERSION = "1.2.3"

add greet(name)
    return f"Hello, {name}!"
end

add _secret()
    return "hidden"
end
"#,
    );

    let output = run_source_in_directory(
        project.path(),
        project.path(),
        r#"
mod example *

say VERSION
say greet("World")

try
    say _secret()
catch error
    say "private"
end
"#,
    );

    assert!(
        output.status.success(),
        "Star import failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "1.2.3\nHello, World!\nprivate"
    );
}

#[test]
fn vpm_package_selective_import_works() {
    let project = tempfile::tempdir().expect("Failed to create temporary project");

    create_vpm_package(
        project.path(),
        "example-package",
        "example-package",
        "example",
        "entry.veymod",
        r#"
set VERSION = "2.0.0"

add greet(name)
    return f"Welcome, {name}!"
end

add unused()
    return "unused"
end
"#,
    );

    let output = run_source_in_directory(
        project.path(),
        project.path(),
        r#"
mod example [greet, VERSION]

say greet("Veyra")
say VERSION
"#,
    );

    assert!(
        output.status.success(),
        "Selective package import failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "Welcome, Veyra!\n2.0.0"
    );
}

#[test]
fn vpm_package_recursion_and_callbacks_work() {
    let project = tempfile::tempdir().expect("Failed to create temporary project");

    create_vpm_package(
        project.path(),
        "transform-package",
        "transform-package",
        "transform",
        "transform.veymod",
        r#"
add factorial(n)
    if n <= 1
        return 1
    end

    return n * factorial(n - 1)
end

add apply(values, callback)
    return values.map(callback)
end
"#,
    );

    let output = run_source_in_directory(
        project.path(),
        project.path(),
        r#"
mod transform

add double(value)
    return value * 2
end

say transform.factorial(5)
say transform.apply([1, 2, 3], double)
"#,
    );

    assert!(
        output.status.success(),
        "Package recursion or callback failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "120\n[2, 4, 6]"
    );
}

#[test]
fn veyra_path_still_finds_regular_modules() {
    let project = tempfile::tempdir().expect("Failed to create temporary project");
    let custom_modules = tempfile::tempdir().expect("Failed to create custom module directory");

    fs::write(
        custom_modules.path().join("external_module.veymod"),
        r#"set VALUE = "found through VEYRA_PATH""#,
    )
    .expect("Failed to write external module");

    let script = project.path().join("main.vey");
    fs::write(&script, "mod external_module\nsay external_module.VALUE\n")
        .expect("Failed to write test script");

    let output = Command::new(veyra_binary())
        .arg(&script)
        .current_dir(project.path())
        .env("VEYRA_PATH", custom_modules.path().as_os_str())
        .output()
        .expect("Failed to execute Veyra");

    assert!(
        output.status.success(),
        "VEYRA_PATH import failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "found through VEYRA_PATH"
    );
}

#[test]
fn malformed_vpm_manifest_reports_an_error() {
    let project = tempfile::tempdir().expect("Failed to create temporary project");
    let package = project.path().join("veyra_packages").join("broken-package");

    fs::create_dir_all(&package).expect("Failed to create package directory");
    fs::write(package.join("veyra.json"), "{ invalid json")
        .expect("Failed to write malformed manifest");

    let output = run_source_in_directory(project.path(), project.path(), "mod example\n");

    assert!(!output.status.success());

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        combined.contains("Invalid package manifest"),
        "Expected a manifest error, got:\n{}",
        combined
    );
}

#[test]
fn missing_vpm_entry_file_reports_an_error() {
    let project = tempfile::tempdir().expect("Failed to create temporary project");

    let package = create_vpm_package(
        project.path(),
        "example-package",
        "example-package",
        "example",
        "entry.veymod",
        r#"set VALUE = "present""#,
    );

    fs::remove_file(package.join("entry.veymod")).expect("Failed to remove package entry");

    let output = run_source_in_directory(project.path(), project.path(), "mod example\n");

    assert!(!output.status.success());

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        combined.contains("Package module file") && combined.contains("not found"),
        "Expected a missing package entry error, got:\n{}",
        combined
    );
}
