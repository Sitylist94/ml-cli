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

    let config = ProjectConfig {
        name: projet_name,
        description,
        author,
        template,
        features: features.iter().map(|feature| feature.to_string()).collect(),
    };

    crate::generator::engine::Engine::generate(config)?;

    Ok(())
}
