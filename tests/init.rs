use mlcli::commands::init::build_config;
use mlcli::templates::registry::Template;

#[test]
fn build_config_preserves_answers_and_selected_features() {
    let config = build_config(
        "fraud-detector".to_owned(),
        "Detects fraudulent transactions".to_owned(),
        "Ada Lovelace".to_owned(),
        Template::ScikitLearn,
        vec!["Docker", "MLflow"],
    );

    assert_eq!(config.name, "fraud-detector");
    assert_eq!(config.description, "Detects fraudulent transactions");
    assert_eq!(config.author, "Ada Lovelace");
    assert!(matches!(config.template, Template::ScikitLearn));
    assert_eq!(config.features, vec!["Docker", "MLflow"]);
}

#[test]
fn build_config_allows_no_optional_features() {
    let config = build_config(
        "minimal-project".to_owned(),
        "Minimal project".to_owned(),
        "Grace Hopper".to_owned(),
        Template::PyTorch,
        vec![],
    );

    assert!(matches!(config.template, Template::PyTorch));
    assert!(config.features.is_empty());
}
