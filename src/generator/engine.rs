use crate::config::ProjectConfig;
use crate::templates::registry::{self, TemplateStatus};

pub struct Engine;

impl Engine {
    pub fn generate(config: ProjectConfig) -> anyhow::Result<()> {
        let status = registry::run(&config.template);

        match status {
            TemplateStatus::Supported => {
                crate::generator::renderer::Renderer::render(&config)?;
                println!("Project '{}' generated successfully.", config.name);
                Ok(())
            }

            TemplateStatus::NotImplemented => {
                println!("This template is not implemented yet.");
                Ok(())
            }
        }
    }
}
