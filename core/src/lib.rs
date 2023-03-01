extern crate antlr_rust;
extern crate serde_json;

mod languages;

pub fn analysis() {
    use languages::Analyzer;

    match languages::AnalyzerFactory::get_analyzer() {
        Some(analyzer) => {
            if let Ok(_) = analyzer.execute("temp/BaseRustTest.java", "./temp") {
            }
        },
        None => println!("failed to create analyzer")
    };
}
