use std::{
    path::PathBuf,
    error::Error, fs
};

use suzaku_extension_sdk::language::{
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

pub fn parse(src_dir: &PathBuf, output_dir: &PathBuf, excludes: Vec<PathBuf>) -> LanguageParseResult<PathBuf> {
    let metadata_dir = output_dir.join(PARSED_RESULT_FOLDER_NAME);
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

    if src_dir.is_file() {
        if let Err(err) = parsing(1, 1, src_dir, &metadata_dir) {
            return Err(err);
        }
    } else if let Some(exts) = languages::ExtensionFactory::get_parse_policy_info("java").unwrap().get_filename_extensions() {
        for ext in exts {
            let filename_pattern = format!("{}/**/*.{}", src_dir.to_str().unwrap(), ext);
            if let Some(files) = list_files(src_dir, filename_pattern.as_str(), &excludes) {
                let mut index: usize = 1;
                let total: usize = files.len();
                for f in files {
                    if let Err(err) = parsing(index, total, &PathBuf::from(f.as_str()), &metadata_dir) {
                        return Err(err);
                    }
                    index += 1;
                }
            }
        }
    }
    Ok(metadata_dir)
}

pub fn data_clean(metadata_dir: &PathBuf, output_dir: &PathBuf) -> LanguageDataCleanResult<PathBuf> {
    let vertex_dir = output_dir.join(ANALYZED_RESULTS_FOLDER_NAME);
    if !metadata_dir.exists() {
        if let Err(_) = fs::create_dir_all(&metadata_dir) {
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

    if metadata_dir.is_file() {
        if let Err(err) = cleanning(1, 1, metadata_dir, &vertex_dir) {
            return Err(err);
        }
    } else {
        let filename_pattern = format!("{}/**/*.{}", metadata_dir.to_str().unwrap(), METADATA_FILE_EXTENSION);
        if let Some(files) = list_files(metadata_dir, filename_pattern.as_str(), &Vec::new()) {
            let mut index: usize = 1;
            let total: usize = files.len();
            for f in files {
                if let Err(err) = cleanning(index, total, &PathBuf::from(f.as_str()), &vertex_dir) {
                    return Err(err);
                }
                index += 1;
            }
        }
    }

    Ok(vertex_dir)
}

fn list_files(src_dir: &PathBuf, filename_pattern: &str, excludes: &Vec<PathBuf>) -> Option<Vec<String>> {
    if let Some(glob_pattern) = src_dir.clone().join(filename_pattern).to_str() {
        let files: &mut Vec<String> = &mut vec![];
        _ = walk(glob_pattern, &mut |p| {
            for exclude in excludes {
                if p.starts_with(exclude) || p.eq(exclude) {
                    return
                }
            }
            
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