extern crate antlr_rust;
extern crate serde_json;

mod languages;

use std::{fs::File, io::Write};

pub fn analysis() {
    use languages::Analyzer;

    match languages::AnalyzerFactory::get_analyzer() {
        Some(analyzer) => {
            if let Ok(json_str) = analyzer.analysis("temp/BaseRustTest.java") {
                if let Ok(mut f) = File::create("./temp/output.json") {
                    let _ = f.write_all(json_str.as_bytes());
                    let _ = f.flush();
                }
            }
        },
        None => println!("failed to create analyzer")
    };
}
