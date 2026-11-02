<div align="center">
    <img src="vpm-banner.png" alt="VPM">
</div>

# Veyra Package Manager (VPM)

VPM is the package manager for the Veyra programming language.

It provides a simple way to create Veyra projects, install local packages, manage dependencies, and use reusable `.veymod` modules.

VPM is currently in its early development stage.

**Current version:** 0.1.0

## Features

VPM 0.1.0 supports:

- Project initialization
- Local package installation
- Package updates
- Package removal
- Dependency installation
- Dependency listing
- JSON-based project and package manifests
- Integration with Veyra's module system
- Package name validation
- Basic filesystem safety checks
- Staged installation and rollback

GitHub packages, package registries, lockfiles, and transitive dependency resolution are not supported yet.

## Requirements

To use the Python implementation of VPM, you need:

- Python 3.10 or newer
- Veyra with VPM package import support

The Python implementation uses only the Python standard library.

Veyra installation instructions are available in the [main repository](../README.md).

## Implementations

VPM currently has two implementations.

### Python Implementation

Located at:

```text
vpm/python/vpm.py
```

This is the primary implementation and the current development focus.

Run it using:

```bash
python vpm/python/vpm.py --version
```

### Veyra Implementation (Experimental)

Located at:

```text
vpm/veyra/vpm.vey
```

An experimental implementation of VPM is also written in Veyra itself.

It supports the initial package management commands, but its development is currently paused.

The Python implementation is recommended.

## Quick Start

### 1. Initialize a Project

Create a new directory and enter it:

```bash
mkdir my-project
cd my-project
```

Initialize the project:

```bash
python /path/to/vpm/python/vpm.py init my-project
```

VPM creates:

```text
my-project/
├── main.vey
└── veyra.json
```

The project manifest contains:

```json
{
  "name": "my-project",
  "version": "0.1.0",
  "entry": "main.vey",
  "dependencies": {}
}
```

The `init` command initializes the current directory. It does not automatically create a directory named after the project.

### 2. Create a Package

Create a package directory:

```text
example-package/
├── veyra.json
└── example.veymod
```

Add the following package manifest to `example-package/veyra.json`:

```json
{
  "name": "example-package",
  "module": "example",
  "version": "0.1.0",
  "entry": "example.veymod",
  "dependencies": {}
}
```

Create `example.veymod`:

```veyra
set PACKAGE_VERSION = "0.1.0"

add greet(name)
    return f"Hello, {name}!"
end
```

The `name` field identifies the package in VPM.

The `module` field specifies the name used when importing the package in Veyra.

These names do not have to be identical.

### 3. Install the Package

From the project directory, run:

```bash
python /path/to/vpm/python/vpm.py add ../example-package
```

VPM copies the package into:

```text
veyra_packages/example-package/
```

The project manifest is updated:

```json
{
  "name": "my-project",
  "version": "0.1.0",
  "entry": "main.vey",
  "dependencies": {
    "example-package": {
      "path": "/absolute/path/to/example-package"
    }
  }
}
```

The source path is stored as an absolute path.

### 4. Import the Package

Update `main.vey`:

```veyra
mod example

say example.PACKAGE_VERSION
say example.greet("World")
```

Run the program:

```bash
veyra main.vey
```

Expected output:

```text
0.1.0
Hello, World!
```

Veyra searches the project's `veyra_packages/` directory and uses each package's `veyra.json` manifest to resolve its module entry point.

## Commands

### `init`

Initialize a Veyra project in the current directory.

```bash
python vpm.py init my-project
```

Creates `veyra.json` and `main.vey` if the entry file does not already exist.

### `add`

Install a package from a local directory.

```bash
python vpm.py add ../example-package
```

If the package is already installed, VPM updates it.

The package source directory must contain a valid `veyra.json` manifest.

### `remove`

Remove a registered dependency and its installed package directory.

```bash
python vpm.py remove example-package
```

This command does not delete the original package source directory.

### `install`

Install or reinstall all dependencies declared in the project manifest.

```bash
python vpm.py install
```

VPM currently supports local path dependencies only.

### `list`

List the project's registered dependencies and their source paths.

```bash
python vpm.py list
```

### `--version`

Display the VPM version.

```bash
python vpm.py --version
```

## Project Structure

A typical Veyra project using VPM looks like this:

```text
my-project/
├── main.vey
├── veyra.json
└── veyra_packages/
    └── example-package/
        ├── veyra.json
        └── example.veymod
```

## Manifest Format

VPM uses `veyra.json` for both projects and packages.

### Project Manifest

```json
{
  "name": "my-project",
  "version": "0.1.0",
  "entry": "main.vey",
  "dependencies": {
    "example-package": {
      "path": "/absolute/path/to/example-package"
    }
  }
}
```

### Package Manifest

```json
{
  "name": "example-package",
  "module": "example",
  "version": "0.1.0",
  "entry": "example.veymod",
  "dependencies": {}
}
```

Package fields:

- `name`: Package identifier used by VPM.
- `module`: Module name used in Veyra imports.
- `version`: Package version.
- `entry`: Entry point of the package, ending in `.veymod`.
- `dependencies`: Package dependency declarations.

Package-level dependencies are reserved for future dependency resolution support.

## Package Security

VPM includes several protections for local package operations:

- Package names are validated before installation or removal.
- Symbolic links in package contents are rejected.
- Package source and destination paths are checked for conflicts.
- New packages are copied into staging directories before installation.
- Existing installations are backed up during replacement.
- Failed installation and manifest update operations can trigger rollback.
- Project manifests are written using temporary files and atomic replacement.

These mechanisms reduce the risk of accidental data loss.

However, VPM 0.1.0 does not guarantee complete filesystem transaction safety.

Power failures, concurrent package operations, and external filesystem modifications may still cause inconsistent states.

VPM does not currently provide cryptographic package verification. Only install packages from sources you trust.

## Running Tests

VPM includes automated regression tests.

From the `vpm/` directory:

```bash
python -m unittest discover -s tests -v
```

The current test suite contains 17 tests covering project initialization, package installation, updates, removal, path validation, symbolic link handling, and rollback behavior.

## Limitations

VPM 0.1.0 does not currently support:

- GitHub or other remote package sources
- Package registries
- Lockfiles (`veyra.lock`)
- Automatic dependency resolution
- Transitive dependencies
- Semantic version constraints
- Cryptographic package verification
- Offline package caching

## Roadmap

### VPM 0.2.0

Planned features:

- GitHub package installation
- Git tag and commit-based version selection
- Lockfile generation
- SHA-256 verification
- Package caching
- Secure archive extraction

### VPM 0.3.0

Planned features:

- Transitive dependency installation
- Semantic version constraints
- Dependency conflict detection
- Advanced dependency resolution

## License

VPM is part of the Veyra project.

See the [repository license](../LICENSE) for details.
