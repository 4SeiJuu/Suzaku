use std::{
    path::PathBuf,
    error::Error, fs
};

use suzaku_extension_sdk::language::{
    parser::{
        LanguageParserPolicy,
        LanguageParseResult,
        LanguageParserPolicyError
    },
    data_cleaner::{
        LanguageDataCleanPolicy,
        LanguageDataCleanResult,
        LanguageDataCleanPolicyError
    }
};

use suzaku_extension_sdk::{
    PARSED_RESULT_FOLDER_NAME,
    METADATA_FILE_EXTENSION,
    ANALYZED_RESULTS_FOLDER_NAME,
    VERTEX_FILE_EXTENSION
};

mod languages;

type GenericError = Box<dyn Error + Send + Sync + 'static>;
type FileSystemWalkResult = Result<(), GenericError>;

pub fn parse(src_dir: &PathBuf, output_dir: &PathBuf) -> LanguageParseResult<PathBuf> {
    let metadata_dir = output_dir.join(PARSED_RESULT_FOLDER_NAME);
    if !metadata_dir.exists() {
        if let Err(_) = fs::create_dir_all(&metadata_dir) {
            return Err(LanguageParserPolicyError {});
        }
    }

    fn parsing(parser: &impl LanguageParserPolicy, src: &PathBuf, output: &PathBuf) -> LanguageParseResult<PathBuf> {
        print!(" * parsing '{}' -> ", src.to_str().unwrap());
        match parser.execute(src, output) {
            Ok(output_file_path) => {
                println!("{}", output_file_path.to_str().unwrap());
                Ok(output_file_path)
            },
            Err(err) => {
                println!("failed");
                Err(err)
            }
        }
    }

    match languages::ExtensionFactory::get_parser_policy("java") {
        Some(parser) => {
            if src_dir.is_file() {
                if let Err(err) = parsing(&parser, src_dir, output_dir) {
                    return Err(err);
                }
            } else if let Some(exts) = parser.get_filename_extensions() {
                for ext in exts {
                    let filename_pattern = format!("{}/**/*.{}", src_dir.to_str().unwrap(), ext);
                    if let Some(files) = list_files(src_dir, filename_pattern.as_str()) {
                        for f in files {
                            if let Err(err) = parsing(&parser, &PathBuf::from(f.as_str()), &metadata_dir) {
                                return Err(err);
                            }
                        }
                    }
                }
            }
            Ok(metadata_dir)
        },
        None => {
            println!("failed to create analyzer");
            Err(LanguageParserPolicyError {})
        }
    }
}

pub fn analysis(metadata_dir: &PathBuf, output_dir: &PathBuf) -> LanguageDataCleanResult<PathBuf> {
    let vertex_dir = output_dir.join(ANALYZED_RESULTS_FOLDER_NAME);
    if !metadata_dir.exists() {
        if let Err(_) = fs::create_dir_all(&metadata_dir) {
            return Err(LanguageDataCleanPolicyError {});
        }
    }

    fn analysing(analyzer: &mut impl LanguageDataCleanPolicy, metadata: &PathBuf, output: &PathBuf) -> LanguageDataCleanResult<PathBuf> {
        print!(" * analysing '{}' -> ", metadata.to_str().unwrap());
        match analyzer.execute(metadata, output) {
            Ok(output_file_path) => {
                println!("{}", output_file_path.to_str().unwrap());
                Ok(output_file_path.to_path_buf())
            },
            Err(err) => {
                println!("failed");
                Err(err)
            }
        }
    }

    match languages::ExtensionFactory::get_analysis_policy("java") {
        Some(mut analyzer) => {
            if metadata_dir.is_file() {
                if let Err(err) = analysing(&mut analyzer, metadata_dir, &vertex_dir) {
                    return Err(err);
                }
            } else {
                let filename_pattern = format!("{}/**/*.{}", metadata_dir.to_str().unwrap(), METADATA_FILE_EXTENSION);
                if let Some(files) = list_files(metadata_dir, filename_pattern.as_str()) {
                    for f in files {
                        if let Err(err) = analysing(&mut analyzer, &PathBuf::from(f.as_str()), &vertex_dir) {
                            return Err(err);
                        }
                    }
                }
            }
            Ok(vertex_dir)
        },
        None => {
            println!("failed to create analyzer");
            Err(LanguageDataCleanPolicyError {})
        }
    }
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

fn walk(pattern: &str, cb: &mut dyn for<'a> FnMut(&'a PathBuf)) -> FileSystemWalkResult {
    for entry in glob::glob(pattern)? {
        cb(&entry?);
    }
    Ok(())
}