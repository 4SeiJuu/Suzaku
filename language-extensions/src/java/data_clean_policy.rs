use std::{
    path::PathBuf, 
    collections::HashMap, 
    fs::{
        self, 
        File
    }, 
    io::Write
};

use serde::Deserialize;
use serde_json::Value;

use suzaku_extension_sdk::{
    language::{
        data_cleaner::{
            LanguageDataCleanPolicy,
            LanguageDataCleanPolicyError,
            LanguageDataCleanResult
        }, inode::INode, 
        ivertex::{
            IVertex,
            VertexType,
            VertexCategories, TypeDescriptor, ParamDescriptor
        }
    },
    stack::Stack,
    VERTEX_FILE_EXTENSION, 
};

use crate::java::node_type::JavaNodeType;

use super::{
    vertex::JavaVertex, 
    node::JavaNode
};

pub struct JavaDataCleanListener {
    vertexes: HashMap<VertexCategories, Vec<JavaVertex>>,
    stack: Stack<JavaVertex>
}

impl JavaDataCleanListener {
    pub fn results(&self) -> &HashMap<VertexCategories, Vec<JavaVertex>> {
        &self.vertexes
    }

    fn enter(&mut self, node: &JavaNode) {
        match node.get_node_type() {
            JavaNodeType::PackageDeclaration => self.analysis_package_declaration(node),
            JavaNodeType::ImportDeclaration => self.analysis_import_declaration(node),
            JavaNodeType::ClassDeclaration => self.analysis_class_declaration(node),
            JavaNodeType::InterfaceDeclaration => {},
            JavaNodeType::EnumDeclaration => {},
            JavaNodeType::FieldDeclaration => self.analysis_field_declaration(node),
            JavaNodeType::MethodDeclaration => self.analysis_method_declaration(node),
            JavaNodeType::AnnotationTypeDeclaration => {},
            JavaNodeType::ConstructorDeclaration => self.analysis_constructor(node),
            JavaNodeType::VariableDeclarators => {},
            JavaNodeType::Creator => self.analysis_creator(node),
            JavaNodeType::MethodCall => self.analysis_method_call(node),
            _ => ()
        }
    }

    fn exit(&mut self, node: &JavaNode) {
        let top = self.stack.top();
        if top.is_none() {
            return;
        }

        let ty = top.unwrap().get_type();
        if ty.is_none() {
            panic!("[ERROR] invalid top node of stack. vertex type not found: {:?}", top);
        }

        if let Some((category, vertex)) = match node.get_node_type() {
            JavaNodeType::PackageDeclaration => {
                if let VertexType::Package(_) = ty.unwrap() {
                    Some((VertexCategories::Package, self.stack.pop()))
                } else {
                    None
                }
            },
            JavaNodeType::ImportDeclaration => {
                if let VertexType::Import(_) = ty.unwrap() {
                    Some((VertexCategories::Imports, self.stack.pop()))
                } else {
                    None
                }
            }
            JavaNodeType::ClassDeclaration => {
                if let VertexType::Class(_, _, _, _, _, _, _) = ty.unwrap() {
                    Some((VertexCategories::Classes, self.stack.pop()))
                } else {
                    None
                } 
            },
            JavaNodeType::InterfaceDeclaration => None,
            JavaNodeType::EnumDeclaration => None,
            JavaNodeType::FieldDeclaration => {
                if let VertexType::Field(_, _, _, _, _, _) = ty.unwrap() {
                    Some((VertexCategories::Fields, self.stack.pop()))
                } else {
                    None
                }
            },
            JavaNodeType::MethodDeclaration => {
                if let VertexType::Method(_, _, _, _, _, _, _) = ty.unwrap() {
                    Some((VertexCategories::Methods, self.stack.pop()))
                } else {
                    None
                }
            },
            JavaNodeType::ConstructorDeclaration => {
                if let VertexType::Constructor(_, _, _, _, _) = ty.unwrap() {
                    Some((VertexCategories::Constructors, self.stack.pop()))
                } else {
                    None
                }
            },
            JavaNodeType::Creator => {
                if let VertexType::Creator(_, _, _) = ty.unwrap() {
                    Some((VertexCategories::CreatorCalls, self.stack.pop()))
                } else {
                    None
                }
            },
            JavaNodeType::MethodCall => {
                if let VertexType::MethodCall(_, _, _, _) = ty.unwrap() {
                    Some((VertexCategories::MethodCalls, self.stack.pop()))
                } else {
                    None
                }
            },
            _ => None
        } {
            if let Some(vertex) = vertex {
                self.add_vertext_to_parent(category, vertex.clone());
                self.add_vertex(category, vertex); 
            }
        }
    }
    
