extern crate antlr_rust;
extern crate serde_json;
extern crate glob;

use std::{
    path::{PathBuf, Path},
    error::Error
};

mod languages;

type GenericError = Box<dyn Error + Send + Sync + 'static>;

pub fn analysis(src_dir: &str, output_dir: &str) {
    use languages::Analyzer;

    match languages::AnalyzerFactory::get_analyzer() {
        Some(analyzer) => {
            let filename_pattern = format!("*.{}", analyzer.get_src_file_extension());
            if let Some(files) = list_files(src_dir, filename_pattern.as_str()) {
                for f in files {
                    if let Ok(_) = analyzer.execute(f.as_str(), output_dir) {
                    }
                }
            }
        },
        None => println!("failed to create analyzer")
    };
}

fn list_files(src_dir: &str, filename_pattern: &str) -> Option<Vec<String>> {
    if let Some(glob_pattern) = Path::new(src_dir).join(filename_pattern).to_str() {
        let files: &mut Vec<String> = &mut vec![];
        _ = walk(glob_pattern, &mut |p| {
            if let Some(str) = p.to_str() {
                files.push(String::from(str))
            } 
        });
        return Some(files.clone());
    }
    None
}

fn walk(pattern: &str, cb: &mut dyn for<'a> FnMut(&'a PathBuf)) -> Result<(), GenericError> {
    for entry in glob::glob(pattern)? {
        cb(&entry?);
    }
    Ok(())
}