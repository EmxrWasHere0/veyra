import importlib.util
import json
import os
import tempfile
import unittest

from pathlib import Path


VPM_FILE = Path(__file__).resolve().parents[1] / "python" / "vpm.py"

spec = importlib.util.spec_from_file_location("vpm", VPM_FILE)
vpm = importlib.util.module_from_spec(spec)
spec.loader.exec_module(vpm)


class VPMTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.original_cwd = Path.cwd()

        os.chdir(self.temp.name)

        self.project = Path.cwd()
        self.package_source = self.project / "example-package"

        self.package_source.mkdir()

        package_manifest = {
            "name": "example-package",
            "module": "example",
            "version": "0.1.0",
            "entry": "example.veymod",
            "dependencies": {},
        }

        (self.package_source / "veyra.json").write_text(
            json.dumps(package_manifest),
            encoding="utf-8",
        )

        (self.package_source / "example.veymod").write_text(
            'set PACKAGE_VERSION = "0.1.0"\n',
            encoding="utf-8",
        )

    def tearDown(self):
        os.chdir(self.original_cwd)
        self.temp.cleanup()

    def test_init_creates_project(self):
        vpm.init_project("test-project")

        self.assertTrue(Path("veyra.json").is_file())
        self.assertTrue(Path("main.vey").is_file())

        manifest = vpm.load_manifest()

        self.assertEqual(manifest["name"], "test-project")
        self.assertEqual(manifest["dependencies"], {})

    def test_init_rejects_existing_project(self):
        vpm.init_project("test-project")

        with self.assertRaises(RuntimeError):
            vpm.init_project("another-project")

    def test_add_package(self):
        vpm.init_project("test-project")
        vpm.add_package(str(self.package_source))

        installed = Path("veyra_packages/example-package")

        self.assertTrue(installed.is_dir())
        self.assertTrue((installed / "example.veymod").is_file())

        manifest = vpm.load_manifest()

        self.assertIn(
            "example-package",
            manifest["dependencies"],
        )

    def test_remove_package(self):
        vpm.init_project("test-project")
        vpm.add_package(str(self.package_source))

        vpm.remove_package("example-package")

        self.assertFalse(Path("veyra_packages/example-package").exists())

        manifest = vpm.load_manifest()

        self.assertNotIn(
            "example-package",
            manifest["dependencies"],
        )

    def test_install_restores_missing_package(self):
        vpm.init_project("test-project")
        vpm.add_package(str(self.package_source))

        installed = Path("veyra_packages/example-package")

        import shutil

        shutil.rmtree(installed)

        vpm.install_packages()

        self.assertTrue(installed.is_dir())
        self.assertTrue((installed / "example.veymod").is_file())

    def test_add_updates_existing_package(self):
        vpm.init_project("test-project")
        vpm.add_package(str(self.package_source))

        source_module = self.package_source / "example.veymod"

        source_module.write_text(
            'set PACKAGE_VERSION = "0.2.0"\n',
            encoding="utf-8",
        )

        vpm.add_package(str(self.package_source))

        installed_module = Path("veyra_packages/example-package/example.veymod")

        self.assertIn(
            "0.2.0",
            installed_module.read_text(encoding="utf-8"),
        )

    def test_invalid_package_name(self):
        invalid_names = [
            "../danger",
            "bad/name",
            "bad\\name",
            ".hidden",
            "",
            "123package",
        ]

        for name in invalid_names:
            with self.subTest(name=name):
                with self.assertRaises(RuntimeError):
                    vpm.validate_package_name(name)

    def test_install_rejects_name_mismatch(self):
        vpm.init_project("test-project")
        vpm.add_package(str(self.package_source))

        package_manifest_path = self.package_source / "veyra.json"

        package_manifest = json.loads(package_manifest_path.read_text(encoding="utf-8"))

        package_manifest["name"] = "different-package"

        package_manifest_path.write_text(
            json.dumps(package_manifest),
            encoding="utf-8",
        )

        with self.assertRaises(RuntimeError):
            vpm.install_packages()

        installed = Path("veyra_packages/example-package/example.veymod")

        self.assertTrue(installed.is_file())

    def test_add_rejects_source_inside_destination(self):
        vpm.init_project("test-project")

        source = Path("veyra_packages/example-package/source")
        source.mkdir(parents=True)

        (source / "veyra.json").write_text(
            json.dumps(
                {
                    "name": "example-package",
                    "module": "example",
                    "version": "0.1.0",
                    "entry": "example.veymod",
                    "dependencies": {},
                }
            ),
            encoding="utf-8",
        )

        with self.assertRaises(RuntimeError):
            vpm.add_package(str(source))

    def test_missing_manifest(self):
        with self.assertRaises(RuntimeError):
            vpm.load_manifest()

    def test_add_rejects_symlink_in_package(self):
        vpm.init_project("test-project")

        external = self.project / "external.txt"
        external.write_text("private data", encoding="utf-8")

        link = self.package_source / "external-link.txt"

        try:
            link.symlink_to(external)
        except (OSError, NotImplementedError):
            self.skipTest("Symbolic links are not available")

        with self.assertRaises(RuntimeError):
            vpm.add_package(str(self.package_source))

        self.assertFalse(Path("veyra_packages/example-package").exists())

    def test_install_rejects_invalid_json(self):
        vpm.init_project("test-project")
        vpm.add_package(str(self.package_source))

        installed = Path("veyra_packages/example-package/example.veymod")

        original_content = installed.read_text(encoding="utf-8")

        (self.package_source / "veyra.json").write_text(
            "{ invalid json",
            encoding="utf-8",
        )

        with self.assertRaises(json.JSONDecodeError):
            vpm.install_packages()

        self.assertEqual(
            installed.read_text(encoding="utf-8"),
            original_content,
        )

    def test_failed_copy_preserves_installed_package(self):
        from unittest.mock import patch

        vpm.init_project("test-project")
        vpm.add_package(str(self.package_source))

        installed = Path("veyra_packages/example-package/example.veymod")

        original_content = installed.read_text(encoding="utf-8")

        with patch.object(
            vpm.shutil,
            "copytree",
            side_effect=OSError("Simulated copy failure"),
        ):
            with self.assertRaises(OSError):
                vpm.install_packages()

        self.assertEqual(
            installed.read_text(encoding="utf-8"),
            original_content,
        )

    def test_remove_rejects_symlink_destination(self):
        vpm.init_project("test-project")
        vpm.add_package(str(self.package_source))

        destination = Path("veyra_packages/example-package")

        import shutil

        shutil.rmtree(destination)

        external = self.project / "external-directory"
        external.mkdir()

        try:
            destination.symlink_to(external, target_is_directory=True)
        except (OSError, NotImplementedError):
            self.skipTest("Symbolic links are not available")

        with self.assertRaises(RuntimeError):
            vpm.remove_package("example-package")

        self.assertTrue(external.is_dir())

        manifest = vpm.load_manifest()
        self.assertIn("example-package", manifest["dependencies"])

    def test_add_manifest_failure_preserves_existing_registration(self):
        from unittest.mock import patch

        vpm.init_project("test-project")
        vpm.add_package(str(self.package_source))

        original_manifest = vpm.load_manifest()

        with patch.object(
            vpm,
            "save_manifest",
            side_effect=OSError("Simulated manifest write failure"),
        ):
            with self.assertRaises(OSError):
                vpm.add_package(str(self.package_source))

        self.assertEqual(vpm.load_manifest(), original_manifest)

    def test_add_manifest_failure_rolls_back_package(self):
        from unittest.mock import patch

        vpm.init_project("test-project")
        vpm.add_package(str(self.package_source))

        installed = Path("veyra_packages/example-package/example.veymod")

        original_content = installed.read_text(encoding="utf-8")

        source_module = self.package_source / "example.veymod"
        source_module.write_text(
            'set PACKAGE_VERSION = "0.2.0"\n',
            encoding="utf-8",
        )

        with patch.object(
            vpm,
            "save_manifest",
            side_effect=OSError("Simulated manifest failure"),
        ):
            with self.assertRaises(OSError):
                vpm.add_package(str(self.package_source))

        self.assertEqual(
            installed.read_text(encoding="utf-8"),
            original_content,
        )

    def test_remove_manifest_failure_restores_package(self):
        from unittest.mock import patch

        vpm.init_project("test-project")
        vpm.add_package(str(self.package_source))

        destination = Path("veyra_packages/example-package")

        with patch.object(
            vpm,
            "save_manifest",
            side_effect=OSError("Simulated manifest failure"),
        ):
            with self.assertRaises(OSError):
                vpm.remove_package("example-package")

        self.assertTrue(destination.is_dir())

        manifest = vpm.load_manifest()

        self.assertIn(
            "example-package",
            manifest["dependencies"],
        )


if __name__ == "__main__":
    unittest.main()