    fn analysis_package_declaration(&mut self, node: &JavaNode) {
        assert_eq!(node.get_node_type(), JavaNodeType::PackageDeclaration);
        for member in node.get_members() {
            match member.get_node_type() {
                JavaNodeType::QualifiedName => {
                    let mut idents: Vec<String> = Vec::new();
                    for ident in member.get_members() {
                        idents.push(ident.get_attr().as_ref().unwrap().to_string());
                    }
                    let ty = VertexType::Package(idents);
                    self.push_to_stack(JavaVertex::new(ty));
                },
                _ => ()
            }
        }
    }

    fn analysis_import_declaration(&mut self, node: &JavaNode) {
        assert_eq!(node.get_node_type(), JavaNodeType::ImportDeclaration);
        for member in node.get_members() {
            match member.get_node_type() {
                JavaNodeType::QualifiedName => {
                    let mut idents: Vec<String> = Vec::new();
                    for ident in member.get_members() {
                        idents.push(ident.get_attr().as_ref().unwrap().to_string());
                    }
                    let type_name = idents.swap_remove(idents.len() - 1);
                    let ty = VertexType::Import(TypeDescriptor {
                        package: idents, 
                        name: type_name
                    });
                    self.push_to_stack(JavaVertex::new(ty));
                },
                _ => ()
            }
        }
    }

    fn analysis_class_declaration(&mut self, node: &JavaNode) {
        assert_eq!(node.get_node_type(), JavaNodeType::ClassDeclaration);

        let mut annotations: Vec<String> = Vec::new();
        let mut modifiers: Vec<String> = Vec::new();
        let mut ident: Option<String> = None;
        let mut extends: Option<TypeDescriptor> = None;
        let mut implements: Vec<TypeDescriptor> = Vec::new();

        for member in node.get_members() {
            match member.get_node_type() {
                JavaNodeType::Annotation => if let Some(attr) = member.get_attr() {
                    annotations.push(attr.to_string());
                },
                JavaNodeType::Modifier => if let Some(attr) = member.get_attr() {
                    modifiers.push(attr.to_string());
                },
                JavaNodeType::Identifier => if let Some(attr) = member.get_attr() {
                    ident = Some(attr.to_string());
                },
                JavaNodeType::TypeType => if let Some(attr) = member.get_attr() {
                    extends = match self.get_dependency_full_type_name(attr.as_str()) {
                        Some(pn) => Some(TypeDescriptor {
                            package: pn.clone(), 
                            name: attr.to_string()
                        }),
                        None => Some(TypeDescriptor {
                            package: Vec::new(), 
                            name: attr.to_string()
                        })
                    };
                },
                JavaNodeType::TypeList => if let Some(attr) = member.get_attr() {
                    let implement = match self.get_dependency_full_type_name(attr.as_str()) {
                        Some(pn) => TypeDescriptor { package: pn.clone(), name: attr.to_string()},
                        None => TypeDescriptor {package: Vec::new(), name: attr.to_string()}
                    };
                    implements.push(implement)
                },
                _ => ()
            }
        }

        if let Some(package_name) = self.get_package_name() {
            let ty = match self.get_type_name() {
                Some(type_name) => VertexType::Class(package_name.clone(), type_name, annotations, modifiers, ident.unwrap(), extends, implements.clone()),
                None => VertexType::Class(package_name.clone(), Vec::new(), annotations, modifiers, ident.unwrap(), extends, implements.clone())
            };
            self.push_to_stack(JavaVertex::new(ty));
        }
    }

