use std::{path::PathBuf, fs::{self, File}, io::Write, collections::HashMap};

use serde::Deserialize;
use serde_json::Value;

use suzaku_extension_sdk::{
    language::{analyzer::{
        LanguageAnalysisPolicy,
        LanguageAnalysisResult,
        LanguageAnalysisPolicyError
    }, ivertex::{VertexType, VertexCategories, IVertex, ToSignature}},
    utils, GRAPH_FILE_EXTENSION
};

use suzaku_extension_sdk::{
    VERTEX_FILE_EXTENSION
};

use super::vertex::JavaVertex;

enum JavaAnalysisedResultCategory {
    Depends,
    Defines,
    CallOuts
}

#[derive(Debug)]
struct GraphNode {
    package: Option<VertexType>,
    ty: Option<VertexType>,
    defines: Vec<VertexType>,
    callouts: Vec<VertexType>
}

impl GraphNode {
    pub fn get_name(&self) -> String {
        self.ty.as_ref().unwrap().to_string()
    }

    pub fn get_signature(&self) -> String {
        self.ty.as_ref().unwrap().to_signature()
    }

    pub fn to_graphviz_node(&self) -> String {
        let node_name = self.get_signature();
        let label = self.get_name();

        let mut rows: Vec<String> = Vec::new();
        // for defination in &self.defines {
        //     rows.push(format!("<tr><td align='left'>\"{}\"</td></tr>", defination.to_string()));
        // }

        format!("{} [label=<\
        <table border='1' cellborder='0' cellspacing='1'>\
            <tr><td align='center'><b>{}</b></td></tr>\
            {}\
        </table>>];", node_name, label, rows.join(""))
    }
}

pub struct JavaAnalyzer {
    graph_nodes: HashMap<String, GraphNode>,
    depends: Vec<VertexType>,
}

impl JavaAnalyzer {
    pub fn analysis(&mut self, vertexes: &HashMap<VertexCategories, Vec<JavaVertex>>) -> LanguageAnalysisResult<()> {
        let package = match vertexes.get(&VertexCategories::Package) {
            Some(jvs) => match jvs.get(0) {
                Some(jv) => jv.get_type(),
                None => None
            },
            None => None
        };

        if let Some(jvs) = vertexes.get(&VertexCategories::Classes) {
            for jv in jvs {
                let mut graph_node = GraphNode {
                    package: match package {
                        Some(ty) => Some(ty.clone()),
                        None => None
                    },
                    ty: match jv.get_type() {
                        Some(t) => Some(t.clone()),
                        None => None
                    },
                    defines: Vec::new(),
                    callouts: Vec::new()
                };

                // definations
                if let Some(members) = jv.get_member_by_category(VertexCategories::Constructors) {
                    self.apply(&mut graph_node, JavaAnalysisedResultCategory::Defines, members);
                }

                if let Some(members) = jv.get_member_by_category(VertexCategories::Fields) {
                    self.apply(&mut graph_node, JavaAnalysisedResultCategory::Defines, members);
                }

                if let Some(members) = jv.get_member_by_category(VertexCategories::Methods) {
                    self.apply(&mut graph_node, JavaAnalysisedResultCategory::Defines, members);
                }

                // call outs
                if let Some(members) = jv.get_member_by_category(VertexCategories::CreatorCalls) {
                    self.apply(&mut graph_node, JavaAnalysisedResultCategory::CallOuts, members);
                }

                if let Some(members) = jv.get_member_by_category(VertexCategories::MethodCalls) {
                    self.apply(&mut graph_node, JavaAnalysisedResultCategory::CallOuts, members);
                }

                self.graph_nodes.insert(graph_node.get_signature(), graph_node);
            }
        }

        if let Some(jvs) = vertexes.get(&VertexCategories::Interfaces) {
            for jv in jvs {
                let mut graph_node = GraphNode {
                    package: match package {
                        Some(ty) => Some(ty.clone()),
                        None => None
                    },
                    ty: match jv.get_type() {
                        Some(t) => Some(t.clone()),
                        None => None
                    },
                    defines: Vec::new(),
                    callouts: Vec::new()
                };

                // definations
                if let Some(members) = jv.get_member_by_category(VertexCategories::Methods) {
                    self.apply(&mut graph_node, JavaAnalysisedResultCategory::Defines, members);
                }

                // call outs
                if let Some(members) = jv.get_member_by_category(VertexCategories::CreatorCalls) {
                    self.apply(&mut graph_node, JavaAnalysisedResultCategory::CallOuts, members);
                }

                if let Some(members) = jv.get_member_by_category(VertexCategories::MethodCalls) {
                    self.apply(&mut graph_node, JavaAnalysisedResultCategory::CallOuts, members);
                }

                self.graph_nodes.insert(graph_node.get_signature(), graph_node);
            }
        }
        Ok(())
    }

