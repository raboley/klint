//! File and directory processing module

use anyhow::{Context, Result};
use glob::glob;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info};
use walkdir::WalkDir;

/// Process files based on the given path (file, directory, or glob pattern)
/// 
/// # Arguments
/// 
/// * `path` - The path to process (can be a file, directory, or glob pattern)
/// * `recursive` - Whether to process directories recursively
/// 
/// # Returns
/// 
/// A vector of PathBuf objects for all KQL files found
pub fn process_path(path: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
    if path.is_file() {
        if is_kql_file(path) {
            Ok(vec![path.to_path_buf()])
        } else {
            Ok(vec![])
        }
    } else if path.is_dir() {
        process_directory(path, recursive)
    } else {
        // Try as glob pattern
        process_glob(path.to_str().unwrap_or(""))
    }
}

/// Check if a file is a KQL file based on its extension
/// 
/// # Arguments
/// 
/// * `path` - The file path to check
/// 
/// # Returns
/// 
/// True if the file has a KQL-related extension (.kql, .kusto, .csl)
pub fn is_kql_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| matches!(ext.to_lowercase().as_str(), "kql" | "kusto" | "csl"))
        .unwrap_or(false)
}

/// Process a directory
fn process_directory(dir: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    if recursive {
        walk_directory(dir, &mut files)?;
    } else {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && is_kql_file(&path) {
                files.push(path);
            }
        }
    }
    
    info!("Found {} KQL files in {:?}", files.len(), dir);
    Ok(files)
}

/// Process a glob pattern
fn process_glob(pattern: &str) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    for entry in glob(pattern).context("Failed to parse glob pattern")? {
        match entry {
            Ok(path) if is_kql_file(&path) => {
                debug!("Matched file: {:?}", path);
                files.push(path);
            }
            Ok(_) => {} // Skip non-KQL files
            Err(e) => {
                debug!("Error processing glob entry: {}", e);
            }
        }
    }
    
    info!("Found {} KQL files matching pattern: {}", files.len(), pattern);
    Ok(files)
}

/// Walk a directory recursively and collect KQL files
fn walk_directory(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && is_kql_file(path) {
            debug!("Found KQL file: {:?}", path);
            files.push(path.to_path_buf());
        }
    }
    
    Ok(())
}

/// Read the contents of a file as a string
/// 
/// # Arguments
/// 
/// * `path` - The path to the file to read
/// 
/// # Returns
/// 
/// The file contents as a string, or an error if reading fails
pub fn read_file(path: &Path) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {:?}", path))
}

/// Write content to a file
/// 
/// # Arguments
/// 
/// * `path` - The path where the file should be written
/// * `content` - The content to write to the file
/// 
/// # Returns
/// 
/// Ok(()) if successful, or an error if writing fails
pub fn write_file(path: &Path, content: &str) -> Result<()> {
    fs::write(path, content)
        .with_context(|| format!("Failed to write file: {:?}", path))
}