    fn analysis_field_declaration(&mut self, node: &JavaNode) {
        assert_eq!(node.get_node_type(), JavaNodeType::FieldDeclaration);

        let mut modifiers: Vec<String> = Vec::new();
        let mut ty: Option<TypeDescriptor> = None;
        let mut variable_id: Option<String> = None;
        let mut variable_init: Option<String> = None;

        for member in node.get_members() {
            match member.get_node_type() {
                JavaNodeType::Modifier => if let Some(attr) = member.get_attr() {
                    modifiers.push(attr.to_string());
                },
                JavaNodeType::TypeType => if let Some(attr) = member.get_attr() {
                    match self.get_dependency_full_type_name(attr.as_str()) {
                        Some(pp) => ty = Some(TypeDescriptor {package: pp.clone(), name: attr.to_string()}),
                        None => ty = Some(TypeDescriptor {package: Vec::new(), name: attr.to_string()})
                    }
                },
                JavaNodeType::VariableDeclarator => {
                    if member.get_members().len() == 0 {
                        variable_id = Some(member.get_attr().as_ref().unwrap().to_string());
                        continue;
                    }

                    for child in member.get_members() {
                        match child.get_node_type() {
                            JavaNodeType::VariableDeclaratorId => variable_id = Some(child.get_attr().as_ref().unwrap().to_string()),
                            JavaNodeType::VariableInitializer => if let Some(attr) = child.get_attr() {
                                variable_init = Some(attr.to_string());
                            }
                            _ => ()
                        }
                    }
                },
                _ => ()
            }
        }

        let package = match self.get_package_name() {
            Some(pn) => pn.clone(),
            None => Vec::new()
        };

        let ty = match self.get_type_name() {
            Some(type_name) => VertexType::Field(package, type_name, modifiers.clone(), ty, variable_id, variable_init),
            None => VertexType::Field(package, Vec::new(), modifiers.clone(), ty, variable_id, variable_init)
        };
        self.push_to_stack(JavaVertex::new(ty));
    }