    fn apply(&self, node: &mut GraphNode, category: JavaAnalysisedResultCategory, vertexes: &Vec<Box<JavaVertex>>) {
        for jv in vertexes {
            if let Some(v) = jv.get_type() {
                match category {
                    JavaAnalysisedResultCategory::Defines => node.defines.push(v.clone()),
                    JavaAnalysisedResultCategory::CallOuts => node.callouts.push(v.clone()),
                    _ => ()
                }
            }
        }
    }

    pub fn load_vertex_from_file(&self, path: &PathBuf) -> LanguageAnalysisResult<HashMap<VertexCategories, Vec<JavaVertex>>> {
        let context_str = fs::read_to_string(path).expect("should read context of file");
        
        let mut deserializer = serde_json::Deserializer::from_str(&context_str);
        deserializer.disable_recursion_limit();
        let deserializer = serde_stacker::Deserializer::new(&mut deserializer);

        match Value::deserialize(deserializer) {
            Ok(v) => match serde_json::from_value(v) {
                Ok(contents) => Ok(contents),
                Err(e) => Err(LanguageAnalysisPolicyError {})
            },
            Err(e) => Err(LanguageAnalysisPolicyError {})
        }
    }

    pub fn generate_graphviz_dot_file(&self, file_path: &PathBuf) -> LanguageAnalysisResult<()> {
        let mut lines: Vec<String> = vec!["digraph A {".to_string()];
        lines.push("  node [shape=plaintext fontname=\"Sans serif\" fontsize=\"8\"];".to_string());

        // add nodes
        for (_name, node) in &self.graph_nodes {
            lines.push(format!("  {}", node.to_graphviz_node()));
        }

        // add relationships
        for ty in &self.depends {

        }
        lines.push("}".to_string());

        if let Ok(mut f) = File::create(&file_path) {
            let _ = f.write_all(lines.join("\n").as_bytes());
            let _ = f.flush();
        }
        Ok(())
    }
}

impl LanguageAnalysisPolicy for JavaAnalyzer {
    fn new() -> Self {
        JavaAnalyzer {
            graph_nodes: HashMap::new(),
            depends: Vec::new()
        }
    }
    
    fn execute(&mut self, data: &Vec<PathBuf>, output: &PathBuf) -> LanguageAnalysisResult<PathBuf> {
        let mut filelist: Vec<PathBuf> = Vec::new();
        for d in data {
            if d.is_file() {
                filelist.push(d.to_path_buf());
            } else {
                let filename_pattern = format!("{}/**/*.{}", d.to_str().unwrap(), VERTEX_FILE_EXTENSION);
                if let Some(mut files) = utils::list_files(d, filename_pattern.as_str(), &Vec::new()) {
                    filelist.append(&mut files);
                }
            }
        }

        for vertex_file in filelist {
            if let Ok(vertexes) = self.load_vertex_from_file(&PathBuf::from(vertex_file)) {
                if let Err(err) = self.analysis(&vertexes) {
                    return Err(err);
                }
            }
        }

        for (key, node) in &self.graph_nodes {
            println!("# {}", key);
            for def in &node.defines {
                println!(" * [{}] {}", def.to_signature(), def.to_string())
            }

            for call in &node.callouts {
                println!(" * [{}] {}", call.to_signature(), call.to_string());
            }
        }

        let dot_file_path = output.join(format!("{}.{}", "graphviz", GRAPH_FILE_EXTENSION));
        _ = self.generate_graphviz_dot_file(&dot_file_path);

        Ok(dot_file_path)
    }
}