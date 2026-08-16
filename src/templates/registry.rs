#[derive(Debug)]
pub enum Template {
    ScikitLearn,
    PyTorch,
    TensorFlow,
}

#[derive(Debug)]
pub enum TemplateStatus {
    Supported,
    NotImplemented,
}

pub fn run(template: Template) -> TemplateStatus {
    let status = match template {
        Template::ScikitLearn => TemplateStatus::Supported,
        Template::PyTorch | Template::TensorFlow => TemplateStatus::NotImplemented,

    };
    return status;
}