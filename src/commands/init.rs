use crate::config::ProjectConfig;
use crate::templates::registry::Template;
use inquire::{MultiSelect, Select, Text};

pub fn run() -> anyhow::Result<()> {
    let projet_name = Text::new("Project name:").prompt().unwrap();
    let description = Text::new("Description:").prompt().unwrap();
    let author = Text::new("Author:").prompt().unwrap();

    let templates_options = vec!["Scikit-learn", "PyTorch", "TensorFlow"];

    let selected_template = Select::new("Template:", templates_options)
        .prompt()
        .unwrap();

    let template = match selected_template {
        "Scikit-learn" => Template::ScikitLearn,
        "PyTorch" => Template::PyTorch,
        "TensorFlow" => Template::TensorFlow,
        _ => unreachable!(),
    };

    let features_options = vec!["Docker", "Kubernetes", "MLflow", "DVC"];

    let features = MultiSelect::new("Optional features:", features_options)
        .prompt()
        .unwrap();

    let config = build_config(projet_name, description, author, template, features);

    crate::generator::engine::Engine::generate(config)?;

    Ok(())
}

pub fn build_config(
    name: String,
    description: String,
    author: String,
    template: Template,
    features: Vec<&str>,
) -> ProjectConfig {
    ProjectConfig {
        name,
        description,
        author,
        template,
        features: features.into_iter().map(str::to_owned).collect(),
    }
}
