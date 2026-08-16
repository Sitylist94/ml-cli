use crate::config::ProjectConfig;
use crate::generator::{context, filesystem};
use crate::templates::registry::Template;
use std::fs;
use std::path::Path;
use tera::Tera;

pub struct Renderer;

impl Renderer {
    pub fn render(config: &ProjectConfig) -> anyhow::Result<()> {
        let folder = match config.template {
            Template::ScikitLearn => "scikit-learn",
            Template::PyTorch => "pytorch",
            Template::TensorFlow => "tensorflow",
        };

        let template_dir = filesystem::template_path(folder);
        Self::render_from_template_dir(config, &template_dir)
    }

    pub fn render_from_template_dir(
        config: &ProjectConfig,
        template_dir: &Path,
    ) -> anyhow::Result<()> {
        Self::render_to_directory(config, template_dir, Path::new(&config.name))
    }

    pub fn render_to_directory(
        config: &ProjectConfig,
        template_dir: &Path,
        output_root: &Path,
    ) -> anyhow::Result<()> {
        if !template_dir.exists() {
            anyhow::bail!("Template directory not found: {}", template_dir.display());
        }

        if output_root.exists() {
            anyhow::bail!("Directory already exists: {}", output_root.display());
        }
        fs::create_dir_all(output_root)?;

        let mut tera = Tera::default();
        let ctx = context::build(config);
        let files = filesystem::collect_template_files(template_dir)?;

        for relative in files {
            let source = template_dir.join(&relative);
            let raw = fs::read_to_string(&source)?;

            let content = if relative.extension().and_then(|e| e.to_str()) == Some("tera") {
                let name = relative.to_string_lossy();
                tera.add_raw_template(&name, &raw)?;
                tera.render(&name, &ctx)?
            } else {
                raw
            };

            let out_relative = strip_tera_extension(&relative);
            filesystem::write_file(output_root, &out_relative, &content)?;
        }

        Ok(())
    }
}

fn strip_tera_extension(path: &Path) -> std::path::PathBuf {
    let as_str = path.to_string_lossy();
    if let Some(stripped) = as_str.strip_suffix(".tera") {
        std::path::PathBuf::from(stripped)
    } else {
        path.to_path_buf()
    }
}
