use crate::config::ProjectConfig;
use crate::templates::registry::Template;
use tera::Context;

pub fn build(config: &ProjectConfig) -> Context {
    let mut ctx = Context::new();

    ctx.insert("name", &config.name);
    ctx.insert("description", &config.description);
    ctx.insert("author", &config.author);
    ctx.insert("template", &template_name(&config.template));
    ctx.insert("features", &config.features);

    // Helpers booléens pratiques dans les templates
    ctx.insert("has_docker", &config.features.iter().any(|f| f == "Docker"));
    ctx.insert(
        "has_kubernetes",
        &config.features.iter().any(|f| f == "Kubernetes"),
    );
    ctx.insert("has_mlflow", &config.features.iter().any(|f| f == "MLflow"));
    ctx.insert("has_dvc", &config.features.iter().any(|f| f == "DVC"));

    ctx
}

fn template_name(template: &Template) -> &'static str {
    match template {
        Template::ScikitLearn => "scikit-learn",
        Template::PyTorch => "pytorch",
        Template::TensorFlow => "tensorflow",
    }
}
