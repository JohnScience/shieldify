use std::{path::Path, str::FromStr};

use anyhow::{Context, Result, bail};
use markdown::{to_mdast, ParseOptions, mdast::Node};

/// Relevant data from a Rust crate's manifest (Cargo.toml).
struct Manifest {
    crate_name: String,
    hosting_provider_data: HostingProviderData,
}

/// One of the hosting providers supported by <https://deps.rs/>.
#[allow(dead_code)]
enum HostingProviderData {
    GitHub(GithubData),
    GitLab,
    Bitbucket,
    Codeberg,
    Gitea,
}

struct GithubData {
    user: String,
    repo: String,
}

impl Manifest {
    fn parse(manifest: &Path) -> Result<Self> {
        let manifest_contents = std::fs::read_to_string(manifest)
            .with_context(|| format!("Failed to read manifest at {}", manifest.display()))?;

        let manifest_toml_val: toml::Value = toml::from_str(&manifest_contents)
            .with_context(|| format!("Failed to parse manifest at {}", manifest.display()))?;

        let package = manifest_toml_val.get("package")
            .with_context(|| format!("Failed to get package from manifest at {}. See: https://doc.rust-lang.org/cargo/reference/manifest.html#the-package-section", manifest.display()))?;

        let name = package.get("name")
            .with_context(|| format!("Failed to get name from package at {}. See: https://doc.rust-lang.org/cargo/reference/manifest.html#the-name-field", manifest.display()))?;

        let repository = package.get("repository")
            .with_context(|| format!("Failed to get repository from package at {}. See: https://doc.rust-lang.org/cargo/reference/manifest.html#the-repository-field", manifest.display()))?;

        let toml::Value::String(crate_name) = name else {
            bail!("Name field in the package at {} was expected to be a string", manifest.display());
        };

        let toml::Value::String(repository) = repository else {
            bail!("Repository field in the package at {} was expected to be a string", manifest.display());
        };

        // TODO: Support more hosting providers.
        let github_data: GithubData = repository.parse()
            .with_context(|| format!("Failed to parse repository URL in the package at {}", manifest.display()))?;


        Ok(Self {
            crate_name: crate_name.clone(),
            hosting_provider_data: HostingProviderData::GitHub(github_data),
        })
    }
}

impl HostingProviderData {
    fn deps_rs_provider_component(&self) -> &'static str {
        match self {
            Self::GitHub(_) => "github",
            _ => unimplemented!(),
        }
    }
}

impl FromStr for GithubData {
    type Err = anyhow::Error;

    fn from_str(orig_s: &str) -> std::result::Result<Self, Self::Err> {
        let wo_scheme: &str = orig_s.strip_prefix("https://").unwrap_or(orig_s);
        let wo_domain: &str = wo_scheme.strip_prefix("github.com/")
            .with_context(|| format!("Failed to get GitHub repository URL from {orig_s}"))?;
        let mut split = wo_domain.split('/');
        let user = split.next()
            .with_context(|| format!("Failed to get user from GitHub repository URL: {orig_s}"))?;
        let repo = split.next()
            .with_context(|| format!("Failed to get project from GitHub repository URL: {orig_s}"))?;
        Ok(Self {
            user: user.to_owned(),
            repo: repo.to_owned(),
        })
    }
}

fn main() -> Result<()> {
    let cur_dir = std::env::current_dir().unwrap();
    let manifest = cur_dir.join("Cargo.toml");
    let readme = cur_dir.join("README.md");

    if !manifest.exists() {
        let err_msg = format!(
            "A Rust crate expected.\n\
            Manifest [Cargo.toml] expected at {}", manifest.display()
        );
        bail!(err_msg);
    }

    if !readme.exists() {
        let err_msg = format!(
            "A README.md expected at {}", readme.display()
        );
        bail!(err_msg);
    }

    let Manifest { crate_name, hosting_provider_data } = Manifest::parse(&manifest)?;

    let provider = hosting_provider_data.deps_rs_provider_component();

    let HostingProviderData::GitHub(GithubData { user, repo }) = hosting_provider_data else {
        bail!("Only GitHub repos are supported for now");
    };

    let mut readme_contents: String = std::fs::read_to_string(&readme)
        .with_context(|| format!("Failed to read README.md at {}", readme.display()))?;

    let readme_mdast = match to_mdast(&readme_contents, &ParseOptions::default()) {
        Ok(mdast) => mdast,
        Err(err) => {
            let err_msg = format!(
                "Failed to parse README.md at {}\n\
                Error: {}", readme.display(), err
            );
            bail!(err_msg);
        }
    };

    let Node::Root(readme_mdast) = readme_mdast else {
        bail!("Invalid README.md at {}", readme.display());
    };
    
    let mut readme_mdast_it = readme_mdast.children.into_iter();

    let offset = loop {
        let Some(node) = readme_mdast_it.next() else {
            bail!("A heading expected in README.md at {}", readme.display());
        };
        if let Node::Heading(heading) = node {
            let depth = heading.depth;
            let position = heading.position
                .with_context(|| format!("Failed to get position of a heading with depth {depth} in README.md at {}", readme.display()))?;
            if depth != 1 {
                let start_line = position.start.line;
                let start_column = position.start.column;
                let end_line = position.end.line;
                let end_column = position.end.column;
                
                let err_msg = format!(
                    "The first heading in README.md at {} was expected to have depth 1.\n\
                    Found a heading with depth {depth} instead \
                    (look L{start_line}C{start_column}:L{end_line}C{end_column})", readme.display(), 
                );
                bail!(err_msg);
            }
            if depth == 1 {
                break position.end.offset;
            }
        }
    };

    let shields = format!(
        "\n\n\
        [![Crates.io](https://img.shields.io/crates/v/{crate_name})](https://crates.io/crates/{crate_name})\n\
        [![Downloads](https://img.shields.io/crates/d/{crate_name}.svg)](https://crates.io/crates/{crate_name})\n\
        [![Documentation](https://docs.rs/{crate_name}/badge.svg)](https://docs.rs/{crate_name})\n\
        [![License](https://img.shields.io/crates/l/{crate_name})](https://crates.io/crates/{crate_name})\n\
        [![Dependency Status](https://deps.rs/repo/{provider}/{user}/{repo}/status.svg)](https://deps.rs/repo/github/{user}/{repo})\
        "
    );

    readme_contents.insert_str(offset, &shields);

    std::fs::write(&readme, readme_contents)
        .with_context(|| format!("Failed to write to README.md at {}", readme.display()))?;

    Ok(())
}
