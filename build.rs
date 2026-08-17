use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn collect_files(directory: &Path, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(directory).expect("failed to read templates directory") {
        let path = entry.expect("failed to read template entry").path();
        if path.is_dir() {
            collect_files(&path, files);
        } else {
            files.push(path);
        }
    }
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let templates_dir = manifest_dir.join("templates");
    println!("cargo:rerun-if-changed={}", templates_dir.display());

    let mut files = Vec::new();
    collect_files(&templates_dir, &mut files);
    files.sort();

    let entries = files
        .iter()
        .map(|path| {
            let relative = path
                .strip_prefix(&templates_dir)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/");
            format!(
                "    ({relative:?}, include_str!({:?})),",
                path.to_string_lossy()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let generated = format!("pub static FILES: &[(&str, &str)] = &[\n{entries}\n];\n");
    let output = PathBuf::from(env::var("OUT_DIR").unwrap()).join("embedded_templates.rs");
    fs::write(output, generated).expect("failed to generate embedded template manifest");
}
