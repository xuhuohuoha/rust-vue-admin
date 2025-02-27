//! File Tools

use std::fs;
use std::path::Path;

/// 获取指定路径下的所有文件
pub fn get_files(dir_path: &str) -> Vec<String> {
    let dir = Path::new(dir_path);
    let mut files = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                files.push(path.to_str().unwrap().to_string());
            }
        }
    }

    files
}

/// 获取指定路径下的所有指定类型文件
pub fn get_files_with_extension(dir_path: &str, extension: &str) -> Vec<String> {
    let dir = Path::new(dir_path);
    let mut files = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() && path.extension().unwrap().to_str().unwrap() == extension {
                files.push(path.to_str().unwrap().to_string());
            }
        }
    }

    files
}

/// 获取指定路径下的所有文件目录
pub fn get_directories(dir_path: &str) -> Vec<String> {
    let dir = Path::new(dir_path);
    let mut directories = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                directories.push(path.to_str().unwrap().to_string());
            }
        }
    }

    directories
}

/// 获取指定文件类型
pub fn get_extension(file_path: &str) -> String {
    let path = Path::new(file_path);
    path.extension().unwrap().to_str().unwrap().to_string()
}
