use std::{
    path::{PathBuf, Path},
    error::Error
};

use suzaku_extension_sdk::language::parser::LanguageParserPolicy;

mod languages;

type GenericError = Box<dyn Error + Send + Sync + 'static>;

pub fn parse(src_dir: &PathBuf, output_dir: &PathBuf) {
    match languages::AnalyzerFactory::get_analyzer("java9") {
        Some(analyzer) => {
            if src_dir.is_file() {
                print!(" * analysing '{}' ... ", src_dir.to_str().unwrap());
                if let Ok(output_file_path) = analyzer.execute(src_dir, output_dir) {
                    println!("done - Output: {}", output_file_path);
                } else {
                    println!("failed");
                }
            } else {
                let filename_pattern = format!("{}/**/*.{}", src_dir.to_str().unwrap(), analyzer.get_filename_extension());
                if let Some(files) = list_files(src_dir, filename_pattern.as_str()) {
                    for f in files {
                        print!(" * analysing '{}' ... ", f);
                        if let Ok(output_file_path) = analyzer.execute(&Path::new(f.as_str()).to_path_buf(), output_dir) {
                            println!("done - Output: {}", output_file_path);
                        } else {
                            println!("failed");
                        }
                    }
                }
            }
        },
        None => println!("failed to create analyzer")
    };
}

fn list_files<'a>(src_dir: &PathBuf, filename_pattern: &str) -> Option<Vec<String>> {
    if let Some(glob_pattern) = src_dir.clone().join(filename_pattern).to_str() {
        let files: &mut Vec<String> = &mut vec![];
        _ = walk(glob_pattern, &mut |p| {
            if let Some(str) = p.to_str() {
                files.push(String::from(str));
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