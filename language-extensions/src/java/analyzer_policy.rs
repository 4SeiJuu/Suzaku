use std::{
    path::PathBuf, 
    collections::HashMap, 
    fs::{
        self, 
        File
    }, 
    io::Write
};

use suzaku_extension_sdk::{
    language::{
        analyzer::{
            LanguageAnalysisPolicy,
            LanguageAnalysisPolicyError,
            LanguageAnalysisResult
        }, inode::INode, 
        ivertex::{
            Vertex,
            VertexType,
            VertexRelationship
        }
    },
    ANALYZED_RESULTS_FOLDER_NAME,
    VERTEX_FILE_EXTENSION,
};

use crate::java::node_type::JavaNodeType;

use super::{
    vertex::JavaVertex, 
    node::JavaNode
};

pub struct JavaAnalysisListener {}

impl JavaAnalysisListener {
    fn listen(&self, node: &JavaNode) {
        match node.get_node_type() {
            JavaNodeType::PackageDeclaration => self.analysis_package_declaration(node),
            JavaNodeType::ImportDeclaration => {},
            JavaNodeType::ClassDeclaration => {},
            JavaNodeType::InterfaceDeclaration => {}
            JavaNodeType::FieldDeclaration => {},
            JavaNodeType::MethodDeclaration => {},
            JavaNodeType::AnnotationTypeDeclaration => {},
            JavaNodeType::ConstructorDeclaration => {},
            JavaNodeType::VariableDeclarators => {},
            JavaNodeType::Creator => {},
            JavaNodeType::MethodCall => {},
            JavaNodeType::Annotation => {},
            JavaNodeType::Modifier => {},
            JavaNodeType::Identifier => {},
            JavaNodeType::TypeType => {},
            JavaNodeType::TypeList => {},
            _ => ()
        }
    }
    
    fn analysis_package_declaration(&self, node: &JavaNode) {
        assert_eq!(node.get_node_type(), JavaNodeType::PackageDeclaration);

        for member in node.get_members() {
            match member.get_node_type() {
                JavaNodeType::QualifiedName => {},
                _ => ()
            }
        }
    }
}

pub struct JavaAnalysisPolicy {
    vertexes: HashMap<VertexRelationship, Vec<JavaVertex>>
}

impl<'a> JavaAnalysisPolicy {
    pub fn analysis(&mut self, node: &JavaNode, listener: &JavaAnalysisListener) -> LanguageAnalysisResult<JavaVertex> {
        assert_eq!(node.get_node_type(), JavaNodeType::File);
        JavaAnalysisPolicy::tree_walker(&node, listener);

        Err(LanguageAnalysisPolicyError {  })
    }

    fn tree_walker(node: &JavaNode, listener: &JavaAnalysisListener) {
        listener.listen(&node);
        for child in node.get_members() {
            JavaAnalysisPolicy::tree_walker(child, listener);
        }
    }
}

impl LanguageAnalysisPolicy for JavaAnalysisPolicy {
    fn new() -> Self {
        JavaAnalysisPolicy {
            vertexes: HashMap::new()
        }
    }

    fn execute(&mut self, metadata: &PathBuf, output: &PathBuf) -> LanguageAnalysisResult<PathBuf> {
        if metadata.is_file() && metadata.exists() {
            let context_str = fs::read_to_string(metadata).expect("should read context of file");
            let context: JavaNode = serde_json::from_str(&context_str).expect("failed to convert metadata to hashmap");

            let listener = JavaAnalysisListener{};

            if let Ok(jv) = self.analysis(&context, &listener) {
                if !output.exists() {
                    _ = fs::create_dir_all(&output);
                }

                let output_file_path = output.join(format!("{}.{}", metadata.file_stem().unwrap().to_str().unwrap(), VERTEX_FILE_EXTENSION));
                if let Ok(mut f) = File::create(&output_file_path) {
                    let _ = f.write_all(serde_json::to_string(&jv).unwrap().as_bytes());
                    let _ = f.flush();
                }
                return Ok(output_file_path);
            }
        }
        Err(LanguageAnalysisPolicyError {})
    }
}



