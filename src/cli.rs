use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "fm")]
#[command(version = "0.1.0")]
#[command(about = "Rust file manager CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    List {
      dir: PathBuf,  
    },
    Info {
      path: PathBuf,  
    },
}