use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Retourne tous les fichiers (pas les dossiers) sous `template_dir`,
/// avec leur chemin relatif.
pub fn collect_template_files(template_dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(template_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let relative = entry.path().strip_prefix(template_dir)?.to_path_buf();
        files.push(relative);
    }

    Ok(files)
}

/// Crée les dossiers parents puis écrit le contenu.
pub fn write_file(output_root: &Path, relative: &Path, content: &str) -> anyhow::Result<()> {
    let destination = output_root.join(relative);

    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&destination, content)?;
    Ok(())
}

/// Résout le chemin du template embarqué / local.
pub fn template_path(template_folder: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("templates")
        .join(template_folder)
}
