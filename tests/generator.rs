use mlcli::config::{
    DatasetStorage, DependencyFile, Documentation, EnvironmentManager, LintingFormatting,
    OpenSourceLicense, ProjectConfig, PyDataPackages, TestingFramework,
};
use mlcli::generator::renderer::Renderer;
use mlcli::templates::registry::Template;
use tempfile::tempdir;

fn config(output_path: &std::path::Path) -> ProjectConfig {
    ProjectConfig {
        name: output_path.to_string_lossy().into_owned(),
        repo_name: output_path.to_string_lossy().into_owned(),
        module_name: "example_project".to_owned(),
        description: "A generated test project".to_owned(),
        author: "octocat".to_owned(),
        python_version: "3.12".to_owned(),
        dataset_storage: DatasetStorage::None,
        bucket: None,
        aws_profile: None,
        environment_manager: EnvironmentManager::Virtualenv,
        dependency_file: DependencyFile::RequirementsTxt,
        pydata_packages: PyDataPackages::Basic,
        testing_framework: TestingFramework::Pytest,
        linting_formatting: LintingFormatting::Ruff,
        license: OpenSourceLicense::Mit,
        documentation: Documentation::Mkdocs,
        include_code_scaffold: true,
        template: Template::ScikitLearn,
    }
}

#[test]
fn renderer_creates_and_renders_scikit_learn_project() {
    let directory = tempdir().unwrap();
    let output_path = directory.path().join("example-project");

    Renderer::render(&config(&output_path)).unwrap();

    let readme = std::fs::read_to_string(output_path.join("README.md")).unwrap();
    assert!(!readme.trim().is_empty());
    assert!(output_path.join("requirements.txt").is_file());
    assert!(output_path.join("src/example_project/main.py").is_file());
    assert!(output_path.join("tests/test_smoke.py").is_file());
    assert!(output_path.join("LICENSE").is_file());
}

#[test]
fn renderer_interpolates_project_config_values_in_tera_files() {
    let directory = tempdir().unwrap();
    let template_path = directory.path().join("template");
    let output_path = directory.path().join("generated-project");
    std::fs::create_dir(&template_path).unwrap();
    std::fs::write(
        template_path.join("README.md.tera"),
        "{{ name }}|{{ description }}|{{ author }}|{{ template }}",
    )
    .unwrap();

    let config = ProjectConfig {
        name: "fraud-detector".to_owned(),
        repo_name: "fraud-detector".to_owned(),
        module_name: "fraud_detector".to_owned(),
        description: "Detects fraudulent transactions".to_owned(),
        author: "Ada Lovelace".to_owned(),
        python_version: "3.12".to_owned(),
        dataset_storage: DatasetStorage::S3,
        bucket: Some("datasets".to_owned()),
        aws_profile: Some("default".to_owned()),
        environment_manager: EnvironmentManager::Uv,
        dependency_file: DependencyFile::PyprojectToml,
        pydata_packages: PyDataPackages::Basic,
        testing_framework: TestingFramework::Pytest,
        linting_formatting: LintingFormatting::Ruff,
        license: OpenSourceLicense::Mit,
        documentation: Documentation::Mkdocs,
        include_code_scaffold: true,
        template: Template::ScikitLearn,
    };
    Renderer::render_to_directory(&config, &template_path, &output_path).unwrap();

    let readme = std::fs::read_to_string(output_path.join("README.md")).unwrap();
    assert_eq!(
        readme,
        "fraud-detector|Detects fraudulent transactions|Ada Lovelace|scikit-learn"
    );
    assert!(output_path.join("pyproject.toml").is_file());
    assert!(!output_path.join("requirements.txt").exists());
    assert!(output_path.join(".env.example").is_file());
    assert!(output_path.join("SETUP.md").is_file());
    assert!(output_path.join("docs/index.md").is_file());
}

#[test]
fn renderer_rejects_an_existing_output_directory() {
    let directory = tempdir().unwrap();
    let output_path = directory.path().join("existing-project");
    std::fs::create_dir(&output_path).unwrap();

    let error = Renderer::render(&config(&output_path)).unwrap_err();

    assert!(error.to_string().contains("Directory already exists"));
}
