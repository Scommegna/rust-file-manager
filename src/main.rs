mod filesystem;

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

        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Type 'help' to show available commands");
        }
    }
}

fn show_help() {
    println!("Available Commands:");
    println!("  list <dir>                    List files of directory.");
    println!("  info <file or dir>            Show information of file or directory.");
    println!("  copy <origem> <destino>       Copia um arquivo");
    println!("  move <origem> <destino>       Move ou renomeia um arquivo");
    println!("  help                          Show help message.");
    println!("  exit                          Quits file manager.");
}