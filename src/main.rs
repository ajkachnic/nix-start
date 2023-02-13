use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    template: Option<String>,
}

fn get_template_path() -> Result<PathBuf> {
    let mut path = home::home_dir().unwrap();
    path.push(".config");
    path.push("nix-start");
    Ok(path)
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(template) = args.template {
        let path = get_template_path()?;
        let mut template = path.join(template);

        let shell = std::env::var("SHELL")?;

        template.set_extension("nix");

        std::process::Command::new("nix-shell")
            .arg("--run")
            .arg(shell)
            .arg(template)
            .output()?;
    } else {
        // args.print_long_help();
        let path = get_template_path()?;
        let templates = get_templates(&path)?;

        println!("Available templates:");

        for template in templates {
            println!(
                "  - {}",
                template
                    .file_stem()
                    .expect("Could not get file stem")
                    .to_str()
                    .unwrap()
            );
        }
    }

    Ok(())
}

fn get_templates(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut templates = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            templates.append(&mut get_templates(&path)?);
        } else {
            templates.push(path);
        }
    }

    Ok(templates)
}
