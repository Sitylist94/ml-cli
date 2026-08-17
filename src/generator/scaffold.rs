use crate::config::{
    DatasetStorage, DependencyFile, Documentation, EnvironmentManager, LintingFormatting,
    OpenSourceLicense, ProjectConfig, PyDataPackages, TestingFramework,
};
use crate::generator::filesystem;
use std::fs;
use std::path::Path;

pub fn write(config: &ProjectConfig, output_root: &Path) -> anyhow::Result<()> {
    write_dependencies(config, output_root)?;
    write_environment_instructions(config, output_root)?;
    write_dataset_config(config, output_root)?;
    write_linting(config, output_root)?;
    write_license(config, output_root)?;
    write_documentation(config, output_root)?;
    if config.include_code_scaffold {
        write_code_scaffold(config, output_root)?;
    }
    Ok(())
}

fn dependencies(config: &ProjectConfig) -> Vec<&'static str> {
    let mut packages = Vec::new();
    if matches!(config.pydata_packages, PyDataPackages::Basic) {
        packages.extend(["numpy", "pandas", "scikit-learn"]);
    }
    if config.testing_framework == TestingFramework::Pytest {
        packages.push("pytest");
    }
    match config.linting_formatting {
        LintingFormatting::Ruff => packages.push("ruff"),
        LintingFormatting::Flake8BlackIsort => packages.extend(["flake8", "black", "isort"]),
    }
    if matches!(config.documentation, Documentation::Mkdocs) {
        packages.push("mkdocs");
    }
    packages
}

fn write_dependencies(config: &ProjectConfig, root: &Path) -> anyhow::Result<()> {
    let packages = dependencies(config);
    let content = match config.dependency_file {
        DependencyFile::RequirementsTxt => packages.join("\n") + "\n",
        DependencyFile::PyprojectToml => format!("[project]\nname = {:?}\nversion = \"0.1.0\"\ndescription = {:?}\nrequires-python = \">={}\"\ndependencies = [{}]\n", config.repo_name, config.description, config.python_version, packages.iter().map(|p| format!("\n  {:?}", p)).collect::<Vec<_>>().join(",")),
        DependencyFile::EnvironmentYml => format!("name: {}\nchannels:\n  - conda-forge\ndependencies:\n  - python={}\n{}", config.repo_name, config.python_version, packages.iter().map(|p| format!("  - {p}\n")).collect::<String>()),
        DependencyFile::Pipfile => format!("[requires]\npython_version = {:?}\n\n[packages]\n{}", config.python_version, packages.iter().map(|p| format!("{p} = \"*\"\n")).collect::<String>()),
        DependencyFile::PixiToml => format!("[project]\nname = {:?}\nversion = \"0.1.0\"\n\n[dependencies]\n{}", config.repo_name, packages.iter().map(|p| format!("{p} = \"*\"\n")).collect::<String>()),
    };
    let filename = match config.dependency_file {
        DependencyFile::RequirementsTxt => "requirements.txt",
        DependencyFile::PyprojectToml => "pyproject.toml",
        DependencyFile::EnvironmentYml => "environment.yml",
        DependencyFile::Pipfile => "Pipfile",
        DependencyFile::PixiToml => "pixi.toml",
    };
    filesystem::write_file(root, Path::new(filename), &content)?;
    for candidate in [
        "requirements.txt",
        "pyproject.toml",
        "environment.yml",
        "Pipfile",
        "pixi.toml",
    ] {
        if candidate != filename {
            let path = root.join(candidate);
            if path.is_file() {
                fs::remove_file(path)?;
            }
        }
    }
    Ok(())
}

