use crate::config::{
    DatasetStorage, DependencyFile, Documentation, EnvironmentManager, LintingFormatting,
    OpenSourceLicense, ProjectConfig, PyDataPackages, TestingFramework,
};
use crate::templates::registry::Template;
use inquire::{Confirm, Select, Text};

pub fn run() -> anyhow::Result<()> {
    let name = prompt("Project name (Nom lisible du projet):")?;
    let repo_name = prompt("Repo name (Nom du repository/dossier):")?;
    let module_name = prompt("Module name (Nom du module Python):")?;
    let author = prompt("Author name (Nom de l'auteur):")?;
    let description = prompt("Description (Description courte du projet):")?;
    let python_version = Text::new("Python version number:")
        .with_default("3.12")
        .prompt()?;
    let dataset_storage = select(
        "Dataset storage:",
        &["none", "azure", "s3", "gcs"],
        |v| match v {
            "azure" => DatasetStorage::Azure,
            "s3" => DatasetStorage::S3,
            "gcs" => DatasetStorage::Gcs,
            _ => DatasetStorage::None,
        },
    )?;
    let bucket = (!matches!(dataset_storage, DatasetStorage::None))
        .then(|| prompt("Bucket (Bucket de stockage):"))
        .transpose()?;
    let aws_profile = matches!(dataset_storage, DatasetStorage::S3)
        .then(|| {
            Text::new("AWS profile (Profil AWS):")
                .with_default("default")
                .prompt()
        })
        .transpose()?;
    let environment_manager = select(
        "Environment manager:",
        &[
            "virtualenv",
            "conda",
            "pipenv",
            "uv",
            "pixi",
            "poetry",
            "none",
        ],
        |v| match v {
            "conda" => EnvironmentManager::Conda,
            "pipenv" => EnvironmentManager::Pipenv,
            "uv" => EnvironmentManager::Uv,
            "pixi" => EnvironmentManager::Pixi,
            "poetry" => EnvironmentManager::Poetry,
            "none" => EnvironmentManager::None,
            _ => EnvironmentManager::Virtualenv,
        },
    )?;
    let dependency_file = select(
        "Dependency file:",
        &[
            "requirements.txt",
            "pyproject.toml",
            "environment.yml",
            "Pipfile",
            "pixi.toml",
        ],
        |v| match v {
            "pyproject.toml" => DependencyFile::PyprojectToml,
            "environment.yml" => DependencyFile::EnvironmentYml,
            "Pipfile" => DependencyFile::Pipfile,
            "pixi.toml" => DependencyFile::PixiToml,
            _ => DependencyFile::RequirementsTxt,
        },
    )?;
    let pydata_packages = select("PyData packages:", &["none", "basic"], |v| {
        if v == "basic" {
            PyDataPackages::Basic
        } else {
            PyDataPackages::None
        }
    })?;
    let testing_framework =
        select(
            "Testing framework:",
            &["none", "pytest", "unittest"],
            |v| match v {
                "pytest" => TestingFramework::Pytest,
                "unittest" => TestingFramework::Unittest,
                _ => TestingFramework::None,
            },
        )?;
    let linting_formatting = select(
        "Linting and formatting:",
        &["ruff", "flake8 + black + isort"],
        |v| {
            if v == "ruff" {
                LintingFormatting::Ruff
            } else {
                LintingFormatting::Flake8BlackIsort
            }
        },
    )?;
    let license = select(
        "Open-source license:",
        &["aucune", "MIT", "BSD-3-Clause"],
        |v| match v {
            "MIT" => OpenSourceLicense::Mit,
            "BSD-3-Clause" => OpenSourceLicense::Bsd3Clause,
            _ => OpenSourceLicense::None,
        },
    )?;
    let documentation = select("Documentation:", &["mkdocs", "none"], |v| {
        if v == "mkdocs" {
            Documentation::Mkdocs
        } else {
            Documentation::None
        }
    })?;
    let include_code_scaffold = Confirm::new("Include code scaffold?")
        .with_default(true)
        .prompt()?;
    let template = select(
        "Template:",
        &["Scikit-learn", "PyTorch", "TensorFlow"],
        |v| match v {
            "PyTorch" => Template::PyTorch,
            "TensorFlow" => Template::TensorFlow,
            _ => Template::ScikitLearn,
        },
    )?;

    crate::generator::engine::Engine::generate(ProjectConfig {
        name,
        repo_name,
        module_name,
        author,
        description,
        python_version,
        dataset_storage,
        bucket,
        aws_profile,
        environment_manager,
        dependency_file,
        pydata_packages,
        testing_framework,
        linting_formatting,
        license,
        documentation,
        include_code_scaffold,
        template,
    })
}

fn prompt(message: &str) -> anyhow::Result<String> {
    Ok(Text::new(message).prompt()?)
}

fn select<T>(message: &str, options: &[&str], map: impl FnOnce(&str) -> T) -> anyhow::Result<T> {
    Ok(map(Select::new(message, options.to_vec()).prompt()?))
}
