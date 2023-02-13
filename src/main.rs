use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use owo_colors::OwoColorize;

const SHELL: &str = "zsh";

#[derive(Parser, Debug)]
struct Args {
    template: Option<String>,
    #[clap(short, long, default_value = SHELL)]
    run: String,
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

        // let shell = std::env::var("SHELL")?;
        let run_flag = args.run;

        template.set_extension("nix");

        println!(
            "[{}] Executing {} {}",
            "shell".cyan(),
            "nix-shell".blue(),
            template.display().green()
        );

        let status = std::process::Command::new("nix-shell")
            .arg("--run")
            .arg(run_flag)
            .arg(template)
            .status()?;

        println!(
            "[{}] {} exited with {}",
            "shell".cyan(),
            "nix-shell".blue(),
            if status.success() {
                status.green().to_string()
            } else {
                status.red().to_string()
            }
        )
    } else {
        let path = get_template_path()?;
        let templates = get_templates(&path)?;

        println!("[{}] Available templates:", "shell".cyan());

        for template in templates {
            println!(
                "  - {}",
                template
                    .file_stem()
                    .expect("Could not get file stem")
                    .to_str()
                    .unwrap()
                    .blue()
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
