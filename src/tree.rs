use std::fs;
use std::io;
use std::path::Path;

pub fn print_tree(path: &Path) -> io::Result<()> {
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Path not found"
        ))
    }

    println!("{}", path.display());
    print_tree_recursive(path, "", true)?;

    Ok(())
}

fn print_tree_recursive(path: &Path, prefix: &str, _is_last: bool) -> io::Result<()> {
    if !path.is_dir() {
        return Ok(())
    }

    let entries = fs::read_dir(path)?;

    let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();

    entries.sort_by_key(|entry| entry.path());

    let total = entries.len();

    for(index, entry) in entries.iter().enumerate() {
        let path = entry.path();
        let is_last = index == total - 1;

        let connector = if is_last { "└── " } else { "├── " };

        let name = path.file_name().map(|name| name.to_string_lossy()).unwrap_or_default();

        println!("{}{}{}", prefix, connector, name);

        if path.is_dir() {
            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            print_tree_recursive(&path, &new_prefix, is_last)?;
        }
    }

    Ok(())
}