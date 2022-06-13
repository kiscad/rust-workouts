use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn visit_dirs(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut result = vec![dir.to_path_buf()];
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let sub_result = visit_dirs(&path)?;
                result.push(path);
                result.extend_from_slice(sub_result.as_slice());
            } else {
                result.push(path);
            }
        }
    }
    Ok(result)
}

pub fn find_files(dir: &Path, types: &Vec<&str>) -> io::Result<Vec<PathBuf>> {
    let mut result = vec![];
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() && !path.is_symlink() {
                let sub_result = find_files(&path, types)?;
                result.extend_from_slice(sub_result.as_slice());
            } else {
                let ext = path.extension();
                match ext {
                    Some(extval) => {
                        let extname = extval.to_str().unwrap();
                        let found_types = types.iter().find(|&&x| x == extname);
                        match found_types {
                            Some(_) => result.push(path),
                            None => continue,
                        }
                    }
                    None => continue,
                }
            }
        }
    }
    Ok(result)
}
