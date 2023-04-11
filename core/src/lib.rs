use std::{
    path::PathBuf,
    fs
};

use suzaku_extension_sdk::{
    language::{
        parser::{
            LanguageParserPolicy,
            LanguageParsePolicyInfo,
            LanguageParseResult,
            LanguageParserPolicyError
        },
        data_cleaner::{
            LanguageDataCleanPolicy,
            LanguageDataCleanResult,
            LanguageDataCleanPolicyError
        },
        analyzer::{
            LanguageAnalysisPolicy,
            LanguageAnalysisResult,
            LanguageAnalysisPolicyError
        },
    },
    utils,
};

use suzaku_extension_sdk::{
    METADATA_FOLDER_NAME,
    METADATA_FILE_EXTENSION,
    VERTEX_FOLDER_NAME,
    VERTEX_FILE_EXTENSION
};

mod languages;

pub fn parse(sources: &Vec<PathBuf>, output_dir: &PathBuf, excludes: &Vec<PathBuf>) -> LanguageParseResult<PathBuf> {
    let metadata_dir = output_dir.join(METADATA_FOLDER_NAME);
    if !metadata_dir.exists() {
        if let Err(_) = fs::create_dir_all(&metadata_dir) {
            return Err(LanguageParserPolicyError {});
        }
    }

    fn parsing(index: usize, total: usize, src: &PathBuf, output: &PathBuf) -> LanguageParseResult<PathBuf> {
        print!(" * parsing [{} / {}] '{}' -> ", index, total, src.to_str().unwrap());
        match languages::ExtensionFactory::get_parser_policy("java") {
            Some(parser) => {
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
            },
            None => {
                println!("failed to create analyzer");
                Err(LanguageParserPolicyError {})
            }
        }
    }

    let mut src_files: Vec<PathBuf> = Vec::new();
    for source in sources {
        if source.is_file() {
            src_files.push(source.to_path_buf());
        } else if let Some(exts) = languages::ExtensionFactory::get_parse_policy_info("java").unwrap().get_filename_extensions() {
            for ext in exts {
                let filename_pattern = format!("{}/**/*.{}", source.to_str().unwrap(), ext);
                if let Some(mut files) = utils::list_files(source, filename_pattern.as_str(), &excludes) {
                    src_files.append(&mut files);
                }
            }
        }
    }

    let mut index: usize = 1;
    let total: usize = src_files.len();
    for src_file in src_files {
        if let Err(err) = parsing(index, total, &src_file, &metadata_dir) {
            return Err(err);
        }
        index += 1;
    }
    Ok(metadata_dir)
}

pub fn data_clean(metadatas: &Vec<PathBuf>, output_dir: &PathBuf, excludes: &Vec<PathBuf>) -> LanguageDataCleanResult<PathBuf> {
    let vertex_dir = output_dir.join(VERTEX_FOLDER_NAME);
    if !vertex_dir.exists() {
        if let Err(_) = fs::create_dir_all(&vertex_dir) {
            return Err(LanguageDataCleanPolicyError {});
        }
    }

    fn cleanning(index: usize, total: usize, metadata: &PathBuf, output: &PathBuf) -> LanguageDataCleanResult<PathBuf> {
        print!(" * data cleaning [{} / {}] '{}' -> ", index, total, metadata.to_str().unwrap());
        match languages::ExtensionFactory::get_data_clean_policy("java") {
            Some(mut policy) => {
                match policy.execute(metadata, output) {
                    Ok(output_file_path) => {
                        println!("{}", output_file_path.to_str().unwrap());
                        Ok(output_file_path.to_path_buf())
                    },
                    Err(err) => {
                        println!("failed");
                        Err(err)
                    }
                }
            },
            None => {
                println!("failed to get policy");
                Err(LanguageDataCleanPolicyError {})
            }
        }
    }

    let mut metadata_files: Vec<PathBuf> = Vec::new();
    for metadata in metadatas {
        if metadata.is_file() {
            metadata_files.push(metadata.to_path_buf());
        } else {
            let filename_pattern = format!("{}/**/*.{}", metadata.to_str().unwrap(), METADATA_FILE_EXTENSION);
            if let Some(mut files) = utils::list_files(metadata, filename_pattern.as_str(), excludes) {
                metadata_files.append(&mut files);
            }
        }
    }

    let mut index: usize = 1;
    let total: usize = metadata_files.len();
    for metadata_file in metadata_files {
        if let Err(err) = cleanning(index, total, &metadata_file, &vertex_dir) {
            return Err(err);
        }
        index += 1;
    }
    Ok(vertex_dir)
}

pub fn analysis(vertex_dir: &Vec<PathBuf>, output_dir: &PathBuf, excludes: &Vec<PathBuf>) -> LanguageAnalysisResult<PathBuf> {
    match languages::ExtensionFactory::get_analysis_policy("java") {
        Some(mut analyzer) => analyzer.execute(vertex_dir, output_dir),
        None => Err(LanguageAnalysisPolicyError {})
    }
}
