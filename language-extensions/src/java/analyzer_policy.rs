use std::{path::PathBuf, collections::HashMap, fs::{File, self}, io::Write};

use suzaku_extension_sdk::{
    language::{analyzer::{
        LanguageAnalysisPolicy,
        LanguageAnalysisResult, LanguageAnalysisPolicyError,
    }, element::{Elements, ElementCategories, IElement, ToSignature}},
    GRAPH_FILE_EXTENSION
};

use super::{
    element::JavaElement,
    graphviz::{
        GraphVertex,
        GraphEdge
    }
};

enum JavaAnalysisedResultCategory {
    Depends,
    Defines,
    CallOuts
}

pub struct JavaAnalyzer {
    depends: Vec<GraphEdge>,
    elements: HashMap<String, GraphVertex>
}

impl JavaAnalyzer {
    pub fn load_vertex_from_file(&self, path: &PathBuf) -> LanguageAnalysisResult<HashMap<ElementCategories, Vec<JavaElement>>> {
        let context_str = fs::read_to_string(path).expect("should read context of file");

        match serde_json::from_str(&context_str) {
            Ok(contents) => Ok(contents),
            Err(err) => Err(LanguageAnalysisPolicyError {})
        }
    }

    pub fn analysis(&mut self, vertexes: &HashMap<ElementCategories, Vec<JavaElement>>) -> LanguageAnalysisResult<()> {
        let get_package = |vt: Option<&Elements>| -> Option<Elements> {
            match vt {
                Some(ty) => match &ty {
                    // definations
                    &Elements::Class(td, _, _, _, _, _) 
                    | &Elements::Interface(td, _, _, _, _)
                    | &Elements::Constructor(td, _, _, _)
                    | &Elements::Field(td, _, _, _, _)
                    | &Elements::Method(td, _, _, _, _, _) => 
                        Some(Elements::Package(td.package.clone())),
                    // call outs
                    // TODO: need to implement collecting information of caller
                    &Elements::MethodCall(_, caller, _, _) => None, 
                    &Elements::CreatorCall(pkg, _, _) => Some(Elements::Package(pkg.clone())),
                    _ => None
                },
                None => None
            }
        };

        let get_ty = |vt: Option<&Elements>| -> Option<Elements> {
            match vt {
                Some(t) => Some(t.clone()),
                None => None
            }
        };

        if let Some(jvs) = vertexes.get(&ElementCategories::Classes) {
            for jv in jvs {
                let vt = jv.get_type();
                let mut graph_node = GraphVertex {
                    package: get_package(vt),
                    ty: get_ty(vt),
                    defines: Vec::new(),
                    callouts: Vec::new()
                };

                // definations
                if let Some(members) = jv.get_member_by_category(ElementCategories::Constructors) {
                    self.apply(&mut graph_node, JavaAnalysisedResultCategory::Defines, members);
                }

                if let Some(members) = jv.get_member_by_category(ElementCategories::Fields) {
                    self.apply(&mut graph_node, JavaAnalysisedResultCategory::Defines, members);
                }

                if let Some(members) = jv.get_member_by_category(ElementCategories::Methods) {
                    self.apply(&mut graph_node, JavaAnalysisedResultCategory::Defines, members);
                }

                // call outs
                if let Some(members) = jv.get_member_by_category(ElementCategories::CreatorCalls) {
                    self.apply(&mut graph_node, JavaAnalysisedResultCategory::CallOuts, members);
                }

                if let Some(members) = jv.get_member_by_category(ElementCategories::MethodCalls) {
                    self.apply(&mut graph_node, JavaAnalysisedResultCategory::CallOuts, members);
                }

                self.elements.insert(graph_node.get_signature(), graph_node);
            }
        }

        if let Some(jvs) = vertexes.get(&ElementCategories::Interfaces) {
            for jv in jvs {
                let mut graph_node = GraphVertex {
                    package: get_package(jv.get_type()),
                    ty: get_package(jv.get_type()),
                    defines: Vec::new(),
                    callouts: Vec::new()
                };

                // definations
                if let Some(members) = jv.get_member_by_category(ElementCategories::Methods) {
                    self.apply(&mut graph_node, JavaAnalysisedResultCategory::Defines, members);
                }

                // call outs
                if let Some(members) = jv.get_member_by_category(ElementCategories::CreatorCalls) {
                    self.apply(&mut graph_node, JavaAnalysisedResultCategory::CallOuts, members);
                }

                if let Some(members) = jv.get_member_by_category(ElementCategories::MethodCalls) {
                    self.apply(&mut graph_node, JavaAnalysisedResultCategory::CallOuts, members);
                }

                self.elements.insert(graph_node.get_signature(), graph_node);
            }
        }
        Ok(())
    }

    fn apply(&self, node: &mut GraphVertex, category: JavaAnalysisedResultCategory, vertexes: &Vec<Box<JavaElement>>) {
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
        for (_name, node) in &self.elements {
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
            elements: HashMap::new(),
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
                    let mut sorted_graph_nodes: Vec<_> = self.elements.iter().collect();
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