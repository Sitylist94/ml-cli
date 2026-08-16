use crate::templates::registry::Template;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub description: String,
    pub author: String,
    pub template: Template,
    pub features: Vec<String>,
}
