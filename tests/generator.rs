use mlcli::config::ProjectConfig;
use mlcli::generator::renderer::Renderer;
use mlcli::templates::registry::Template;
use tempfile::tempdir;

fn config(output_path: &std::path::Path) -> ProjectConfig {
    ProjectConfig {
        name: output_path.to_string_lossy().into_owned(),
        description: "A generated test project".to_owned(),
        author: "octocat".to_owned(),
        template: Template::ScikitLearn,
        features: vec!["Docker".to_owned()],
    }
}

#[test]
fn renderer_creates_and_renders_scikit_learn_project() {
    let directory = tempdir().unwrap();
    let output_path = directory.path().join("example-project");

    Renderer::render(&config(&output_path)).unwrap();

    let readme = std::fs::read_to_string(output_path.join("README.md")).unwrap();
    assert!(readme.contains("example-project"));
    assert!(readme.contains("A generated test project"));
    assert!(readme.contains("https://github.com/octocat/"));
    assert!(output_path.join("requirements.txt").is_file());
}

#[test]
fn renderer_rejects_an_existing_output_directory() {
    let directory = tempdir().unwrap();
    let output_path = directory.path().join("existing-project");
    std::fs::create_dir(&output_path).unwrap();

    let error = Renderer::render(&config(&output_path)).unwrap_err();

    assert!(error.to_string().contains("Directory already exists"));
}