// fn analysis_package_declaration(node: &'a JavaNode) -> Option<(&VertexRelationship, &[JavaVertex<'a>])> {
    //     assert_eq!(node.get_node_type(), JavaNodeType::PackageDeclaration);

    //     for member in node.get_members() {
    //         match member.get_node_type() {
    //             JavaNodeType::QualifiedName => {
    //                 let vertex_type: &'a VertexType<'a> = VertexType::<'a>::Package(member.get_attr().as_ref().unwrap()).as_ref();
    //                 return Some((VertexRelationship::Package.as_ref(), &[JavaVertex::<'a>::new(vertex_type)]))
    //             },
    //             _ => return None
    //         }
    //     }
    //     None
    // }
    


    // pub fn analysis(&mut self, context: JavaNode) -> LanguageAnalysisResult<JavaVertex<'a>> {
    //     assert_eq!(context.get_node_type(), JavaNodeType::File);

    //     for member in context.get_members() {
    //         match member.get_node_type() {
    //             JavaNodeType::PackageDeclaration => self.analysis_package(member),
    //             JavaNodeType::ImportDeclaration => self.analysis_import(member),
    //             JavaNodeType::ClassDeclaration => self.analysis_class(member),
    //             JavaNodeType::FieldDeclaration => self.analysis_field(member),
    //             JavaNodeType::MethodDeclaration => self.analysis_method(member),
    //             _ => ()
    //         }
    //     }
    //     Ok(JavaVertex::new(&VertexType::Record))
    // }

    // fn analysis_package(&mut self, package_node: &JavaNode) {
    //     assert_eq!(package_node.get_node_type(), JavaNodeType::PackageDeclaration);

    //     for member in package_node.get_members() {
    //         if member.get_node_type() == JavaNodeType::QualifiedName {
    //             println!("[PACKAGE] {:?}", member.get_attr());
    //         }
    //     }
    // }

    // fn analysis_import(&mut self, import_node: &JavaNode) {
    //     assert_eq!(import_node.get_node_type(), JavaNodeType::ImportDeclaration);

    //     for member in import_node.get_members() {
    //         if member.get_node_type() == JavaNodeType::QualifiedName {
    //             println!("[IMPORT] {:?}", member.get_attr());
    //         }
    //     }
    // }

    // fn analysis_class(&mut self, class_node: &JavaNode) {
    //     assert_eq!(class_node.get_node_type(), JavaNodeType::ClassDeclaration);

    //     let mut annotations: Vec<String> = Vec::new();
    //     let mut modifiers: Vec<String> = Vec::new();
    //     let mut ident: Option<String> = None;
    //     let mut extends: Option<String> = None;
    //     let mut implements: Vec<String> = Vec::new();

    //     for member in class_node.get_members() {
    //         match member.get_node_type() {
    //             JavaNodeType::Annotation => if let Some(attr) = member.get_attr() {
    //                 annotations.push(attr.clone());
    //             },
    //             JavaNodeType::Modifier => if let Some(attr) = member.get_attr() {
    //                 modifiers.push(attr.clone());
    //             },
    //             JavaNodeType::Identifier => if let Some(attr) = member.get_attr() {
    //                 ident = Some(attr.clone());
    //             },
    //             JavaNodeType::TypeType => if let Some(attr) = member.get_attr() {
    //                 extends = Some(attr.clone());
    //             },
    //             JavaNodeType::TypeList => if let Some(attr) = member.get_attr() {
    //                 implements.push(attr.clone())
    //             }
    //             JavaNodeType::ClassBody => self.analysis_class_body(member),
    //             _ => ()
    //         }
    //     }

    //     for anno in annotations {
    //         println!("[ANNOTATION] {}", anno);
    //     }

    //     println!("[CLASS] {} class {} extends {:?} implements {}", modifiers.join(" ").as_str(), ident.unwrap(), extends, implements.join(", "));
    // }

    // fn analysis_class_body(&mut self, class_body_node: &JavaNode) {
    //     assert_eq!(class_body_node.get_node_type(), JavaNodeType::ClassBody);

    //     for member in class_body_node.get_members() {
    //         match member.get_node_type() {
    //             JavaNodeType::FieldDeclaration => self.analysis_field(member),
    //             JavaNodeType::AnnotationTypeDeclaration => self.analysis_annotation_declaration(member),
    //             JavaNodeType::ClassDeclaration => self.analysis_class(member),
    //             JavaNodeType::MethodDeclaration => self.analysis_method(member),
    //             JavaNodeType::ConstructorDeclaration => self.analysis_ctor(member),
    //             _ => ()
    //         }
    //     }
    // }

    // fn analysis_field(&mut self, field_node: &JavaNode) {
    //     assert_eq!(field_node.get_node_type(), JavaNodeType::FieldDeclaration);

    //     let mut modifiers: Vec<String> = Vec::new();
    //     let mut ty: Option<&String> = None;
    //     let mut variable_id: Option<&String> = None;
    //     let mut variable_init: Option<&String> = None;

    //     for member in field_node.get_members() {
    //         match member.get_node_type() {
    //             JavaNodeType::Modifier => modifiers.push(member.get_attr().as_ref().unwrap().clone()),
    //             JavaNodeType::TypeType => ty = Some(member.get_attr().as_ref().unwrap()),
    //             JavaNodeType::VariableDeclarators => {
    //                 if member.get_members().len() == 0 {
    //                     variable_id = Some(member.get_attr().as_ref().unwrap());
    //                     continue;
    //                 }

    //                 for child in member.get_members() {
    //                     match child.get_node_type() {
    //                         JavaNodeType::VariableDeclaratorId => variable_id = Some(child.get_attr().as_ref().unwrap()),
    //                         JavaNodeType::VariableInitializer => if child.get_members().len() == 0 {
    //                             variable_init = Some(child.get_attr().as_ref().unwrap())
    //                         } else {
    //                             // creator & method call
    //                         },
    //                         _ => () // println!("[WARNING] not produced node: {}", child.dump().unwrap())
    //                     }
    //                 }
    //             },
    //             _ => ()
    //         }
    //     }

    //     println!("[FIELD] modifiers: {}, type: {}, ident: {}, initializer: {:?}", modifiers.join(" "), ty.unwrap(), variable_id.unwrap(), variable_init);
    // }

    // fn analysis_ctor(&mut self, ctor: &JavaNode) {
    //     assert_eq!(ctor.get_node_type(), JavaNodeType::ConstructorDeclaration);
    // }

    // fn analysis_method(&mut self, method_node: &JavaNode) {
    //     assert_eq!(method_node.get_node_type(), JavaNodeType::MethodDeclaration);

    //     let mut annotation: Option<&String> = None;
    //     let mut modifiers: Vec<String> = Vec::new();
    //     let mut ret_type: Option<&String> = None;
    //     let mut name: Option<&String> = None;
    //     let mut params: Vec<(&String, &String)> = Vec::new();

    //     for member in method_node.get_members() {
    //         match member.get_node_type() {
    //             JavaNodeType::Annotation => annotation = Some(member.get_attr().as_ref().unwrap()),
    //             JavaNodeType::Modifier => modifiers.push(member.get_attr().as_ref().unwrap().clone()),
    //             JavaNodeType::TypeTypeOrVoid => ret_type = Some(member.get_attr().as_ref().unwrap()),
    //             JavaNodeType::Identifier => name = Some(member.get_attr().as_ref().unwrap()),
    //             JavaNodeType::FormalParameters => {
    //                 for child in member.get_members() {
    //                     match child.get_node_type() {
    //                         JavaNodeType::FormalParameterList => {
    //                             for param in child.get_members() {
    //                                 match param.get_node_type() {
    //                                     JavaNodeType::FormalParameter => {
    //                                         let mut ty: Option<&String> = None;
    //                                         let mut ident: Option<&String> = None;
    //                                         for part in param.get_members() {
    //                                             match part.get_node_type() {
    //                                                 JavaNodeType::TypeType => ty = Some(part.get_attr().as_ref().unwrap()),
    //                                                 JavaNodeType::VariableDeclaratorId => ident = Some(part.get_attr().as_ref().unwrap()),
    //                                                 _ => () // println!("[WARNING] not produced node: {}", part.dump().unwrap())
    //                                             }
    //                                         }
    //                                         params.push((ty.unwrap(), ident.unwrap()))
    //                                     },
    //                                     _ => ()
    //                                 }
    //                             }
    //                         },
    //                         _ => ()
    //                     }
    //                 }
    //             },
    //             _ => ()
    //         }
    //     }

    //     let mut param_strs: Vec<String> = Vec::new();
    //     if params.len() == 0 {
    //         param_strs.push(String::from("()"));
    //     } else {
    //         for p in params {
    //             param_strs.push(format!("{} {}", p.0, p.1));
    //         }
    //     }
    //     println!("[METHOD] annotation: {:?}, modifiers: {}, return type: {}, ident: {}, params: {}", 
    //         annotation, modifiers.join(" "), ret_type.unwrap(), name.unwrap(), param_strs.join(", "));
    // }

    // fn analysis_annotation_declaration(&mut self, annotation_declaration_node: &JavaNode) {
    //     assert_eq!(annotation_declaration_node.get_node_type(), JavaNodeType::AnnotationTypeDeclaration);

    //     for annotation in annotation_declaration_node.get_members() {
    //         let mut name: Option<&String> = None;
    //         let mut ele_value: Option<&String> = None;
    //         let mut ele_value_pairs: Vec<(String, String)> = Vec::new();
    
    //         let mut analysis_element_value_pair = |node: &JavaNode| {
    //             assert_eq!(node.get_node_type(), JavaNodeType::ElementValuePair);
    
    //             let mut ident: Option<&String> = None;
    //             let mut value: Option<&String> = None;
    //             for item in node.get_members() {
    //                 match item.get_node_type() {
    //                     JavaNodeType::Identifier => ident = Some(item.get_attr().as_ref().unwrap()),
    //                     JavaNodeType::ElementValue => value = Some(item.get_attr().as_ref().unwrap()),
    //                     _ => ()
    //                 }
    //             }
    
    //             if ident.is_none() || value.is_none() {
    //                 panic!("[ERROR] ident and value should not be none for ElementValue: {}", node.dump().unwrap());
    //             }
    
    //             ele_value_pairs.push((ident.unwrap().clone(), value.unwrap().clone()));
    //         };

    //         match annotation.get_node_type() {
    //             JavaNodeType::Annotation => {
    //                 for member in annotation.get_members() {
    //                     match member.get_node_type() {
    //                         JavaNodeType::QualifiedName => name = Some(member.get_attr().as_ref().unwrap()),
    //                         JavaNodeType::ElementValue => ele_value = Some(member.get_attr().as_ref().unwrap()),
    //                         JavaNodeType::ElementValuePair => analysis_element_value_pair(member),
    //                         JavaNodeType::ElementValuePairs => for pair in member.get_members() {
    //                             analysis_element_value_pair(pair);
    //                         },
    //                         _ => ()
    //                     }
    //                 }
    //             },
    //             _ => ()
    //         }

    //         print!("[ANNOTATION] name: {:?}, ", name);
    //         if let Some(value) = ele_value {
    //             println!(" element value: {}", value);
    //         } else {
    //             print!(" element value pairs: [");
    //             for (ident, value) in ele_value_pairs {
    //                 print!(" (ident: {}, value: {}), ", ident, value);
    //             }
    //             println!("]");
    //         }
    //     }
    // }