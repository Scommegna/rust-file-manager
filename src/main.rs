mod cli;
mod filesystem;
mod search;
mod tree;
mod benchmark;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    
    match cli.command { 
        Commands::List { dir } => {
            filesystem::list_dir(&dir)?;
        }
        Commands::Info { path } => {
            filesystem::show_info(&path)?;
        }
    }
    
    Ok(())
}
