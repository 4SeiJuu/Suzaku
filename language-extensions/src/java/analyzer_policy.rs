use std::{
    path::PathBuf, 
    collections::HashMap, 
    fs::{
        File, 
        self
    }, 
    io::Write
};
use serde::Serialize;

use suzaku_extension_sdk::{
    language::{
        analyzer::{
            LanguageAnalysisPolicy,
            LanguageAnalysisResult, 
            LanguageAnalysisPolicyError,
        }, 
        element::{
            Elements, 
            ElementCategories, 
            IElement, 
            ToSignature
        }
    },
};

use super::{
    element::JavaElement,
    graphviz::{
        GraphVertex,
        GraphEdge
    }
};

enum JavaElementCategory {
    Defines,
    CallOuts
}

#[derive(Debug, Serialize)]
pub struct JavaAnalyzer {
    depends: Vec<GraphEdge>,
    elements: HashMap<String, GraphVertex>
}

impl JavaAnalyzer {
    pub fn load_elements_from_file(&self, path: &PathBuf) -> LanguageAnalysisResult<HashMap<ElementCategories, Vec<JavaElement>>> {
        let context_str = fs::read_to_string(path).expect("should read context of file");

        match serde_json::from_str(&context_str) {
            Ok(contents) => Ok(contents),
            Err(err) => Err(LanguageAnalysisPolicyError {})
        }
    }

    pub fn analysis(&mut self, elements: &HashMap<ElementCategories, Vec<JavaElement>>) -> LanguageAnalysisResult<()> {
        if let Some(jvs) = elements.get(&ElementCategories::Classes) {
            for jv in jvs {
                if let Some((_, vertex)) = self.collect_elements(jv) {
                    self.elements.insert(vertex.get_signature(), vertex);
                }
            }
        }

        if let Some(jvs) = elements.get(&ElementCategories::Interfaces) {
            for jv in jvs {
                if let Some((_, vertex)) = self.collect_elements(jv) {
                    self.elements.insert(vertex.get_signature(), vertex);
                }
            }
        }

        self.collect_depends();

        Ok(())
    }

    fn collect_elements(&self, jv: &JavaElement) -> Option<(JavaElementCategory, GraphVertex)> {
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

        let applying = |parent: &mut GraphVertex, jv: &JavaElement| {
            if let Some((cate, vertex)) = self.collect_elements(jv) {
                if let Some(element_type) = vertex.ty {
                    match cate {
                        JavaElementCategory::Defines => parent.defines.push(element_type),
                        JavaElementCategory::CallOuts => parent.callouts.push(element_type),
                        _ => ()
                    }
                }
            }
        };

        let element_type = jv.get_type();
        if let Some(et) = element_type {
            let mut graph_node = GraphVertex {
                package: get_package(element_type),
                ty: get_ty(element_type),
                defines: Vec::new(),
                callouts: Vec::new()
            };

            // apply child members
            if let Some(constructors) = jv.get_member_by_category(ElementCategories::Constructors) {
                for constructor in constructors {
                    applying(&mut graph_node, &constructor);
                }
            }

            if let Some(fields) = jv.get_member_by_category(ElementCategories::Fields) {
                for field in fields {
                    applying(&mut graph_node, &field);
                }
            }

            if let Some(methods) = jv.get_member_by_category(ElementCategories::Methods) {
                for method in methods {
                    applying(&mut graph_node, &method);
                }
            }

            if let Some(creator_calls) = jv.get_member_by_category(ElementCategories::CreatorCalls) {
                for creator_call in creator_calls {
                    applying(&mut graph_node, &creator_call);
                }
            }

            if let Some(method_calls) = jv.get_member_by_category(ElementCategories::MethodCalls) {
                for method_call in method_calls {
                    applying(&mut graph_node, &method_call);
                }
            }
    
            return match et {
                Elements::Class(_, _, _, _, _, _) 
                | Elements::Interface(_, _, _, _, _) 
                | Elements::Constructor(_, _, _, _) 
                | Elements::Field(_, _, _, _, _) 
                | Elements::Method(_, _, _, _, _, _) => 
                    Some((JavaElementCategory::Defines, graph_node)),
                Elements::CreatorCall(_, _, _) 
                | Elements::MethodCall(_, _, _, _) =>
                    Some((JavaElementCategory::CallOuts, graph_node)),
                _ => None
            }
        }
        None
    }

    fn collect_depends(&mut self) {
        let mut depends: Vec<(GraphVertex, GraphVertex, bool)> = Vec::new();
        for element in self.elements.values() {
            if let Some(element_type) = &element.ty {
                if let Some((to, need_append)) = self.collect_depends_in_element(element_type) {
                    depends.push((element.clone(), to.clone(), need_append));
                }
            }

            for define in &element.defines {
                if let Some((to, need_append)) = self.collect_depends_in_element(define) {
                    depends.push((element.clone(), to.clone(), need_append));
                }
            }

            for callouts in &element.callouts {
                if let Some((to, need_append)) = self.collect_depends_in_element(callouts) {
                    depends.push((element.clone(), to.clone(), need_append));
                }
            }
        }

        for (from, to, need_append) in depends {
            self.apply_depends(from, to, need_append)
        }
    }

    fn collect_depends_in_element(&self, element: &Elements) -> Option<(GraphVertex, bool)> {
        let create_vertex = |package: Elements, ty: Elements| -> GraphVertex {
            GraphVertex {
                package: Some(package),
                ty: Some(ty),
                defines: Vec::new(),
                callouts: Vec::new()
            }
        };

        let signature = element.to_signature();
        match self.elements.get(&signature) {
            Some(vertex) => Some((vertex.clone(), false)),
            None => {
                match element {
                    Elements::Class(td, _, _, _, _, _) => {
                        // ancestors, annotations, modifiers, name, extends, implements
                        Some((create_vertex(Elements::Package(td.package.clone()), Elements::Class(td.clone(), Vec::new(), Vec::new(), String::from(""), Vec::new(), Vec::new())), true))
                    },
                    Elements::Interface(td, _, _, _, _) => {
                        // ancestors, annotations, modifiers, name, extends
                        Some((create_vertex(Elements::Package(td.package.clone()), Elements::Interface(td.clone(), Vec::new(), Vec::new(), String::from(""), Vec::new())), true))
                    },
                    _ => None
                }
            }
        }
    }

    fn apply_depends (&mut self, from: GraphVertex, to: GraphVertex, need_append: bool) {
        if from.get_signature() != to.get_signature() {
            if need_append {
                self.elements.insert(to.clone().get_signature(), to.clone());
            }
            self.depends.push(GraphEdge { from: from, to: to });
        }
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
            print!("# loading elements ... ");
            let load_result = self.load_elements_from_file(data);
            match load_result {
                Ok(all_elements) => {
                    println!("done"); // loading elements done
    
                    print!("# analysing ... ");
                    if let Err(err) = self.analysis(&all_elements) {
                        println!("failed. Error: {:?}", err);
                        return Err(err);
                    } else {
                        println!("done")
                    }

                    // save data
                    let output_file_path = output.join(String::from("analysed.data"));
                    if let Ok(mut f) = File::create(&output_file_path) {
                        let _ = f.write_all(serde_json::to_string(&self).unwrap().as_bytes());
                        let _ = f.flush();
                    }
            
                    // outputs for debug
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
            
                    return Ok(output_file_path);
                },
                Err(err) => {
                    println!("failed"); // loading elements failed
                    println!("{:?}", err);
                    return Err(LanguageAnalysisPolicyError {})
                }
            }
        }

        Err(LanguageAnalysisPolicyError {})
    }
}