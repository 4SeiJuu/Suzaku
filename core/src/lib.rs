use std::{
    path::PathBuf,
    error::Error
};

use suzaku_extension_sdk::language::parser::LanguageParserPolicy;

mod languages;

type GenericError = Box<dyn Error + Send + Sync + 'static>;

pub fn parse(src_dir: &PathBuf, output_dir: &PathBuf) {
    fn parsing(analyzer: &impl LanguageParserPolicy, src: &PathBuf, output: &PathBuf) {
        print!(" * parsing '{}' -> ", src.to_str().unwrap());
        if let Ok(output_file_path) = analyzer.execute(src, output) {
            println!("{}", output_file_path);
        } else {
            println!("failed");
        }
    }

    match languages::ExtensionFactory::get_parser_policy("java") {
        Some(analyzer) => {
            if src_dir.is_file() {
                parsing(&analyzer, src_dir, output_dir);
            } else {
                let filename_pattern = format!("{}/**/*.{}", src_dir.to_str().unwrap(), analyzer.get_filename_extension());
                if let Some(files) = list_files(src_dir, filename_pattern.as_str()) {
                    for f in files {
                        parsing(&analyzer, &PathBuf::from(f.as_str()), output_dir);
                    }
                }
            }
        },
        None => println!("failed to create analyzer")
    };
}

fn list_files(src_dir: &PathBuf, filename_pattern: &str) -> Option<Vec<String>> {
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