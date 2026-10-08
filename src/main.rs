use dpino::*;

use clap::Parser;
use cli::{Cli, Commands};
use scanner::Scanner;
use installer::Installer;
use anyhow::{Context, Result};
use std::path::PathBuf;

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path, json } => {
            let mut scanner = Scanner::new()?;
            let packages = scanner.scan_directory(&path)
                .with_context(|| format!("Failed to scan directory: {}", path.display()))?;

            if json {
                println!("{}", serde_json::to_string_pretty(&packages)?);
            } else {
                println!("Found {} package(s):", packages.len());
                for pkg in &packages {
                    println!("  - {} ({}) - {}", pkg.name, pkg.package_type, pkg.source_path.display());
                }
            }
        }
        Commands::Inspect { path } => {
            let format = formats::detect_format(&path)
                .ok_or_else(|| anyhow::anyhow!("Unknown package format: {}", path.display()))?;
            
            let metadata = format.extract_metadata(&path)
                .with_context(|| format!("Failed to extract metadata from: {}", path.display()))?;

            println!("{}", serde_json::to_string_pretty(&metadata)?);
        }
        Commands::List { json } => {
            let scanner = Scanner::new()?;
            let packages = scanner.list_all();

            if json {
                println!("{}", serde_json::to_string_pretty(&packages)?);
            } else {
                println!("Cached packages ({}):", packages.len());
                for pkg in packages {
                    println!("  - {} ({}) - {}", pkg.name, pkg.package_type, pkg.source_path.display());
                }
            }
        }
        Commands::Extract { path, out } => {
            Installer::extract_package(&path, &out)
                .with_context(|| format!("Failed to extract package: {}", path.display()))?;
            println!("Extracted to: {}", out.display());
        }
        Commands::InstallDesktop { path, user, dry_run } => {
            let format = formats::detect_format(&path)
                .ok_or_else(|| anyhow::anyhow!("Unknown package format: {}", path.display()))?;
            
            let metadata = format.extract_metadata(&path)
                .with_context(|| format!("Failed to extract metadata from: {}", path.display()))?;

            let desktop_id = Installer::install_desktop(&metadata, user, dry_run)?;
            println!("Desktop entry installed: {}", desktop_id);
        }
        Commands::Uninstall { desktop_id, user } => {
            Installer::uninstall(&desktop_id, user)?;
            println!("Desktop entry uninstalled: {}", desktop_id);
        }
        Commands::Refresh => {
            let mut scanner = Scanner::new()?;
            if let Err(e) = std::process::Command::new("update-desktop-database")
                .arg(utils::get_xdg_applications_dir()?)
                .output()
            {
                log::warn!("Failed to refresh desktop database: {}", e);
            }
            println!("Cache and desktop database refreshed");
        }
        Commands::Browse { path } => {
            let mut scanner = Scanner::new()?;
            if let Some(scan_path) = path {
                scanner.scan_directory(&scan_path)?;
            }
            let mut tui = tui::Tui::new(scanner);
            tui.run()?;
        }
    }

    Ok(())
}

