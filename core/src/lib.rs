use std::{
    path::PathBuf,
    fs
};

use suzaku_extension_sdk::{
    parser::{
        LanguageParserPolicy,
        LanguageParsePolicyInfo,
        LanguageParseResult,
        LanguageParserPolicyError
    },
    extractor::{
        LanguageDataExtractorPolicy,
        LanguageDataExtractorResult,
        LanguageDataExtractorPolicyError
    },
    analyzer::{
        LanguageAnalysisPolicy,
        LanguageAnalysisResult,
        LanguageAnalysisPolicyError
    }, 
    mapper::{
        LanguageDataMapperPolicy, 
        LanguageDataMapperResult, 
        LanguageDataMapperPolicyError
    }, 
    reporter::{
        Reporter, 
        ReporterError, 
        ReporterResult
    },
    utils, 
    METADATA_FOLDER_NAME,
    METADATA_FILE_EXTENSION,
    ELEMENT_FOLDER_NAME,
};

mod factory;

pub fn parse(sources: &Vec<PathBuf>, output_dir: &PathBuf, excludes: &Vec<PathBuf>) -> LanguageParseResult<PathBuf> {
    let metadata_dir = output_dir.join(METADATA_FOLDER_NAME);
    if !metadata_dir.exists() {
        if let Err(_) = fs::create_dir_all(&metadata_dir) {
            return Err(LanguageParserPolicyError {});
        }
    }

    fn parsing(index: usize, total: usize, src: &PathBuf, output: &PathBuf) -> LanguageParseResult<PathBuf> {
        print!(" * parsing [{} / {}] '{}' -> ", index, total, src.to_str().unwrap());
        match factory::ExtensionFactory::get_parser_policy("java") {
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
        } else if let Some(exts) = factory::ExtensionFactory::get_parse_policy_info("java").unwrap().get_filename_extensions() {
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

pub fn extract(metadatas: &Vec<PathBuf>, output_dir: &PathBuf, excludes: &Vec<PathBuf>) -> LanguageDataExtractorResult<PathBuf> {
    let elements_dir = output_dir.join(ELEMENT_FOLDER_NAME);
    if !elements_dir.exists() {
        if let Err(_) = fs::create_dir_all(&elements_dir) {
            return Err(LanguageDataExtractorPolicyError {});
        }
    }

    fn extracting(index: usize, total: usize, metadata: &PathBuf, output: &PathBuf) -> LanguageDataExtractorResult<PathBuf> {
        print!(" * data cleaning [{} / {}] '{}' -> ", index, total, metadata.to_str().unwrap());
        match factory::ExtensionFactory::get_data_extractor_policy("java") {
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
                Err(LanguageDataExtractorPolicyError {})
            }
        }
    }

    // collect metadata files
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

    // cleaning each metadata file
    let mut index: usize = 1;
    let total: usize = metadata_files.len();
    for metadata_file in metadata_files {
        if let Err(err) = extracting(index, total, &metadata_file, &elements_dir) {
            return Err(err);
        }
        index += 1;
    }

    Ok(output_dir.clone())
}

pub fn map(elements: &Vec<PathBuf>, output: &PathBuf, excludes: &Vec<PathBuf>) -> LanguageDataMapperResult<PathBuf> {
    print!(" * data mapping ... ");
    match factory::ExtensionFactory::get_data_mapper_policy("java") {
        Some(mut policy) => {
            match policy.execute(elements, output) {
                Ok(elements_file_path) => Ok(elements_file_path),
                Err(err) => Err(LanguageDataMapperPolicyError{})
            }
        },
        None => Err(LanguageDataMapperPolicyError {})
    }
}

pub fn analysis(elements_file: &PathBuf, output_dir: &PathBuf) -> LanguageAnalysisResult<PathBuf> {
    match factory::ExtensionFactory::get_analyzer_policy("java") {
        Some(mut analyzer) => analyzer.execute(elements_file, output_dir),
        None => Err(LanguageAnalysisPolicyError {})
    }
}

pub fn report(data: &PathBuf, output_dir: &PathBuf) -> ReporterResult<PathBuf> {
    match factory::ExtensionFactory::get_reporter("") {
        Some(reporter) => reporter.generate(data, output_dir),
        None => Err(ReporterError {})
    }
}

pub fn pipeline(config: &PathBuf, output_dir: &PathBuf) {

}