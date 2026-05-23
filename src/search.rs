use std::fs;
use std::io;
use std::path::Path;

pub fn search_by_name(target: &str, dir: &Path) -> io::Result<()> {
    if !dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Directory not found"
        ))
    }

    if !dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Provided path must be a directory"
        ))
    }

    search_recursive(target, dir);

    Ok(())
}

fn search_recursive(target: &str, dir: &Path) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => {
            return;
        }
    };

    for entry_result in entries {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(_) => {
                continue;
            }
        };
        let path = entry.path();

        if let Some(name) = path.file_name() {
            let name = name.to_string_lossy();

            if name.contains(target) {
                println!("{}", path.display());
            }
        }

        if path.is_dir() {
            search_recursive(target, &path);
        }
    }
}