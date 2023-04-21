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
    analyzer::{
        LanguageAnalysisPolicy,
        LanguageAnalysisResult, 
        LanguageAnalysisPolicyError,
    }, 
    element::{
        Elements, 
        ElementCategories, 
        IElement, 
        ToSignature, 
        TypeDescriptor
    },
};

use super::super::data::{
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
    depends: HashMap<String, GraphEdge>,
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
                    self.elements.insert(vertex.to_signature(), vertex);
                }
            }
        }

        if let Some(jvs) = elements.get(&ElementCategories::Interfaces) {
            for jv in jvs {
                if let Some((_, vertex)) = self.collect_elements(jv) {
                    self.elements.insert(vertex.to_signature(), vertex);
                }
            }
        }

        self.collect_depends();

        Ok(())
    }

    fn collect_elements(&self, jv: &JavaElement) -> Option<(JavaElementCategory, GraphVertex)> {
        let applying = |parent: &mut GraphVertex, jv: &JavaElement| {
            if let Some((cate, vertex)) = self.collect_elements(jv) {
                match cate {
                    JavaElementCategory::Defines => parent.defines.push(vertex.ty),
                    JavaElementCategory::CallOuts => parent.callouts.push(vertex.ty),
                    _ => ()
                }
            }
        };

        let element_type = jv.get_type();
        if let Some(et) = element_type {
            let mut graph_node = GraphVertex::new(et.clone());

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
                Elements::CreatorCall(_, _) 
                | Elements::MethodCall(_, _, _, _) =>
                    Some((JavaElementCategory::CallOuts, graph_node)),
                _ => None
            }
        }
        None
    }

    fn collect_depends(&mut self) {
        let mut need_append_vertexes: Vec<GraphVertex> = Vec::new();
        for element in self.elements.values() {
            let from = GraphVertex::new(element.ty.clone());

            if let Some(depends) = self.collect_depends_in_element(element) {
                for (to, need_append) in depends {
                    if from.get_package_name() == to.to_signature() {
                        continue;
                    }

                    let edge = GraphEdge { from: from.clone(), to:  to.clone()};
                    let key = edge.to_signature();
                    if !self.depends.contains_key(&key) {
                        self.depends.insert(key, edge);
                        if need_append {
                            need_append_vertexes.push(to);
                        }
                    }
                }
            }
        }

        for v in need_append_vertexes {
            self.elements.insert(v.to_signature(), v);
        }
    }

    fn collect_depends_in_element(&self, gv: &GraphVertex) -> Option<Vec<(GraphVertex, bool)>> {
        let mut depends: Vec<(GraphVertex, bool)> = Vec::new();
        let mut collecting = |descriptor: &TypeDescriptor| {
            if let Some(depend) = self.collect_depends_by_type_descriptor(descriptor) {
                depends.push(depend);
            }
        };

        let mut element_collecting = |element_type: &Elements| {
            match element_type {
                // ancestors, annotations, modifiers, name, extends, implements
                Elements::Class(_, _, _, _, extends, implements) => {
                    for extend in extends {
                        collecting(extend);
                    }

                    for implement in implements {
                        collecting(implement);
                    }
                },
                // ancestors, annotations, modifiers, name, extends
                Elements::Interface(_, _, _, _, extends) => {
                    for extend in extends {
                        collecting(extend);
                    }
                },
                // ancestors, modifiers, ident, params(modifiers, type, name)
                Elements::Constructor(_, _, _, params) => {
                    for param in params {
                        collecting(&param.ty);
                    }
                },
                // ancestors, modifiers, field type, field name, field value
                Elements::Field(_, _, field_type, _, _) => {
                    if let Some(ft) = field_type {
                        collecting(ft);
                    }
                },
                // ancestors, annotation, modifiers, return type, function name, params(variable(modifier, type, name))
                Elements::Method(_, _, _, ret_type, _, params) => {
                    collecting(ret_type);

                    for param in params {
                        collecting(&param.ty);
                    }
                },
                // type, rest
                Elements::CreatorCall(creator_type, rests) => {
                    collecting(creator_type);

                    // TODO:
                    // for rest in rests {
                    //     collecting(&rest.ty);
                    // }
                }
                // cast, caller, method name, params((annotation, type, name))
                Elements::MethodCall(_, caller, _, params) => {
                    collecting(&caller.ty);

                    // TODO:
                    // for param in params {
                    //     collecting(&param.ty);
                    // }
                },
                _ => {}
            }
        };

        element_collecting(&gv.ty);

        // defines
        for define in &gv.defines {
            element_collecting(define);
        }

        // callouts
        for callout in &gv.callouts {
            element_collecting(callout);
        }

        Some(depends)
    }

    fn collect_depends_by_type_descriptor(&self, type_descriptor: &TypeDescriptor) -> Option<(GraphVertex, bool)> {
        match self.elements.get(&type_descriptor.to_signature()) {
            Some(vertex) => Some((GraphVertex::new(vertex.ty.clone()), false)),
            None => Some((GraphVertex::new(Elements::UnknownType(type_descriptor.clone())), true))
        }
    }
}

impl LanguageAnalysisPolicy for JavaAnalyzer {
    fn new() -> Self {
        JavaAnalyzer {
            elements: HashMap::new(),
            depends: HashMap::new()
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
            
                    return Ok(output_file_path);
                },
                Err(err) => {
                    println!("failed"); // loading elements failed
                    return Err(LanguageAnalysisPolicyError {})
                }
            }
        }

        Err(LanguageAnalysisPolicyError {})
    }
}