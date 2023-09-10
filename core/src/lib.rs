use std::{
    path::PathBuf,
    fs::{
        self, 
        File
    }, 
    collections::HashMap, 
    io::Write
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
    utils::collect_input_files,
    METADATA_FOLDER_NAME,
    METADATA_FILE_EXTENSION,
    ELEMENT_FOLDER_NAME, 
    ELEMENT_FILE_EXTENSION,
};

mod factory;

pub fn parse(sources: &Vec<PathBuf>, output_dir: &PathBuf, excludes: &Vec<PathBuf>) -> LanguageParseResult<PathBuf> {
    let metadata_dir = output_dir.join(METADATA_FOLDER_NAME);
    if !metadata_dir.exists() {
        if let Err(_) = fs::create_dir_all(&metadata_dir) {
            return Err(LanguageParserPolicyError {});
        }
    }

    fn parsing(index: usize, total: usize, folder: &PathBuf, relative_file_path: &PathBuf, output: &PathBuf) -> LanguageParseResult<PathBuf> {
        print!(" * parsing [{} / {}] '{}' -> ", index, total, PathBuf::from_iter(vec![folder, relative_file_path]).to_str().unwrap());
        match factory::ExtensionFactory::get_parser_policy("java") {
            Some(parser) => {
                match parser.execute(folder, relative_file_path) {
                    Ok(data) => {
                        let output_file_path = output.join(format!("{}.{}", relative_file_path.file_stem().unwrap().to_str().unwrap(), METADATA_FILE_EXTENSION));
                        if let Ok(mut f) = File::create(&output_file_path) {
                            let _ = f.write_all(data.as_bytes());
                            let _ = f.flush();
                        }
                        
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

    print!("# collecting source files ... ");
    let mut src_files: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
    let mut total: usize = 0;
    if let Some(exts) = factory::ExtensionFactory::get_parse_policy_info("java").unwrap().get_filename_extensions() {
        for ext in exts {
            let filename_pattern = format!("**/*.{}", ext);
            if let Some(mut collected) = collect_input_files(sources, &filename_pattern, &excludes) {
                for (key, values) in &mut collected {
                    if let Some(fl) = src_files.get_mut(key) {
                        fl.append(values);
                    } else {
                        src_files.insert(key.to_path_buf(), Vec::from(values.clone()));
                    }
                    total += values.len();
                }
            }
        }
    }
    println!("done");
    println!("----------------------------------");
    println!(" total: {} files found", total);
    println!("----------------------------------\n");

    println!("# start parsing ...");
    let mut index: usize = 1;
    for (folder, files) in src_files {
        for f in files {
            let output_folder = PathBuf::from_iter(vec![metadata_dir.clone(), f.parent().unwrap().to_path_buf()]);
            if !&output_folder.exists() {
                if let Err(_) = fs::create_dir_all(output_folder.clone()) {
                    return Err(LanguageParserPolicyError {});
                }
            }
            if let Err(err) = parsing(index, total, &folder, &f, &output_folder) {
                return Err(err)
            }
            index += 1;
        }
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
        print!(" * extracting [{} / {}] '{}' -> ", index, total, metadata.to_str().unwrap());
        match factory::ExtensionFactory::get_data_extractor_policy("java") {
            Some(mut policy) => {
                match policy.execute(metadata) {
                    Ok(data) => {
                        let output_file_path = output.join(format!("{}.{}", metadata.file_stem().unwrap().to_str().unwrap(), ELEMENT_FILE_EXTENSION));
                        if let Ok(mut f) = File::create(&output_file_path) {
                            let _ = f.write_all(data.as_bytes());
                            let _ = f.flush();
                        }
                        
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
                println!("failed to get policy");
                Err(LanguageDataExtractorPolicyError {})
            }
        }
    }

    // collect metadata files
    print!("# collecting source files ... ");
    let mut metadata_files: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
    let mut total: usize = 0;
    let filename_pattern = format!("**/*.{}", METADATA_FILE_EXTENSION);
    if let Some(mut collected) = collect_input_files(metadatas, &filename_pattern, &excludes) {
        for (key, values) in &mut collected {
            if let Some(fl) = metadata_files.get_mut(key) {
                fl.append(values);
            } else {
                metadata_files.insert(key.to_path_buf(), Vec::from(values.clone()));
            }
            total += values.len();
        }
    }
    println!("done");
    println!("----------------------------------");
    println!(" total: {} files found", total);
    println!("----------------------------------\n");

    // cleaning each metadata file
    let mut index: usize = 1;
    for (folder, files) in metadata_files {
        for f in files {
            let target_file = PathBuf::from_iter(vec![folder.clone(), f.clone()]);
            let output_folder = PathBuf::from_iter(vec![elements_dir.clone(), f.parent().unwrap().to_path_buf()]);
            if !&output_folder.exists() {
                if let Err(_) = fs::create_dir_all(output_folder.clone()) {
                    return Err(LanguageDataExtractorPolicyError {});
                }
            }
            if let Err(err) = extracting(index, total, &target_file, &output_folder) {
                return Err(err);
            }
            index += 1;
        }
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
