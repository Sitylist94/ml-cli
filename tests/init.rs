use mlcli::config::{
    DatasetStorage, DependencyFile, Documentation, EnvironmentManager, LintingFormatting,
    OpenSourceLicense, ProjectConfig, PyDataPackages, TestingFramework,
};
use mlcli::templates::registry::Template;

#[test]
fn project_config_keeps_all_generator_choices() {
    let config = ProjectConfig {
        name: "Fraud Detector".to_owned(),
        repo_name: "fraud-detector".to_owned(),
        module_name: "fraud_detector".to_owned(),
        author: "Ada Lovelace".to_owned(),
        description: "Detect fraud".to_owned(),
        python_version: "3.12".to_owned(),
        dataset_storage: DatasetStorage::S3,
        bucket: Some("ml-data".to_owned()),
        aws_profile: Some("research".to_owned()),
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
    assert_eq!(config.repo_name, "fraud-detector");
    assert_eq!(config.bucket.as_deref(), Some("ml-data"));
    assert!(matches!(config.environment_manager, EnvironmentManager::Uv));
    assert!(config.include_code_scaffold);
}
