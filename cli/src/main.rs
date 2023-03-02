extern crate core;
extern crate clap;

use std::path::{
    Path, 
    PathBuf
};
use std::fs;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Analysis {
        // Sets source code folder path which will be analyzed
        #[arg(short, long, value_name = "SRC", default_value_t = String::from("."))]
        src: String,
    
        // Set output folder path which will store the output files
        #[arg(short, long, value_name = "OUTPUT", default_value_t = String::from("."))]
        output: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Analysis { src, output }) => {
            if let Ok(src_dir) = fs::canonicalize(Path::new(src)) {
                if let Ok(output_dir) = fs::canonicalize(Path::new(output)) {
                    core::analysis(&src_dir, &output_dir);
                }
            }
            // core::analysis(fs::canonicalize(Path::new(src)) PathBuf::from(src), &PathBuf::from(output));
        }
        None => {}
    }
}
