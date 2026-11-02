# Veyra Package Manager
# Python Version
# This package manager requires Python in order to run

# If youh have Veyra installed, `vpm` also has a Veyra version at:
# (project_root)/vpm/veyra/vpm.vey
# It is highly recommended to use the Python version of `vpm`

import argparse
import json
import os
import re
import shutil
import sys
import tempfile

from pathlib import Path


VERSION = "0.1.0"

MANIFEST = "veyra.json"
PACKAGE_DIR = "veyra_packages"


def validate_package_name(name):
    if not isinstance(name, str):
        raise RuntimeError("Package name must be text.")

    if not re.fullmatch(r"[a-zA-Z][a-zA-Z0-9_-]*", name):
        raise RuntimeError(
            f"Invalid package name '{name}'. "
            "Use letters, numbers, underscores, and hyphens."
        )

    return name


def load_manifest():
    path = Path(MANIFEST)

    if not path.is_file():
        raise RuntimeError("No veyra.json found. Run 'vpm init' first.")

    with path.open("r", encoding="utf-8") as file:
        data = json.load(file)

    if not isinstance(data, dict):
        raise RuntimeError("Project manifest must contain a JSON object.")

    if not isinstance(data.get("dependencies"), dict):
        raise RuntimeError("Project manifest must contain a dependencies object.")

    return data


def save_manifest(data):
    manifest_path = Path(MANIFEST)
    parent = manifest_path.resolve().parent

    temporary_path = None

    try:
        with tempfile.NamedTemporaryFile(
            mode="w",
            encoding="utf-8",
            dir=parent,
            prefix=".veyra-manifest-",
            suffix=".tmp",
            delete=False,
        ) as file:
            temporary_path = Path(file.name)
            json.dump(data, file, indent=2)
            file.write("\n")

        os.replace(temporary_path, manifest_path)

    finally:
        if temporary_path is not None:
            temporary_path.unlink(missing_ok=True)

    print("Updated veyra.json")


def validate_directory_source(source):
    source = Path(source)

    if source.is_symlink():
        raise RuntimeError(f"Package source cannot be a symbolic link: '{source}'")

    if not source.is_dir():
        raise RuntimeError(f"Package directory not found: '{source}'")

    return source.resolve()


def validate_package_manifest(source, expected_name=None):
    package_manifest = source / MANIFEST

    if package_manifest.is_symlink():
        raise RuntimeError("Package manifest cannot be a symbolic link.")

    if not package_manifest.is_file():
        raise RuntimeError("Package does not contain veyra.json.")

    with package_manifest.open("r", encoding="utf-8") as file:
        package = json.load(file)

    if not isinstance(package, dict):
        raise RuntimeError("Package manifest must contain a JSON object.")

    name = validate_package_name(package.get("name"))

    if expected_name is not None and name != expected_name:
        raise RuntimeError(
            f"Package name mismatch: expected '{expected_name}', found '{name}'."
        )

    return name


def validate_installation_paths(source, destination):
    source = source.resolve()

    parent = destination.parent.resolve()
    destination = parent / destination.name

    if source == destination or destination in source.parents:
        raise RuntimeError(
            "Package source cannot be inside its installation directory."
        )

    if source in destination.parents:
        raise RuntimeError(
            "Package installation directory cannot be inside the source directory."
        )

    if destination.is_symlink():
        raise RuntimeError(
            f"Package destination cannot be a symbolic link: '{destination}'"
        )

    if destination.exists() and not destination.is_dir():
        raise RuntimeError(f"Package destination is not a directory: '{destination}'")

    return destination


def validate_package_tree(source):
    for root, directories, files in os.walk(source, followlinks=False):
        for name in directories + files:
            path = Path(root) / name

            if path.is_symlink():
                raise RuntimeError(
                    f"Symbolic links are not supported in packages: '{path}'"
                )


def replace_package_tree(source, destination, update_manifest=None):
    source = validate_directory_source(source)

    destination.parent.mkdir(parents=True, exist_ok=True)
    destination = validate_installation_paths(source, destination)

    validate_package_tree(source)

    with tempfile.TemporaryDirectory(
        prefix=".veyra-stage-",
        dir=destination.parent,
    ) as staging_directory:
        staged_package = Path(staging_directory) / "package"

        shutil.copytree(source, staged_package)

        backup_directory = None
        backup_package = None

        if destination.exists():
            backup_directory = Path(
                tempfile.mkdtemp(
                    prefix=".veyra-backup-",
                    dir=destination.parent,
                )
            )

            backup_package = backup_directory / "package"
            destination.rename(backup_package)

        try:
            staged_package.rename(destination)

            if update_manifest is not None:
                update_manifest()

        except Exception as error:
            try:
                if destination.exists():
                    shutil.rmtree(destination)

                if backup_package is not None and backup_package.exists():
                    backup_package.rename(destination)

            except Exception as rollback_error:
                recovery_path = (
                    str(backup_directory)
                    if backup_directory is not None
                    else "No backup available"
                )

                raise RuntimeError(
                    f"Installation failed: {error}; "
                    f"rollback failed: {rollback_error}; "
                    f"recovery location: '{recovery_path}'"
                ) from rollback_error

            raise

        finally:
            if backup_directory is not None:
                if backup_package is None or not backup_package.exists():
                    shutil.rmtree(backup_directory, ignore_errors=True)


