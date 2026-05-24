use rayon::{ThreadPoolBuilder, prelude::*};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn search_by_name(target: &str, dir: &Path, threads: Option<usize>) -> io::Result<()> {
    if !dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Directory not found",
        ));
    }

    if !dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Provided path must be a directory",
        ));
    }

    match threads {
        None => sequential_search_by_name(target, dir),
        Some(1) => sequential_search_by_name(target, dir),
        Some(0) => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Thread number must be greater than 0",
        )),
        Some(thread_count) => parallel_search_by_name(target, dir, thread_count),
    }
}

fn sequential_search_by_name(target: &str, dir: &Path) -> io::Result<()> {
    let mut results = Vec::new();
    search_recursive(target, dir, &mut results);
    results.sort();

    for path in results {
        println!("{}", path.display());
    }

    Ok(())
}

fn search_recursive(target: &str, dir: &Path, results: &mut Vec<PathBuf>) {
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
                results.push(path.clone());
            }
        }

        if path.is_dir() {
            search_recursive(target, &path, results);
        }
    }
}

fn parallel_search_by_name(target: &str, dir: &Path, threads: usize) -> io::Result<()> {
    let pool = ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .map_err(|error| io::Error::new(io::ErrorKind::Other, error.to_string()))?;

    let mut results = pool.install(|| {
        let mut results = Vec::new();
        parallel_search_recursive(target, dir, &mut results);
        results
    });

    results.sort();

    for path in results {
        println!("{}", path.display());
    }

    Ok(())
}

fn parallel_search_recursive(target: &str, dir: &Path, results: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    let paths: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();

    let mut local_results: Vec<PathBuf> = paths
        .par_iter()
        .filter_map(|path| {
            let name = path.file_name()?.to_string_lossy();

            if name.contains(target) {
                Some(path.clone())
            } else {
                None
            }
        })
        .collect();

    results.append(&mut local_results);

    let directories: Vec<PathBuf> = paths.into_iter().filter(|path| path.is_dir()).collect();

    let nested_results: Vec<Vec<PathBuf>> = directories
        .par_iter()
        .map(|dir| {
            let mut sub_results = Vec::new();
            parallel_search_recursive(target, dir, &mut sub_results);
            sub_results
        })
        .collect();

    for mut sub_result in nested_results {
        results.append(&mut sub_result);
    }
}
