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

pub fn copy_file(src: &Path, dst: &Path) -> io::Result<()> {
    if !src.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File not found."
        ))
    }

    if !src.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Origin must be a file."
        ))
    }

    fs::copy(src, dst)?;

    Ok(())
}

pub fn move_file(src: &Path, dst: &Path) -> io::Result<()> {
    if ! src.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Origin file not found",
        ))
    }

    fs::rename(src, dst)?;

    Ok(())
}

pub fn delete_path(path: &Path) -> io::Result<()> {
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Path not found"
        ));
    }

    if path.is_file() {
        fs::remove_file(path)?;
    } else if path.is_dir() {
        fs::remove_dir_all(path)?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Path type not supported."
        ))
    }

    Ok(())
}