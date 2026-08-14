use inquire::{MultiSelect, Select, Text};

pub fn run() {
    let _projet_name = Text::new("Project name:").prompt().unwrap();
    let _description = Text::new("Description:").prompt().unwrap();
    let _author = Text::new("Author:").prompt().unwrap();

    let _templates_options = vec!["Scikit-learn", "PyTorch", "TensorFlow"];
    let _template = Select::new("Choose a template:", _templates_options)
        .prompt()
        .unwrap();

    let _features_options = vec!["Docker", "Kubernetes", "MLflow", "DVC"];
    let _features = MultiSelect::new("Optional features:", _features_options)
        .prompt()
        .unwrap();
}