    fn analysis_method_declaration(&mut self, node: &JavaNode) {
        assert_eq!(node.get_node_type(), JavaNodeType::MethodDeclaration);

        let mut annotation: Option<String> = None;
        let mut modifiers: Vec<String> = Vec::new();
        let mut ret_type: Option<TypeDescriptor> = None;
        let mut name: Option<String> = None;
        let mut params: Vec<ParamDescriptor> = Vec::new();

        for member in node.get_members() {
            match member.get_node_type() {
                JavaNodeType::Annotation => if let Some(attr) = member.get_attr() {
                    annotation = Some(attr.as_str().to_string());
                },
                JavaNodeType::Modifier => if let Some(attr) = member.get_attr() {
                    modifiers.push(attr.to_string());
                },
                JavaNodeType::TypeTypeOrVoid => if let Some(attr) = member.get_attr() {
                    let package = match self.get_dependency_full_type_name(attr.as_str()) {
                        Some(pn) => pn.clone(),
                        None => Vec::new()
                    };
                    ret_type = Some(TypeDescriptor { package: package, name: attr.to_string() });
                },
                JavaNodeType::Identifier => if let Some(attr) = member.get_attr() {
                    name = Some(attr.clone());
                },
                JavaNodeType::FormalParameters => for child in member.get_members() {
                    match child.get_node_type() {
                        JavaNodeType::FormalParameter => {
                            let mut modifiers: Vec<String> = Vec::new();
                            let mut ty: Option<TypeDescriptor> = None;
                            let mut ident: Option<String> = None;
                            for item in child.get_members() {
                                match item.get_node_type() {
                                    JavaNodeType::VariableModifier => modifiers.push(item.get_attr().as_ref().unwrap().to_string()),
                                    JavaNodeType::TypeType => {
                                        let name = item.get_attr().as_ref().unwrap().as_str();
                                        let package = match self.get_dependency_full_type_name(name) {
                                            Some(pn) => pn.clone(),
                                            None => Vec::new()
                                        };
                                        ty = Some(TypeDescriptor { package: package, name: name.to_string() })
                                    },
                                    JavaNodeType::VariableDeclaratorId => ident = Some(item.get_attr().as_ref().unwrap().to_string()),
                                    _ => ()
                                };
                            }
                            params.push(ParamDescriptor {
                                modifiers: modifiers, 
                                ty: ty.unwrap(), 
                                name: ident.unwrap().to_string()
                            });
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        }

        let package = match self.get_package_name() {
            Some(pn) => pn.clone(),
            None => Vec::new()
        };

        let ty = match self.get_type_name() {
            Some(type_name) => VertexType::Method(package, type_name, annotation, modifiers, ret_type.unwrap(), name.unwrap().to_string(), params),
            None => VertexType::Method(package, Vec::new(), annotation, modifiers, ret_type.unwrap(), name.unwrap().to_string(), params),
        };

        self.push_to_stack(JavaVertex::new(ty));
    }

    fn analysis_method_call(&mut self, node: &JavaNode) {
        assert_eq!(node.get_node_type(), JavaNodeType::MethodCall);

        let mut cast: Option<String> = None;
        let mut idents: Vec<String> = Vec::new();
        let mut params: Vec<String> = Vec::new();

        for child in node.get_members() {
            match child.get_node_type() {
                JavaNodeType::TypeType => cast = Some(child.get_attr().as_ref().unwrap().to_string()),
                JavaNodeType::Identifier => idents.push(child.get_attr().as_ref().unwrap().to_string()),
                JavaNodeType::ExpressionList => if child.get_members().len() > 0 {
                    for param_node in child.get_members() {
                        if param_node.get_node_type() != JavaNodeType::Separator {
                            params.push(param_node.get_attr().as_ref().unwrap().to_string());
                        }
                    }
                } else {
                    params.push(child.get_attr().as_ref().unwrap().to_string());
                },
                _ => ()
            }
        }

        // debug
        if idents.len() <= 0 {
            println!("= DUMP ==\n{}\n=========", node.dump().unwrap());
        }
        // debug end

        assert!(idents.len() > 0);
        let ident = idents.swap_remove(idents.len() - 1);
        let ty = VertexType::MethodCall(cast, Some(idents.join(".")), ident, params);
        self.push_to_stack(JavaVertex::new(ty));
    }

    fn analysis_creator(&mut self, node: &JavaNode) {
        assert_eq!(node.get_node_type(), JavaNodeType::Creator);

        let mut creator_name: Vec<String> = Vec::new();
        let mut rests: Vec<String> = Vec::new();

        for child in node.get_members() {
            match child.get_node_type() {
                JavaNodeType::CreatedName => for ident in child.get_members() {
                    creator_name.push(ident.get_attr().as_ref().unwrap().to_string())
                },
                JavaNodeType::ClassCreatorRest => if let Some(arguments) = child.get_members().front() {
                    assert_eq!(arguments.get_node_type(), JavaNodeType::Arguments);
                    if arguments.get_members().len() > 0 {
                        for arg in arguments.get_members() {
                            rests.push(arg.get_attr().as_ref().unwrap().to_string());
                        }
                    }
                },
                JavaNodeType::ArrayCreatorRest => if child.get_members().len() > 0 {
                    for item in child.get_members() {
                        rests.push(item.get_attr().as_ref().unwrap().to_string());
                    }
                },
                _ => ()
            }
        }

        let package = match self.get_dependency_full_type_name(creator_name.get(0).unwrap()) {
            Some(p) => p.clone(),
            None => Vec::new()
        };

        let ty = VertexType::Creator(package, creator_name, rests);
        self.push_to_stack(JavaVertex::new(ty));
    }

    fn analysis_constructor(&mut self, node: &JavaNode) {
        assert_eq!(node.get_node_type(), JavaNodeType::ConstructorDeclaration);

        let mut modifiers: Vec<String> = Vec::new();
        let mut ident: Option<String> = None;
        let mut params: Vec<ParamDescriptor> = Vec::new();

        for child in node.get_members() {
            match child.get_node_type() {
                JavaNodeType::Modifier => modifiers.push(child.get_attr().as_ref().unwrap().to_string()),
                JavaNodeType::Identifier => ident = Some(child.get_attr().as_ref().unwrap().to_string()),
                JavaNodeType::FormalParameters => if child.get_members().len() > 0 {
                    for param in child.get_members() {
                        match param.get_node_type() {
                            JavaNodeType::FormalParameter => {
                                let mut param_modifiers: Vec<String> = Vec::new();
                                let mut ty: Option<TypeDescriptor> = None;
                                let mut name: Option<String> = None;

                                for part in param.get_members() {
                                    match part.get_node_type() {
                                        JavaNodeType::VariableModifier => param_modifiers.push(part.get_attr().as_ref().unwrap().to_string()),
                                        JavaNodeType::TypeType => {
                                            let type_name = part.get_attr().as_ref().unwrap().as_str();
                                            let package_name = match self.get_dependency_full_type_name(type_name) {
                                                Some(pn) => pn.clone(),
                                                None => Vec::new()
                                            };
                                            ty = Some(TypeDescriptor {
                                                package: package_name,
                                                name: type_name.to_string()
                                            });
                                        },
                                        JavaNodeType::VariableDeclaratorId => name = Some(part.get_attr().as_ref().unwrap().to_string()),
                                        _ => ()
                                    }
                                }
                                params.push(ParamDescriptor {
                                    modifiers: param_modifiers, 
                                    ty: ty.unwrap(), 
                                    name: name.unwrap()
                                });
                            },
                            _ => ()
                        }
                    }
                },
                _ => ()
            }
        }

        let package = match self.get_package_name() {
            Some(pn) => pn.clone(),
            None => Vec::new()
        };

        let ty = match self.get_type_name() {
            Some(type_name) => VertexType::Constructor(package, type_name, modifiers, ident.unwrap().to_string(), params),
            None => VertexType::Constructor(package, Vec::new(), modifiers, ident.unwrap().to_string(), params),
        };

        self.push_to_stack(JavaVertex::new(ty));
    }

    fn add_vertex(&mut self, category: VertexCategories, vertex: JavaVertex) {
        match self.vertexes.get_mut(&category) {
            Some(vertexes) => _ = vertexes.push(vertex),
            None => _ = self.vertexes.insert(category, vec![vertex]),
        }
    }

    fn push_to_stack(&mut self, vertex: JavaVertex) {
        self.stack.push(vertex);
    }

    fn add_vertext_to_parent(&mut self, category: VertexCategories, vertex: JavaVertex) {
        if self.stack.len() <= 0 {
            return;
        }

        if let Some(top) = self.stack.top_mut() {
            top.add_member(category, vertex);
        }
    }

    fn get_package_name(&self) -> Option<&Vec<String>> {
        if let Some(packages) = self.vertexes.get(VertexCategories::Package.as_ref()) {
            if let VertexType::Package(package_name) = packages.get(0).unwrap().get_type().unwrap() {
                return Some(package_name)
            }
        }
        panic!("[ERROR]: package not found");
    }

    fn get_type_name(&self) -> Option<Vec<String>> {
        if self.stack.empty() {
            return None;
        }

        let mut parents: Vec<String> = Vec::new();
        for index in 0..self.stack.len() {
            if let Some(jv) = self.stack.get_by_index(index) {
                if let VertexType::Class(_, _, _, _, name, _, _) = jv.get_type().unwrap() {
                    parents.push(name.to_string())
                }
            }
        }

        Some(parents)
    }

    fn get_dependency_full_type_name(&self, short_name: &str) -> Option<&Vec<String>> {
        if let Some(imports) = self.vertexes.get(&VertexCategories::Imports) {
            for import in imports {
                if let Some(ty) = import.get_type() {
                    if let VertexType::Import(package) = ty {
                        if package.name.as_str() == short_name {
                            return Some(&package.package)
                        }
                    }
                }
            }
        }
        None
    }
}

pub struct JavaDataCleanPolicy {}

impl<'a> JavaDataCleanPolicy {
    pub fn analysis(&mut self, node: &JavaNode) -> LanguageDataCleanResult<HashMap<VertexCategories, Vec<JavaVertex>>> {
        assert_eq!(node.get_node_type(), JavaNodeType::File);

        let mut listener = JavaDataCleanListener{
            vertexes: HashMap::new(),
            stack: Stack::new(),
        };
        JavaDataCleanPolicy::tree_walker(&node, &mut listener);

        Ok(listener.results().clone())
    }

    fn tree_walker(node: &JavaNode, listener: &mut JavaDataCleanListener) {
        listener.enter(&node);
        for child in node.get_members() {
            JavaDataCleanPolicy::tree_walker(child, listener);
        }
        listener.exit(&node);
    }
}

impl LanguageDataCleanPolicy for JavaDataCleanPolicy {
    fn new() -> Self {
        JavaDataCleanPolicy {}
    }

    fn execute(&mut self, metadata: &PathBuf, output: &PathBuf) -> LanguageDataCleanResult<PathBuf> {
        if metadata.is_file() && metadata.exists() {
            let context_str = fs::read_to_string(metadata).expect("should read context of file");

            // solved following issue:
            //   Error("recursion limit exceeded", line: 1, column: 271738)
            // refer to : https://github.com/lovasoa/bad_json_parsers/issues/7
            //   or comments of disable_recursion_limit method in 
            //     .rust/cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/serde_json-1.0.93/src/de.rs
            let mut deserializer = serde_json::Deserializer::from_str(&context_str);
            deserializer.disable_recursion_limit();
            let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
            let v = Value::deserialize(deserializer).unwrap();
            let context: JavaNode = serde_json::from_value(v).unwrap();

            if let Ok(jv) = self.analysis(&context) {
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
        Err(LanguageDataCleanPolicyError {})
    }
}
