use std::{
    path::PathBuf, 
    fs::{
        self, 
        File
    }, 
    io::Write
};

use suzaku_extension_sdk::{
    reporter::{
        Reporter, 
        ReporterResult, 
        ReporterError
    }, 
    GRAPH_FILE_EXTENSION
};

use super::super::data::graphviz::GraphData;

pub struct GraphvizReporter;

impl Reporter for GraphvizReporter {
    fn new() -> Self {
        GraphvizReporter {}
    }

    fn generate(&self, data_file: &PathBuf, output: &PathBuf) -> ReporterResult<PathBuf> {
        if data_file.is_file() {
            let context_str = fs::read_to_string(data_file).expect("should read context of file");

            return match serde_json::from_str::<GraphData>(&context_str) {
                Ok(data) => {
                    let mut graph_contents = vec![
                        String::from("digraph A {"),
                        String::from("node [shape=plaintext fontname=\"Sans serif\" fontsize=\"8\"];"),
                    ];
    
    
                    for edge in data.depends.values() {
                        graph_contents.push(edge.to_graphviz_edge());
                    }
    
                    for vertex in data.elements.values() {
                        graph_contents.push(vertex.to_graphviz_node());
                    }
    
                    graph_contents.push(String::from("}"));
    
                    // save graphviz file
                    let dot_file_path = output.join(format!("{}.{}", "graphviz", GRAPH_FILE_EXTENSION));
                    if let Ok(mut f) = File::create(&dot_file_path) {
                        let _ = f.write_all(graph_contents.join("\n").as_bytes());
                        let _ = f.flush();
                    }

                    Ok(dot_file_path)
                },
                Err(err) => Err(ReporterError {})
            };
        }

        Err(ReporterError {})
    }
}
