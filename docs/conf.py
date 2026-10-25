# Sphinx configuration for Veyra documentation.

from pathlib import Path
import re

# -- Project information -----------------------------------------------------

project = "Veyra"
copyright = "2026, EmxrDev"
author = "EmxrDev"


def get_veyra_version():
    cargo_toml = Path(__file__).resolve().parent.parent / "Cargo.toml"

    if cargo_toml.exists():
        content = cargo_toml.read_text(encoding="utf-8")

        package_section = re.search(
            r"(?ms)^\[package\]\s*(.*?)(?=^\[|\Z)",
            content,
        )

        if package_section:
            version_match = re.search(
                r'(?m)^\s*version\s*=\s*"([^"]+)"',
                package_section.group(1),
            )

            if version_match:
                return version_match.group(1)

    return "0.2.7"


release = get_veyra_version()
version = release

# -- General configuration ---------------------------------------------------

extensions = ["myst_parser"]

templates_path = ["_templates"]

exclude_patterns = [
    "_build",
    "Thumbs.db",
    ".DS_Store",
]

# -- Options for HTML output -------------------------------------------------

html_theme = "shibuya"

html_static_path = ["_static"]
