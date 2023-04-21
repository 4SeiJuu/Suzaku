use std::{
    path::PathBuf, 
    error::Error, 
    collections::HashMap
};

type GenericError = Box<dyn Error + Send + Sync + 'static>;
type FileSystemWalkResult = Result<(), GenericError>;


pub fn collect_input_files(folders: &Vec<PathBuf>, pattern: &str, excludes: &Vec<PathBuf>) -> Option<HashMap<PathBuf, Vec<PathBuf>>> {
    let mut files: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
    for folder in folders {
        if folder.is_file() {
            files.insert(folder.parent().unwrap().to_path_buf(), vec![PathBuf::from(folder.file_name().unwrap())]);
        } else {
            let full_pattern = format!("{}/{}", folder.to_str().unwrap(), pattern);
            if let Some(found) = list_files(folder, full_pattern.as_str(), &excludes) {
                for f in found {
                    if let Ok(related_file_path) = f.strip_prefix(folder) {
                        if let Some(files_in_folder) = files.get_mut(folder) {
                            files_in_folder.push(related_file_path.to_path_buf());
                        } else {
                            files.insert(folder.to_path_buf(), vec![related_file_path.to_path_buf()]);
                        }
                    } else {
                        panic!("[ERROR] file path not started with folder path. File: {}, Folder: {}", f.to_str().unwrap(), folder.to_str().unwrap());
                    }
                }
            }
        }
    }
    Some(files)
}

pub fn list_files(src_dir: &PathBuf, filename_pattern: &str, excludes: &Vec<PathBuf>) -> Option<Vec<PathBuf>> {
    if let Some(glob_pattern) = src_dir.clone().join(filename_pattern).to_str() {
        let mut files: Vec<PathBuf> = Vec::new();
        _ = walk(glob_pattern, &mut |p| {
            for exclude in excludes {
                if p.starts_with(exclude) || p.eq(exclude) {
                    return
                }
            }
            
            if let Some(str) = p.to_str() {
                files.push(PathBuf::from(str));
            }
        });
        return Some(files);
    }
    None
}

pub fn walk(pattern: &str, cb: &mut dyn for<'a> FnMut(&'a PathBuf)) -> FileSystemWalkResult {
    for entry in glob::glob(pattern)? {
        cb(&entry?);
    }
    Ok(())
}

pub fn vec_join<T>(v: &Vec<T>, sep: &str) -> Option<String>
where T: ToString {
    if v.is_empty() {
        return None
    }
    
    match v.len() {
        1 => match v.get(0) {
            Some(s) => Some(s.to_string()),
            None => None
        },
        _ => {
            let mut t: Vec<String> = Vec::new();
            for i in v {
                t.push(i.to_string());
            }
            Some(t.join(sep))
        }
    }
}