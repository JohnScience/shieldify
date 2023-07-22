use anyhow::{Context, Result, bail};
use markdown::{to_mdast, ParseOptions, mdast::Node};

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

    let manifest_contents = std::fs::read_to_string(&manifest)
        .with_context(|| format!("Failed to read manifest at {}", manifest.display()))?;

    let manifest_toml_val: toml::Value = toml::from_str(&manifest_contents)
        .with_context(|| format!("Failed to parse manifest at {}", manifest.display()))?;

    let package = manifest_toml_val.get("package")
        .with_context(|| format!("Failed to get package from manifest at {}", manifest.display()))?;

    let name = package.get("name")
        .with_context(|| format!("Failed to get name from package at {}", manifest.display()))?;

    let toml::Value::String(name) = name else {
        bail!("Name field in the package at {} was expected to be a string", manifest.display());
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
        [![Crates.io](https://img.shields.io/crates/v/{name})](https://crates.io/crates/{name})\n\
        [![Documentation](https://docs.rs/{name}/badge.svg)](https://docs.rs/{name})\n\
        [![License](https://img.shields.io/crates/l/{name})](https://crates.io/crates/{name})\
        "
    );

    readme_contents.insert_str(offset, &shields);

    std::fs::write(&readme, readme_contents)
        .with_context(|| format!("Failed to write to README.md at {}", readme.display()))?;

    Ok(())
}
