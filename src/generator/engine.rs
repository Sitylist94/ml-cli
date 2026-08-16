use crate::config::ProjectConfig;
use crate::templates::registry::{self, TemplateStatus};

pub struct Engine;

impl Engine {
    pub fn generate(config: ProjectConfig) -> anyhow::Result<()> {
        let status = registry::run(config.template);

        match status {
            TemplateStatus::Supported => {
                println!("Template supported, generation can start.");
                Ok(())
            }

            TemplateStatus::NotImplemented => {
                println!("This template is not implemented yet.");
                Ok(())
            }
        }
    }
}
