fn main() {
    let manifest = std::fs::read_to_string("Cargo.toml")
        .expect("Failed to read Cargo.toml");

    let codename = manifest
        .lines()
        .skip_while(|line| line.trim() != "[package.metadata.veyra]")
        .skip(1)
        .take_while(|line| !line.trim().starts_with('['))
        .find_map(|line| {
            let (key, value) = line.split_once('=')?;

            if key.trim() == "codename" {
                Some(value.trim().trim_matches('"').to_string())
            } else {
                None
            }
        })
        .expect("Missing codename in Cargo.toml");

    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rustc-env=VEYRA_CODENAME={}", codename);
}