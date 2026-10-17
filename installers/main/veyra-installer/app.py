from __future__ import annotations

import queue
import shutil
import socket
import subprocess
import threading
from pathlib import Path
from PIL import Image

import customtkinter as ctk


APP_VERSION = "1.3.0"
REPO_URL = "https://github.com/EmxrWasHere0/veyra.git"
REPO_NAME = "veyra"
VSIX_PATH = Path("veyra-vscode") / "veyra-0.2.3.vsix"

ctk.set_appearance_mode("dark")
ctk.set_default_color_theme("dark-blue")


class VeyraInstaller(ctk.CTk):
    def __init__(self):
        super().__init__()

        self.title("Veyra Installer")
        self.geometry("760x560")
        self.minsize(680, 500)
        self.configure(fg_color="#09090D")

        self.purple = "#8B5CF6"
        self.purple_hover = "#7444DB"
        self.panel = "#15151F"
        self.text_color = "#F4F1FF"
        self.muted = "#A4A4B5"

        self.events = queue.Queue()
        self.page = "welcome"
        self.logs = []
        self.success = False
        self.error_message = ""

        self.install_extension = ctk.BooleanVar(value=True)
        self.ide = ctk.StringVar(value="code")

        self.build_layout()
        self.show_page("welcome")
        self.after(100, self.process_events)

    def build_layout(self):
        header = ctk.CTkFrame(
            self,
            fg_color="#0D0D13",
            corner_radius=0,
            height=82,
        )
        header.pack(fill="x")
        header.pack_propagate(False)

        logo = ctk.CTkFrame(
            header,
            width=46,
            height=46,
            corner_radius=13,
            fg_color=self.purple,
        )
        logo.pack(side="left", padx=(26, 13), pady=18)
        logo.pack_propagate(False)

        logo_image = ctk.CTkImage(
            light_image=Image.open(Path(__file__).parent / "veyra.png"),
            dark_image=Image.open(Path(__file__).parent / "veyra.png"),
            size=(36, 36),
        )

        ctk.CTkLabel(
            logo,
            text="",
            image=logo_image,
        ).place(relx=0.5, rely=0.5, anchor="center")

        brand = ctk.CTkFrame(header, fg_color="transparent")
        brand.pack(side="left", pady=16)

        ctk.CTkLabel(
            brand,
            text="VEYRA",
            text_color=self.text_color,
            font=ctk.CTkFont(size=20, weight="bold"),
        ).pack(anchor="w")

        ctk.CTkLabel(
            brand,
            text="Global Installer",
            text_color=self.muted,
            font=ctk.CTkFont(size=12),
        ).pack(anchor="w")

        ctk.CTkLabel(
            header,
            text=f"v{APP_VERSION}",
            text_color="#88889B",
        ).pack(side="right", padx=27)

        self.content = ctk.CTkFrame(
            self,
            fg_color="transparent",
        )
        self.content.pack(
            fill="both",
            expand=True,
            padx=28,
            pady=(22, 12),
        )

        footer = ctk.CTkFrame(
            self,
            fg_color="#0D0D13",
            corner_radius=0,
            height=76,
        )
        footer.pack(fill="x", side="bottom")
        footer.pack_propagate(False)

        self.step_label = ctk.CTkLabel(
            footer,
            text="STEP 1 OF 4",
            text_color=self.muted,
        )
        self.step_label.pack(side="left", padx=28)

        self.back_button = ctk.CTkButton(
            footer,
            text="Back",
            width=100,
            height=38,
            fg_color="#242431",
            hover_color="#303041",
            command=self.go_back,
        )
        self.back_button.pack(
            side="right",
            padx=(8, 26),
            pady=18,
        )

        self.next_button = ctk.CTkButton(
            footer,
            text="Next",
            width=120,
            height=38,
            fg_color=self.purple,
            hover_color=self.purple_hover,
            command=self.go_next,
        )
        self.next_button.pack(side="right", pady=18)

    def clear_content(self):
        for widget in self.content.winfo_children():
            widget.destroy()

    def heading(self, title, subtitle):
        ctk.CTkLabel(
            self.content,
            text=title,
            anchor="w",
            text_color=self.text_color,
            font=ctk.CTkFont(size=25, weight="bold"),
        ).pack(fill="x", pady=(0, 7))

        ctk.CTkLabel(
            self.content,
            text=subtitle,
            anchor="w",
            justify="left",
            wraplength=650,
            text_color=self.muted,
            font=ctk.CTkFont(size=13),
        ).pack(fill="x", pady=(0, 20))

    def show_page(self, page):
        self.page = page
        self.clear_content()

        if page == "welcome":
            self.step_label.configure(text="STEP 1 OF 4")

            self.heading(
                "Welcome to Veyra",
                "Install the Veyra language tools on this computer.",
            )

            card = ctk.CTkFrame(
                self.content,
                fg_color=self.panel,
                corner_radius=16,
            )
            card.pack(fill="x")

            ctk.CTkLabel(
                card,
                text="Your coding environment, ready.",
                text_color=self.text_color,
                font=ctk.CTkFont(size=17, weight="bold"),
                anchor="w",
            ).pack(fill="x", padx=22, pady=(22, 8))

            ctk.CTkLabel(
                card,
                text=(
                    "The installer checks your internet connection, "
                    "Git, Rust and Cargo. It downloads Veyra, installs "
                    "the CLI, and can optionally install the editor "
                    "extension."
                ),
                text_color=self.muted,
                justify="left",
                anchor="w",
                wraplength=590,
            ).pack(fill="x", padx=22, pady=(0, 22))

            self.back_button.configure(state="disabled")
            self.next_button.configure(
                text="Continue",
                state="normal",
            )

        elif page == "options":
            self.step_label.configure(text="STEP 2 OF 4")

            self.heading(
                "Installation options",
                "Choose your preferred editor.",
            )

            card = ctk.CTkFrame(
                self.content,
                fg_color=self.panel,
                corner_radius=16,
            )
            card.pack(fill="x")

            ctk.CTkCheckBox(
                card,
                text="Install the Veyra VS Code extension",
                variable=self.install_extension,
                fg_color=self.purple,
                hover_color=self.purple_hover,
            ).pack(anchor="w", padx=22, pady=(22, 16))

            ctk.CTkLabel(
                card,
                text="Editor command",
                text_color=self.muted,
            ).pack(anchor="w", padx=22, pady=(0, 8))

            ctk.CTkOptionMenu(
                card,
                variable=self.ide,
                values=[
                    "code",
                    "codium",
                    "code-insiders",
                    "skip",
                ],
                fg_color="#272435",
                button_color=self.purple,
                button_hover_color=self.purple_hover,
            ).pack(anchor="w", padx=22, pady=(0, 22))

            self.back_button.configure(state="normal")
            self.next_button.configure(
                text="Install",
                state="normal",
            )

        elif page == "install":
            self.step_label.configure(text="STEP 3 OF 4")

            self.heading(
                "Installing Veyra",
                "Please wait while the installation runs.",
            )

            self.progress = ctk.CTkProgressBar(
                self.content,
                height=10,
                progress_color=self.purple,
                fg_color="#292536",
            )
            self.progress.pack(fill="x", pady=(0, 8))
            self.progress.set(0)

            self.status_label = ctk.CTkLabel(
                self.content,
                text="Preparing installation...",
                anchor="w",
                text_color="#D8CCFF",
            )
            self.status_label.pack(fill="x", pady=(0, 12))

            self.log_box = ctk.CTkTextbox(
                self.content,
                fg_color="#0D0D13",
                text_color="#D9D6E8",
                border_color="#2B2738",
                border_width=1,
                font=ctk.CTkFont(
                    family="Consolas",
                    size=11,
                ),
            )
            self.log_box.pack(fill="both", expand=True)
            self.log_box.configure(state="disabled")

            self.back_button.configure(state="disabled")
            self.next_button.configure(
                text="Installing...",
                state="disabled",
            )

            # Tkinter variables must be read on the UI thread.
            install_extension = (
                self.install_extension.get() and self.ide.get() != "skip"
            )
            selected_ide = self.ide.get()

            threading.Thread(
                target=self.install_worker,
                args=(install_extension, selected_ide),
                daemon=True,
            ).start()

        elif page == "finish":
            self.step_label.configure(text="STEP 4 OF 4")

            if self.success:
                self.heading(
                    "Veyra is installed",
                    "Setup finished successfully.",
                )
                message = (
                    "Veyra was installed with Cargo. Open a new "
                    "terminal to use it. If the command is not found, "
                    "check that Cargo's bin directory is on PATH."
                )
                color = "#A78BFA"
            else:
                self.heading(
                    "Installation failed",
                    "The installer could not finish.",
                )
                message = self.error_message
                color = "#FCA5A5"

            card = ctk.CTkFrame(
                self.content,
                fg_color=self.panel,
                corner_radius=16,
            )
            card.pack(fill="x")

            ctk.CTkLabel(
                card,
                text=message,
                text_color=color,
                anchor="w",
                justify="left",
                wraplength=610,
            ).pack(fill="x", padx=22, pady=22)

            self.back_button.configure(state="disabled")
            self.next_button.configure(
                text="Finish",
                state="normal",
            )

    def go_back(self):
        if self.page == "options":
            self.show_page("welcome")

    def go_next(self):
        if self.page == "welcome":
            self.show_page("options")
        elif self.page == "options":
            self.show_page("install")
        elif self.page == "finish":
            self.destroy()

    def log(self, message):
        self.events.put(("log", message))

    def stage(self, progress, message):
        self.events.put(("stage", (progress, message)))
        self.log(f"[STEP] {message}")

    def run_command(self, command, error_message):
        self.log("> " + subprocess.list2cmdline(command))

        try:
            process = subprocess.Popen(
                command,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
                encoding="utf-8",
                errors="replace",
                creationflags=getattr(
                    subprocess,
                    "CREATE_NO_WINDOW",
                    0,
                ),
            )
        except OSError as exc:
            raise RuntimeError(f"{error_message}\n{exc}") from exc

        assert process.stdout is not None

        for line in process.stdout:
            line = line.rstrip()
            if line:
                self.log(line)

        code = process.wait()

        if code != 0:
            raise RuntimeError(f"{error_message} (exit code: {code})")

    def install_worker(self, install_extension, ide):
        try:
            self.stage(
                0.08,
                "Checking internet connection...",
            )

            try:
                with socket.create_connection(
                    ("github.com", 443),
                    timeout=5,
                ):
                    pass
            except OSError as exc:
                raise RuntimeError(
                    "Cannot connect to github.com. " "Check your internet connection."
                ) from exc

            self.log("[OK] Internet connection available.")

            self.stage(0.20, "Checking Git...")
            git = shutil.which("git")

            if not git:
                raise RuntimeError(
                    "Git was not found. Install Git "
                    "from https://git-scm.com/download/ "
                    "and restart the installer."
                )

            self.log(f"[OK] Git found: {git}")

            self.stage(0.32, "Checking Rust and Cargo...")

            if not shutil.which("rustup") and not shutil.which("rustc"):
                raise RuntimeError(
                    "Rust was not found. Install it from "
                    "https://rustup.rs/ and restart the installer."
                )

            cargo = shutil.which("cargo")

            if not cargo:
                raise RuntimeError(
                    "Cargo was not found. Restart the installer "
                    "after installing Rust, or check PATH."
                )

            self.log("[OK] Rust and Cargo found.")

            self.stage(
                0.44,
                "Preparing the Veyra repository...",
            )

            repo = Path.home() / "Veyra" / REPO_NAME
            repo.parent.mkdir(parents=True, exist_ok=True)

            if repo.exists():
                if not (repo / ".git").exists():
                    raise RuntimeError(
                        f"{repo} exists but is not a Git repository. "
                        "Rename that folder and retry."
                    )

                self.run_command(
                    [
                        git,
                        "-C",
                        str(repo),
                        "pull",
                        "--ff-only",
                    ],
                    "Could not update the Veyra repository.",
                )
            else:
                self.run_command(
                    [
                        git,
                        "clone",
                        REPO_URL,
                        str(repo),
                    ],
                    "Could not clone the Veyra repository.",
                )

            if not (repo / "Cargo.toml").is_file():
                raise RuntimeError(
                    f"Cargo.toml was not found in {repo}. "
                    "Check the repository layout."
                )

            self.stage(
                0.72,
                "Installing Veyra with Cargo...",
            )

            self.run_command(
                [
                    cargo,
                    "install",
                    "--path",
                    str(repo),
                ],
                "Cargo could not install Veyra.",
            )

            if install_extension:
                self.stage(
                    0.90,
                    "Installing editor extension...",
                )

                ide_path = shutil.which(ide)
                vsix = repo / VSIX_PATH

                if not ide_path:
                    self.log(
                        f"[WARN] '{ide}' was not found; " "skipping the extension."
                    )
                elif not vsix.is_file():
                    self.log(
                        f"[WARN] Extension not found at {vsix}; "
                        "skipping the extension."
                    )
                else:
                    self.run_command(
                        [
                            ide_path,
                            "--install-extension",
                            str(vsix),
                        ],
                        f"Could not install the extension in {ide}.",
                    )
            else:
                self.log("[INFO] Extension installation skipped.")

            self.stage(1.0, "Installation complete.")
            self.events.put(("complete", (True, "")))

        except Exception as exc:
            self.log(f"[ERROR] {exc}")
            self.events.put(("complete", (False, str(exc))))

    def process_events(self):
        try:
            while True:
                event, payload = self.events.get_nowait()

                if event == "log":
                    self.logs.append(str(payload))

                    if self.page == "install":
                        self.log_box.configure(state="normal")
                        self.log_box.insert(
                            "end",
                            str(payload) + "\n",
                        )
                        self.log_box.see("end")
                        self.log_box.configure(state="disabled")

                elif event == "stage":
                    progress, message = payload

                    if self.page == "install":
                        self.progress.set(progress)
                        self.status_label.configure(text=message)

                elif event == "complete":
                    self.success, self.error_message = payload
                    self.show_page("finish")

        except queue.Empty:
            pass

        self.after(100, self.process_events)


if __name__ == "__main__":
    VeyraInstaller().mainloop()