def init_project(name):
    validate_package_name(name)

    if Path(MANIFEST).exists():
        raise RuntimeError("A Veyra project already exists.")

    data = {
        "name": name,
        "version": "0.1.0",
        "entry": "main.vey",
        "dependencies": {},
    }

    save_manifest(data)

    if not Path("main.vey").exists():
        Path("main.vey").write_text(
            'say "Hello from Veyra!"\n',
            encoding="utf-8",
        )

    print(f"Initialized Veyra project '{name}'")


def add_package(source):
    manifest = load_manifest()

    source_path = validate_directory_source(source)
    name = validate_package_manifest(source_path)

    destination = Path(PACKAGE_DIR) / name

    manifest["dependencies"][name] = {"path": str(source_path)}

    replace_package_tree(
        source_path,
        destination,
        update_manifest=lambda: save_manifest(manifest),
    )

    print(f"Installed '{name}' from {source_path}")


def remove_package(name):
    name = validate_package_name(name)

    manifest = load_manifest()

    if name not in manifest["dependencies"]:
        raise RuntimeError(f"Package '{name}' is not installed.")

    destination = Path(PACKAGE_DIR) / name

    if destination.is_symlink():
        raise RuntimeError(
            f"Package destination cannot be a symbolic link: '{destination}'"
        )

    backup_directory = None
    backup_package = None

    if destination.exists():
        if not destination.is_dir():
            raise RuntimeError(
                f"Package destination is not a directory: '{destination}'"
            )

        backup_directory = Path(
            tempfile.mkdtemp(
                prefix=".veyra-remove-",
                dir=destination.parent,
            )
        )

        backup_package = backup_directory / "package"
        destination.rename(backup_package)

    del manifest["dependencies"][name]

    try:
        save_manifest(manifest)

    except Exception:
        if backup_package is not None and backup_package.exists():
            backup_package.rename(destination)

        raise

    finally:
        if backup_directory is not None:
            if backup_package is None or not backup_package.exists():
                shutil.rmtree(backup_directory, ignore_errors=True)

    print(f"Removed '{name}'")


def install_packages():
    manifest = load_manifest()

    dependencies = manifest["dependencies"]

    if not dependencies:
        print("No dependencies to install.")
        return

    for name, dependency in dependencies.items():
        validate_package_name(name)

        if not isinstance(dependency, dict):
            raise RuntimeError(f"Invalid dependency configuration for '{name}'.")

        source_value = dependency.get("path")

        if not isinstance(source_value, str):
            raise RuntimeError(f"Invalid source path for package '{name}'.")

        source = validate_directory_source(source_value)

        validate_package_manifest(source, expected_name=name)

        destination = Path(PACKAGE_DIR) / name

        replace_package_tree(source, destination)

        print(f"Installed '{name}'")


def list_packages():
    manifest = load_manifest()

    dependencies = manifest["dependencies"]

    if not dependencies:
        print("No packages installed.")
        return

    for name, dependency in dependencies.items():
        validate_package_name(name)

        if not isinstance(dependency, dict):
            raise RuntimeError(f"Invalid dependency configuration for '{name}'.")

        print(f"{name}: {dependency.get('path', 'unknown')}")


def main():
    parser = argparse.ArgumentParser(description="VPM - Veyra Package Manager")

    parser.add_argument(
        "--version",
        action="version",
        version=f"VPM {VERSION}",
    )

    commands = parser.add_subparsers(
        dest="command",
        required=True,
    )

    init = commands.add_parser("init")
    init.add_argument("name")

    add = commands.add_parser("add")
    add.add_argument("path")

    remove = commands.add_parser("remove")
    remove.add_argument("name")

    commands.add_parser("install")
    commands.add_parser("list")

    args = parser.parse_args()

    try:
        if args.command == "init":
            init_project(args.name)

        elif args.command == "add":
            add_package(args.path)

        elif args.command == "remove":
            remove_package(args.name)

        elif args.command == "install":
            install_packages()

        elif args.command == "list":
            list_packages()

    except (
        RuntimeError,
        OSError,
        KeyError,
        TypeError,
        ValueError,
        json.JSONDecodeError,
    ) as error:
        print(f"VPM Error: {error}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
