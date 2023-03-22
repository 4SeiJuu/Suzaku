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
    stack::Stack,
    VERTEX_FILE_EXTENSION, 
};

use crate::java::node_type::JavaNodeType;

use super::{
    vertex::JavaVertex, 
    node::JavaNode
};

pub struct JavaAnalysisListener {
    // TODO: the JavaVertex should be stored as ref in vertexes.
    vertexes: HashMap<VertexRelationship, Vec<JavaVertex>>,
    stack: Stack<JavaVertex>
}

impl JavaAnalysisListener {
    pub fn results(&self) -> &HashMap<VertexRelationship, Vec<JavaVertex>> {
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

    fn exit(&mut self, node: &JavaNode) {
        let top = self.stack.top();
        if top.is_none() {
            return;
        }

        let jv = top.unwrap().get_type();
        if jv.is_none() {
            panic!("[ERROR] invalid top node of stack. vertex type not found: {:?}", top);
        }

        match node.get_node_type() {
            JavaNodeType::ClassDeclaration => if let VertexType::Class(_, _, _, _, _) = jv.unwrap() {
                self.stack.pop();
            },
            JavaNodeType::InterfaceDeclaration => {},
            JavaNodeType::EnumDeclaration => {},
            JavaNodeType::MethodDeclaration => {},
            JavaNodeType::ConstructorDeclaration => {},
            JavaNodeType::Creator => {},
            JavaNodeType::MethodCall => {},
            _ => () // println!("[WARNING] invalid top node of stack. expected: {:?}, actual: {:?}", node.get_node_type(), jv)
        }
    }
    
    fn analysis_package_declaration(&mut self, node: &JavaNode) {
        assert_eq!(node.get_node_type(), JavaNodeType::PackageDeclaration);
        for member in node.get_members() {
            match member.get_node_type() {
                JavaNodeType::QualifiedName => self.add_vertex(VertexRelationship::Package, 
                    JavaVertex::new(VertexType::Package(String::from(member.get_attr().as_ref().unwrap().as_str())).as_ref())),
                _ => ()
            }
        }
    }

    fn analysis_import_declaration(&mut self, node: &JavaNode) {
        assert_eq!(node.get_node_type(), JavaNodeType::ImportDeclaration);
        for member in node.get_members() {
            match member.get_node_type() {
                JavaNodeType::QualifiedName => if let Some((pkg, ty)) = member.get_attr().as_ref().unwrap().as_str().rsplit_once('.') {
                    self.add_vertex(VertexRelationship::Imports, 
                        JavaVertex::new(VertexType::Import(String::from(pkg), String::from(ty)).as_ref()));
                },
                _ => ()
            }
        }
    }

    fn analysis_class_declaration(&mut self, node: &JavaNode) {
        assert_eq!(node.get_node_type(), JavaNodeType::ClassDeclaration);

        let mut annotations: Vec<String> = Vec::new();
        let mut modifiers: Vec<&str> = Vec::new();
        let mut ident: Option<&str> = None;
        let mut extends: Option<String> = None;
        let mut implements: Vec<String> = Vec::new();

        for member in node.get_members() {
            match member.get_node_type() {
                JavaNodeType::Annotation => if let Some(attr) = member.get_attr() {
                    annotations.push(attr.to_string());
                },
                JavaNodeType::Modifier => if let Some(attr) = member.get_attr() {
                    modifiers.push(attr.as_str());
                },
                JavaNodeType::Identifier => if let Some(attr) = member.get_attr() {
                    ident = Some(attr.as_str());
                },
                JavaNodeType::TypeType => if let Some(attr) = member.get_attr() {
                    extends = Some(attr.to_string());
                },
                JavaNodeType::TypeList => if let Some(attr) = member.get_attr() {
                    implements.push(attr.to_string())
                },
                _ => ()
            }
        }

        if let Some(package_name) = self.get_package_name() {
            let vertex_type = VertexType::Class(annotations, package_name.to_string(), ident.unwrap().to_string(), extends, implements.clone());

            // TODO: after JavaVertex changed to ref in vertexes, following code should be changed.
            let jv = JavaVertex::new(vertex_type.as_ref());
            self.add_vertex(VertexRelationship::Class, jv.clone());
            self.stack.push(jv);
        }
    }

    fn analysis_field_declaration(&mut self, node: &JavaNode) {
        assert_eq!(node.get_node_type(), JavaNodeType::FieldDeclaration);

        let mut modifiers: Vec<String> = Vec::new();
        let mut ty: Option<String> = None;
        let mut variable_id: Option<String> = None;
        let mut variable_init: Option<String> = None;

        for member in node.get_members() {
            match member.get_node_type() {
                JavaNodeType::Modifier => if let Some(attr) = member.get_attr() {
                    modifiers.push(attr.to_string());
                },
                JavaNodeType::TypeType => if let Some(attr) = member.get_attr() {
                    ty = Some(attr.to_string());
                },
                JavaNodeType::VariableDeclarators => {
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
                            _ => () // println!("[WARNING] not produced node: {}", child.dump().unwrap())
                        }
                    }
                },
                _ => ()
            }
        }