fn write_environment_instructions(config: &ProjectConfig, root: &Path) -> anyhow::Result<()> {
    let instructions = match config.environment_manager {
        EnvironmentManager::Virtualenv => format!(
            "python{} -m venv .venv\n. .venv/bin/activate\n",
            config.python_version
        ),
        EnvironmentManager::Conda => format!(
            "conda create -n {} python={}\nconda activate {}\n",
            config.repo_name, config.python_version, config.repo_name
        ),
        EnvironmentManager::Pipenv => "pipenv install\npipenv shell\n".to_owned(),
        EnvironmentManager::Uv => "uv venv\nuv sync\n".to_owned(),
        EnvironmentManager::Pixi => "pixi install\npixi shell\n".to_owned(),
        EnvironmentManager::Poetry => "poetry install\npoetry shell\n".to_owned(),
        EnvironmentManager::None => return Ok(()),
    };
    filesystem::write_file(
        root,
        Path::new("SETUP.md"),
        &format!("# Environment setup\n\n```bash\n{instructions}```\n"),
    )
}

fn write_dataset_config(config: &ProjectConfig, root: &Path) -> anyhow::Result<()> {
    if matches!(config.dataset_storage, DatasetStorage::None) {
        return Ok(());
    }
    let storage = match config.dataset_storage {
        DatasetStorage::Azure => "azure",
        DatasetStorage::S3 => "s3",
        DatasetStorage::Gcs => "gcs",
        DatasetStorage::None => unreachable!(),
    };
    let mut content = format!(
        "DATASET_STORAGE={storage}\nDATASET_BUCKET={}\n",
        config.bucket.as_deref().unwrap_or_default()
    );
    if let Some(profile) = &config.aws_profile {
        content.push_str(&format!("AWS_PROFILE={profile}\n"));
    }
    filesystem::write_file(root, Path::new(".env.example"), &content)
}

fn write_linting(config: &ProjectConfig, root: &Path) -> anyhow::Result<()> {
    match config.linting_formatting {
        LintingFormatting::Ruff => filesystem::write_file(
            root,
            Path::new("ruff.toml"),
            "target-version = \"py312\"\nline-length = 100\n",
        ),
        LintingFormatting::Flake8BlackIsort => filesystem::write_file(
            root,
            Path::new(".flake8"),
            "[flake8]\nmax-line-length = 100\n\n[isort]\nprofile = black\n",
        ),
    }
}

fn write_license(config: &ProjectConfig, root: &Path) -> anyhow::Result<()> {
    let content = match config.license {
        OpenSourceLicense::None => return Ok(()),
        OpenSourceLicense::Mit => format!("MIT License\n\nCopyright (c) 2026 {}\n\nPermission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the \"Software\"), to deal in the Software without restriction.\n", config.author),
        OpenSourceLicense::Bsd3Clause => format!("BSD 3-Clause License\n\nCopyright (c) 2026, {}\nAll rights reserved.\n\nRedistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met.\n", config.author),
    };
    filesystem::write_file(root, Path::new("LICENSE"), &content)
}

fn write_documentation(config: &ProjectConfig, root: &Path) -> anyhow::Result<()> {
    if !matches!(config.documentation, Documentation::Mkdocs) {
        return Ok(());
    }
    filesystem::write_file(
        root,
        Path::new("mkdocs.yml"),
        &format!("site_name: {}\n", config.name),
    )?;
    filesystem::write_file(
        root,
        Path::new("docs/index.md"),
        &format!("# {}\n\n{}\n", config.name, config.description),
    )
}

fn write_code_scaffold(config: &ProjectConfig, root: &Path) -> anyhow::Result<()> {
    filesystem::write_file(
        root,
        Path::new(&format!("src/{}/__init__.py", config.module_name)),
        "",
    )?;
    filesystem::write_file(root, Path::new(&format!("src/{}/main.py", config.module_name)), "def main() -> None:\n    print(\"Project ready\")\n\n\nif __name__ == \"__main__\":\n    main()\n")?;
    match config.testing_framework {
        TestingFramework::Pytest => filesystem::write_file(root, Path::new("tests/test_smoke.py"), &format!("from {}.main import main\n\n\ndef test_main() -> None:\n    main()\n", config.module_name)),
        TestingFramework::Unittest => filesystem::write_file(root, Path::new("tests/test_smoke.py"), &format!("import unittest\nfrom {}.main import main\n\n\nclass SmokeTest(unittest.TestCase):\n    def test_main(self) -> None:\n        main()\n", config.module_name)),
        TestingFramework::None => Ok(()),
    }
}
