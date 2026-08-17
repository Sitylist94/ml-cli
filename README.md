# mlcli

`mlcli` est une interface en ligne de commande écrite en Rust pour initialiser rapidement des projets de machine learning. Elle guide la création d’un projet à partir d’un template et injecte les informations saisies dans les fichiers du projet.

> Le template Scikit-learn est actuellement le seul template généré. PyTorch et TensorFlow sont proposés dans l’interface, mais ne sont pas encore implémentés.

## Fonctionnalités

- Assistant interactif pour créer un projet avec `mlcli init`.
- Template Scikit-learn embarqué dans le binaire : l’outil fonctionne depuis n’importe quel dossier après installation.
- Personnalisation du nom du projet, de sa description, de son auteur et des fonctionnalités optionnelles.
- Rendu des fichiers `.tera` avec les données du projet.
- Protection contre l’écrasement d’un dossier existant.
- Binaire distribuable pour Windows, macOS et Linux via les [releases GitHub](https://github.com/Sitylist94/ml-cli/releases).

## Installation

### Depuis une release

Téléchargez l’archive correspondant à votre système depuis la page des [releases](https://github.com/Sitylist94/ml-cli/releases), puis extrayez-la.

Sous Windows, ajoutez le dossier qui contient `mlcli.exe` à la variable d’environnement `Path`, ou exécutez le binaire depuis ce dossier.

```powershell
mlcli --help
```

### Depuis les sources

Prérequis : [Rust et Cargo](https://www.rust-lang.org/tools/install).

```bash
git clone https://github.com/Sitylist94/ml-cli.git
cd ml-cli
cargo install --path .
```

Pour lancer le projet sans l’installer :

```bash
cargo run -- init
```

## Utilisation

Placez-vous dans le dossier dans lequel vous souhaitez créer votre projet, puis lancez :

```bash
mlcli init
```

L’assistant vous demande :

1. le nom du projet ;
2. une description ;
3. le nom de l’auteur ;
4. le template souhaité ;
5. les fonctionnalités optionnelles.

Exemple :

```text
Project name: fraud-detector
Description: Detect fraudulent transactions
Author: Ada Lovelace
Template: Scikit-learn
Optional features: Docker, MLflow
```

Le dossier `fraud-detector/` est alors créé dans le dossier courant. Si ce dossier existe déjà, `mlcli` s’arrête afin de ne pas remplacer son contenu.

## Templates

### Scikit-learn

Le template Scikit-learn crée actuellement les fichiers fournis dans `templates/scikit-learn/`, notamment :

```text
<nom-du-projet>/
├── README.md
└── requirements.txt
```

Le fichier `README.md.tera` est rendu en `README.md`. Les variables suivantes sont disponibles dans les templates :

| Variable | Description |
| --- | --- |
| `name` | Nom du projet |
| `description` | Description du projet |
| `author` | Auteur du projet |
| `template` | Identifiant du template, par exemple `scikit-learn` |
| `features` | Liste des fonctionnalités sélectionnées |
| `has_docker` | `true` si Docker est sélectionné |
| `has_kubernetes` | `true` si Kubernetes est sélectionné |
| `has_mlflow` | `true` si MLflow est sélectionné |
| `has_dvc` | `true` si DVC est sélectionné |

Les options Docker, Kubernetes, MLflow et DVC sont disponibles dans l’assistant. Elles sont exposées aux templates pour leur permettre de générer des fichiers ou sections conditionnels.

### PyTorch et TensorFlow

Ces choix sont visibles dans l’assistant mais leur génération n’est pas encore disponible. Ils n’écrivent aucun fichier.

## Commandes

```text
mlcli init
```

Les sous-commandes `add`, `remove`, `validate`, `doctor` et `template` sont réservées à des évolutions futures. Elles ne doivent pas encore être utilisées.

Consultez l’aide de la CLI avec :

```bash
mlcli --help
```

## Développement

Exécutez les vérifications locales avant de contribuer :

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo build --release
```

Les templates sont embarqués dans le binaire pendant la compilation. Toute modification sous `templates/` est donc prise en compte lors du prochain build.

## Publication

Les releases sont gérées avec Release Please et les binaires sont produits avec `cargo-dist`. Utilisez des messages de commit conventionnels, par exemple :

```text
feat: add a new project template
fix: correct embedded template rendering
```

Release Please met à jour le changelog et prépare la version de release automatiquement.

## Licence

Ce projet est distribué sous licence [MIT](LICENSE).
