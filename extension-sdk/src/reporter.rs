use std::path::PathBuf;

#[derive(Debug)]
pub struct ReporterError {}
pub type ReporterResult<T> = std::result::Result<T, ReporterError>;

pub trait Reporter {
    fn new() -> Self;
    fn generate(&self, data: &PathBuf, output: &PathBuf) -> ReporterResult<PathBuf>;
}