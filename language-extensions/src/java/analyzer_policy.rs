use std::{path::PathBuf, collections::HashMap, fs::{File, self}, io::Write};

use serde::Deserialize;
use serde_json::Value;

use suzaku_extension_sdk::{
    language::{analyzer::{
        LanguageAnalysisPolicy,
        LanguageAnalysisResult, LanguageAnalysisPolicyError,
    }, ivertex::{VertexType, VertexCategories, IVertex, ToSignature}},
    GRAPH_FILE_EXTENSION
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

    pub fn get_package_name(&self) -> String {
        match &self.package {
            Some(p) => p.to_string(),
            None => String::from("others")
        }
    }

    pub fn to_graphviz_node(&self) -> String {
        let node_name = self.get_signature();
        let label = self.get_signature();
        let group = self.get_package_name();

        format!("{} [label={}, group={}];", node_name, label, group)
    }
}

pub struct JavaAnalyzer {
    depends: Vec<VertexType>,
    graph_nodes: HashMap<String, GraphNode>
}

impl JavaAnalyzer {
    pub fn load_vertex_from_file(&self, path: &PathBuf) -> LanguageAnalysisResult<HashMap<VertexCategories, Vec<JavaVertex>>> {
        let context_str = fs::read_to_string(path).expect("should read context of file");

        match serde_json::from_str(&context_str) {
            Ok(contents) => Ok(contents),
            Err(err) => Err(LanguageAnalysisPolicyError {})
        }
    }

    pub fn analysis(&mut self, vertexes: &HashMap<VertexCategories, Vec<JavaVertex>>) -> LanguageAnalysisResult<()> {
        let get_package = |vt: Option<&VertexType>| -> Option<VertexType> {
            match vt {
                Some(ty) => match &ty {
                    &VertexType::Class(td, _, _, _, _, _) 
                    | &VertexType::Interface(td, _, _, _, _)
                    | &VertexType::Constructor(td, _, _, _)
                    | &VertexType::Field(td, _, _, _, _)
                    | &VertexType::Method(td, _, _, _, _, _) => 
                        Some(VertexType::Package(td.package.clone())),
                    
                    &VertexType::MethodCall(_, caller, _, _) => None, // TODO: need to implement collecting information of caller
                    &VertexType::CreatorCall(pkg, _, _) => Some(VertexType::Package(pkg.clone())),
                    _ => None
                },
                None => None
            }
        };

        let get_ty = |vt: Option<&VertexType>| -> Option<VertexType> {
            match vt {
                Some(t) => Some(t.clone()),
                None => None
            }
        };

        if let Some(jvs) = vertexes.get(&VertexCategories::Classes) {
            for jv in jvs {
                let vt = jv.get_type();
                let mut graph_node = GraphNode {
                    package: get_package(vt),
                    ty: get_ty(vt),
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
                    package: get_package(jv.get_type()),
                    ty: get_package(jv.get_type()),
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
    
    fn execute(&mut self, data: &PathBuf, output: &PathBuf) -> LanguageAnalysisResult<PathBuf> {
        if data.is_file() {
            print!("# loading vertexes ... ");
            let load_result = self.load_vertex_from_file(data);
            match load_result {
                Ok(all_vertexes) => {
                    println!("done"); // loading vertexes done
    
                    print!("# analysing ... ");
                    if let Err(err) = self.analysis(&all_vertexes) {
                        println!("failed. Error: {:?}", err);
                        return Err(err);
                    } else {
                        println!("done")
                    }
            
                    // outputs
                    let mut sorted_graph_nodes: Vec<_> = self.graph_nodes.iter().collect();
                    sorted_graph_nodes.sort_by_key(|k| k.0);
            
                    for (key, node) in sorted_graph_nodes {
                        println!("# [{}] {}", key, node.ty.as_ref().unwrap().to_string());
            
                        let mut sorted_defines = node.defines.clone();
                        sorted_defines.sort_by(|i, j| i.to_signature().cmp(&j.to_signature()));
            
                        for def in sorted_defines {
                            println!(" * [{}] {}", def.to_signature(), def.to_string())
                        }
            
                        let mut sorted_callouts = node.callouts.clone();
                        sorted_callouts.sort_by(|i, j| i.to_signature().cmp(&j.to_signature()));
            
                        for call in sorted_callouts {
                            println!(" * [{}] {}", call.to_signature(), call.to_string());
                        }
                    }
        
                    // save graphviz file
                    let dot_file_path = output.join(format!("{}.{}", "graphviz", GRAPH_FILE_EXTENSION));
                    _ = self.generate_graphviz_dot_file(&dot_file_path);
            
                    return Ok(dot_file_path);
                },
                Err(err) => {
                    println!("failed"); // loading vertexes failed
                    println!("{:?}", err);
                    return Err(LanguageAnalysisPolicyError {})
                }
            }
        }

        Err(LanguageAnalysisPolicyError {})
    }
}