        if let Some(package_name) = self.get_package_name() {
            if let Some(type_name) = self.get_type_name() {
                let vertex_type = VertexType::Field(package_name.to_string(), type_name.to_string(), modifiers.clone(), ty, variable_id, variable_init);
                self.add_vertex(VertexRelationship::Fields, JavaVertex::new(vertex_type.as_ref()));
            }
        }
    }

    fn analysis_method_declaration(&mut self, node: &JavaNode) {
        assert_eq!(node.get_node_type(), JavaNodeType::MethodDeclaration);

        let mut annotation: Option<String> = None;
        let mut modifiers: Vec<String> = Vec::new();
        let mut ret_type: Option<&str> = None;
        let mut name: Option<&str> = None;
        let mut params: Vec<(Option<String>, String, String)> = Vec::new();

        for member in node.get_members() {
            match member.get_node_type() {
                JavaNodeType::Annotation => if let Some(attr) = member.get_attr() {
                    annotation = Some(attr.as_str().to_string());
                },
                JavaNodeType::Modifier => if let Some(attr) = member.get_attr() {
                    modifiers.push(attr.to_string());
                },
                JavaNodeType::TypeTypeOrVoid => if let Some(attr) = member.get_attr() {
                    ret_type = Some(attr.as_str());
                },
                JavaNodeType::Identifier => if let Some(attr) = member.get_attr() {
                    name = Some(attr.as_str());
                },
                JavaNodeType::FormalParameters => for child in member.get_members() {
                    match child.get_node_type() {
                        JavaNodeType::FormalParameter => {
                            let mut modifier: Option<String> = None;
                            let mut ty: Option<String> = None;
                            let mut ident: Option<String> = None;
                            for item in child.get_members() {
                                match item.get_node_type() {
                                    JavaNodeType::VariableModifier => modifier = Some(item.get_attr().as_ref().unwrap().to_string()),
                                    JavaNodeType::TypeType => ty = Some(item.get_attr().as_ref().unwrap().to_string()),
                                    JavaNodeType::VariableDeclaratorId => ident = Some(item.get_attr().as_ref().unwrap().to_string()),
                                    _ => ()
                                };
                            }
                            params.push((modifier, ty.unwrap().to_string(), ident.unwrap().to_string()));
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        }

        if let Some(package_name) = self.get_package_name() {
            if let Some(type_name) = self.get_type_name() {
                let vertex_type = VertexType::Method(package_name.to_string(), type_name.to_string(), annotation, modifiers, ret_type.unwrap().to_string(), name.unwrap().to_string(), params);
                self.add_vertex(VertexRelationship::Methods, JavaVertex::new(vertex_type.as_ref()));
            }
        }
    }

    fn add_vertex(&mut self, relationship: VertexRelationship, vertex: JavaVertex) {
        match self.vertexes.get_mut(&relationship) {
            Some(vertexes) => _ = vertexes.push(vertex),
            None => _ = self.vertexes.insert(relationship, vec![vertex]),
        }
    }

    fn add_vertext_to_parent(&mut self, relationship: VertexRelationship, vertex: JavaVertex) {
        if let Some(top) = self.stack.top_mut() {
            // match top.
        }
    }

    fn get_package_name(&self) -> Option<&String> {
        if let Some(packages) = self.vertexes.get(VertexRelationship::Package.as_ref()) {
            if let VertexType::Package(package_name) = packages.get(0).unwrap().get_type().unwrap() {
                return Some(package_name)
            }
        }
        panic!("[ERROR]: package not found");
    }

    fn get_type_name(&self) -> Option<&String> {
        if let Some(jv) = self.stack.top() {
            if let VertexType::Class(_, _, name, _, _) = jv.get_type().unwrap() {
                return Some(name);
            }
        }
        panic!("[ERROR] type name not found");
    }
}

pub struct JavaAnalysisPolicy {}

impl<'a> JavaAnalysisPolicy {
    pub fn analysis(&mut self, node: &JavaNode) -> LanguageAnalysisResult<HashMap<VertexRelationship, Vec<JavaVertex>>> {
        assert_eq!(node.get_node_type(), JavaNodeType::File);

        let mut listener = JavaAnalysisListener{
            vertexes: HashMap::new(),
            stack: Stack::new(),
        };
        JavaAnalysisPolicy::tree_walker(&node, &mut listener);

        Ok(listener.results().clone())
    }

    fn tree_walker(node: &JavaNode, listener: &mut JavaAnalysisListener) {
        listener.enter(&node);
        for child in node.get_members() {
            JavaAnalysisPolicy::tree_walker(child, listener);
        }
        listener.exit(&node);
    }
}

impl LanguageAnalysisPolicy for JavaAnalysisPolicy {
    fn new() -> Self {
        JavaAnalysisPolicy {}
    }

    fn execute(&mut self, metadata: &PathBuf, output: &PathBuf) -> LanguageAnalysisResult<PathBuf> {
        if metadata.is_file() && metadata.exists() {
            let context_str = fs::read_to_string(metadata).expect("should read context of file");
            let context: JavaNode = serde_json::from_str(&context_str).expect("failed to convert metadata to hashmap");

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
        Err(LanguageAnalysisPolicyError {})
    }
}
