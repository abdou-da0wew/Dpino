use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "dpino")]
#[command(about = "A Rust-powered package metadata extractor + desktop entry generator + installer + browser")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scan a directory for packages
    Scan {
        /// Directory to scan
        #[arg(short, long)]
        path: PathBuf,
        /// Output as JSON
        #[arg(short, long)]
        json: bool,
    },
    /// Inspect a package file
    Inspect {
        /// Package file to inspect
        path: PathBuf,
    },
    /// List all cached packages
    List {
        /// Output as JSON
        #[arg(short, long)]
        json: bool,
    },
    /// Extract a package to a directory
    Extract {
        /// Package file to extract
        path: PathBuf,
        /// Output directory
        #[arg(short, long)]
        out: PathBuf,
    },
    /// Install desktop entry for a package
    InstallDesktop {
        /// Package file to install
        path: PathBuf,
        /// Install for current user (default: true)
        #[arg(short, long, default_value = "true")]
        user: bool,
        /// Dry run (don't actually install)
        #[arg(long)]
        dry_run: bool,
    },
    /// Uninstall a desktop entry
    Uninstall {
        /// Desktop entry ID (e.g., "myapp.desktop")
        desktop_id: String,
        /// Uninstall from user directory (default: true)
        #[arg(short, long, default_value = "true")]
        user: bool,
    },
    /// Refresh desktop database and cache
    Refresh,
    /// Launch TUI browser
    Browse {
        /// Directory to scan on startup
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
}

