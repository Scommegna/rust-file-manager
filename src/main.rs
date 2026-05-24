mod filesystem;
mod tree;
mod search;

use std::env;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    loop {
        print!("fm> ");

        io::stdout()
            .flush()
            .expect("Error to clean output buffer");

        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error to read command.");
            continue;
        }

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "exit" {
            break;
        }

        handle_command(input);
    }
}

fn handle_command(input: &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();

    let command = parts[0];

    match command {
        "help" => show_help(),

        "list" => {
            if parts.len() != 2 {
                eprintln!("Usage: list <dir>");
                return;
            }

            let path = Path::new(parts[1]);

            if let Err(error) = filesystem::list_dir(path) {
                eprintln!("Error to list directory: {}", error);
            }
        },

        "info" => {
            if parts.len() != 2 {
                eprintln!("Usage: info <file or dir>");
                return;
            }

            let path = Path::new(parts[1]);

            if let Err(error) = filesystem::show_info(path) {
                eprintln!("Error to obtain information: {}", error);
            }
        },

        "copy" => {
          if parts.len() != 3 {
              eprintln!("Usage: copy <origin> <dest>");
              return;
          }

            let src = Path::new(parts[1]);
            let dst = Path::new(parts[2]);

            if let Err(error) = filesystem::copy_file(src, dst) {
                eprintln!("Error to copy file: {}", error);
            }
        },

        "move" => {
          if parts.len() != 3 {
              eprintln!("Usage: move <origin> <dest>");
              return;
          }

            let src = Path::new(parts[1]);
            let dst = Path::new(parts[2]);

            if let Err(error) = filesystem::move_file(src, dst) {
                eprintln!("Error to rename file: {}", error);
            }
        },

        "delete" => {
          if parts.len() != 2 {
              eprintln!("Usage: delete <path>");
              return;
          }

            let path = Path::new(parts[1]);

            if let Err(error) = filesystem::delete_path(path) {
                eprintln!("Error to delete: {}", error);
            }
        },

        "cd" => {
          if parts.len() != 2 {
              eprintln!("Usage: cd <dir>");
              return;
          }

            let path = Path::new(parts[1]);

            if let Err(error) = env::set_current_dir(path) {
                eprintln!("Error to change directory: {}", error);
                return;
            }

            match env::current_dir() {
                Ok(current_path) => {
                    println!("Current directory: {}", current_path.display());
                }
                Err(error) => {
                    eprintln!("Error to obtaion current directory: {}", error);
                }
            }
        },

        "tree" => {
          if parts.len() != 2 {
              eprintln!("Usage: tree <dir>");
              return;
          }

            let path = Path::new(parts[1]);

            if let Err(error) = tree::print_tree(path) {
                eprintln!("Error to print folder tree: {}", error)
            }
        },

        "search" => {
          if parts.len() != 3 && parts.len() != 4 {
              eprintln!("Usage: search <name> <dir> [threads]");
              return;
          }

            let target = parts[1];
            let dir = Path::new(parts[2]);

            let threads = if parts.len() == 4 {
                match parts[3].parse::<usize>() {
                    Ok(value) => Some(value),
                    Err(_) => {
                        eprintln!("Threads number must be an integer.");
                        return;
                    }
                }
            } else {
                None
            };

            if let Err(error) = search::search_by_name(target, dir, threads) {
                eprintln!("Error to search file: {}", error);
            }
        },

        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Type 'help' to show available commands");
        }
    }
}

fn show_help() {
    println!("Available Commands:");
    println!("  list <dir>                             List files of directory.");
    println!("  info <file or dir>                     Show information of file or directory.");
    println!("  copy <origin> <dest>                   Copies a file.");
    println!("  move <origin> <dest>                   Rename file");
    println!("  delete <file_or_dir>                   Remove file or directory");
    println!("  cd <dir>                               Changes current directory");
    println!("  tree <dir>                             Show folder tree");
    println!("  search <name> <dir> [threads]          Search files or directories by name");
    println!("  help                                   Show help message.");
    println!("  exit                                   Quits file manager.");
}