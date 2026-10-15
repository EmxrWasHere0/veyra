import shutil
import socket
import subprocess as sp
import sys
from pathlib import Path

REPO_URL = "https://github.com/EmxrWasHere0/veyra.git"
REPO_DIR = Path("veyra")
EXTENSION_PATH = REPO_DIR / "veyra-vscode" / "veyra-0.2.0.vsix"


def run_command(command: list[str], error_message: str) -> None:
    result = sp.run(command)

    if result.returncode != 0:
        raise RuntimeError(f"{error_message} (exit code: {result.returncode})")


def check_command(command: str) -> bool:
    return shutil.which(command) is not None


def main() -> None:
    print("Veyra Installer for Linux\n" "=========================")

    # 1. Network
    print("[ 1 / 5 ] Checking network connection...")

    try:
        with socket.create_connection(("github.com", 443), timeout=5):
            pass
    except OSError as exc:
        raise RuntimeError(
            "Could not establish a network connection to github.com."
        ) from exc

    # 2. Rust
    print("[ 2 / 5 ] Checking if Rust is available...")

    if not check_command("rustup") and not check_command("rustc"):
        raise RuntimeError("Could not find Rust. Please install Rustup or RustC.")

    if not check_command("cargo"):
        raise RuntimeError(
            "Cargo could not be found. Please install the Rust toolchain."
        )

    # 3. Git
    print("[ 3 / 5 ] Cloning Veyra...")

    if not check_command("git"):
        raise RuntimeError("Git could not be found. Please install Git.")

    if REPO_DIR.exists():
        print(f"Directory '{REPO_DIR}' already exists; skipping clone.")
    else:
        run_command(
            ["git", "clone", REPO_URL, str(REPO_DIR)],
            "Could not clone the Veyra repository.",
        )

    # 4. Install Veyra
    print("[ 4 / 5 ] Installing Veyra...")

    run_command(
        ["cargo", "install", "--path", str(REPO_DIR)],
        "Could not install Veyra. Try running it with 'cargo run'.",
    )

    # 5. IDE extension
    print("[ 5 / 5 ] Installing VS Code extension...")

    ide = (
        input("IDE command (e.g. 'code', 'codium'; 'no' to skip) [code]: ")
        .strip()
        .lower()
        or "code"
    )

    if ide == "no":
        print("IDE extension installation skipped.")
    else:
        ide_path = shutil.which(ide)

        if ide_path is None:
            print(f"Could not find '{ide}'. " "You can install the extension manually.")
        elif not EXTENSION_PATH.is_file():
            print(
                f"Extension file not found: {EXTENSION_PATH}. "
                "You can install it manually."
            )
        else:
            run_command(
                [ide_path, "--install-extension", str(EXTENSION_PATH)],
                f"Could not install the extension in {ide}.",
            )

    print("Installation done!")


if __name__ == "__main__":
    try:
        main()
    except (RuntimeError, OSError) as exc:
        print(f"\nError: {exc}", file=sys.stderr)
        sys.exit(1)
