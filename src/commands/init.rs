use crate::config::ProjectConfig;
use inquire::{MultiSelect, Select, Text};

pub fn run() {
    let projet_name = Text::new("Project name:").prompt().unwrap();
    let description = Text::new("Description:").prompt().unwrap();
    let author = Text::new("Author:").prompt().unwrap();

    let templates_options = vec!["Scikit-learn", "PyTorch", "TensorFlow"];
    let template = Select::new("Choose a template:", templates_options)
        .prompt()
        .unwrap();

    let features_options = vec!["Docker", "Kubernetes", "MLflow", "DVC"];
    let features = MultiSelect::new("Optional features:", features_options)
        .prompt()
        .unwrap();

    let config = ProjectConfig {
        name: projet_name,
        description,
        author,
        template: template.to_string(),
        features: features
            .iter()
            .map(|features| features.to_string())
            .collect(),
    };

    println!("{:#?}", config);
}
