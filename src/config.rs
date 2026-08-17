use crate::templates::registry::Template;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DatasetStorage {
    None,
    Azure,
    S3,
    Gcs,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnvironmentManager {
    Virtualenv,
    Conda,
    Pipenv,
    Uv,
    Pixi,
    Poetry,
    None,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DependencyFile {
    RequirementsTxt,
    PyprojectToml,
    EnvironmentYml,
    Pipfile,
    PixiToml,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PyDataPackages {
    None,
    Basic,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TestingFramework {
    None,
    Pytest,
    Unittest,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LintingFormatting {
    Ruff,
    Flake8BlackIsort,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OpenSourceLicense {
    None,
    Mit,
    Bsd3Clause,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Documentation {
    Mkdocs,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub repo_name: String,
    pub module_name: String,
    pub author: String,
    pub description: String,
    pub python_version: String,
    pub dataset_storage: DatasetStorage,
    pub bucket: Option<String>,
    pub aws_profile: Option<String>,
    pub environment_manager: EnvironmentManager,
    pub dependency_file: DependencyFile,
    pub pydata_packages: PyDataPackages,
    pub testing_framework: TestingFramework,
    pub linting_formatting: LintingFormatting,
    pub license: OpenSourceLicense,
    pub documentation: Documentation,
    pub include_code_scaffold: bool,
    pub template: Template,
}
