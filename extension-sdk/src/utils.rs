use std::{path::PathBuf, error::Error};

type GenericError = Box<dyn Error + Send + Sync + 'static>;
type FileSystemWalkResult = Result<(), GenericError>;

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