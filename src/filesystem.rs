use std::fs;
use std::io;
use std::path::Path;

pub fn list_dir(dir: &Path) -> io::Result<()> {
    let entries = fs::read_dir(dir)?;
    
    for entry_result in entries {
        let entry = entry_result?;
        let path = entry.path();
        
        if let Some(name) = path.file_name() {
            println!("{}", name.to_string_lossy());
        }
    }
    
    Ok(())
}

pub fn show_info(path: &Path) -> io::Result<()> {
    let metadata = fs::metadata(path)?;
    
    println!("Path: {}", path.display());
    println!("Size: {} bytes", metadata.len());
    println!("Is file: {}", metadata.is_file());
    println!("Is directory: {}", metadata.is_dir());
    println!("Readonly: {}", metadata.permissions().readonly());
    
    Ok(())
}