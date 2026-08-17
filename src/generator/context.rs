use crate::config::ProjectConfig;
use crate::templates::registry::Template;
use tera::Context;

pub fn build(config: &ProjectConfig) -> Context {
    let mut ctx = Context::new();

    ctx.insert("name", &config.name);
    ctx.insert("repo_name", &config.repo_name);
    ctx.insert("module_name", &config.module_name);
    ctx.insert("description", &config.description);
    ctx.insert("author", &config.author);
    ctx.insert("python_version", &config.python_version);
    ctx.insert("template", &template_name(&config.template));
    ctx.insert("bucket", &config.bucket);
    ctx.insert("aws_profile", &config.aws_profile);
    ctx.insert("include_code_scaffold", &config.include_code_scaffold);

    ctx
}

fn template_name(template: &Template) -> &'static str {
    match template {
        Template::ScikitLearn => "scikit-learn",
        Template::PyTorch => "pytorch",
        Template::TensorFlow => "tensorflow",
    }
}
