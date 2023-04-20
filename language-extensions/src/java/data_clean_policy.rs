use std::{
    path::PathBuf, 
    collections::HashMap, 
    fs::{
        self, 
        File
    }, 
    io::Write,
};

use regex::Regex;
use serde::Deserialize;
use serde_json::Value;

use suzaku_extension_sdk::{
    extractor::{
        LanguageDataExtractorPolicy,
        LanguageDataExtractorPolicyError,
        LanguageDataExtractorResult
    }, 
    meta::{
        IMeta, 
        Metadata,
    },
    meta_type::MetaType,
    element::{
        IElement,
        Elements,
        ElementCategories, 
        TypeDescriptor, 
        ParamDescriptor, 
        Caller
    },
    stack::Stack,
    ELEMENT_FILE_EXTENSION, 
    utils, 
};

use super::{
    element::JavaElement
};

pub struct JavaDataCleanListener {
    elements: HashMap<ElementCategories, Vec<JavaElement>>,
    stack: Stack<JavaElement>
}

impl JavaDataCleanListener {
    pub fn results(&self) -> HashMap<ElementCategories, Vec<JavaElement>> {
        self.elements.clone()
    }

    fn enter(&mut self, node: &Metadata) {
        match node.get_node_type() {
            MetaType::PackageDeclaration => self.analysis_package_declaration(node),
            MetaType::ImportDeclaration => self.analysis_import_declaration(node),
            MetaType::ClassDeclaration => self.analysis_class_declaration(node),
            MetaType::InterfaceDeclaration => self.analysis_interface_declaration(node),
            MetaType::EnumDeclaration => {},
            MetaType::FieldDeclaration => self.analysis_field_declaration(node),
            MetaType::MethodDeclaration | MetaType::InterfaceMethodDeclaration => self.analysis_method_declaration(node),
            MetaType::AnnotationTypeDeclaration => {},
            MetaType::ConstructorDeclaration => self.analysis_constructor(node),
            MetaType::VariableDeclarators => {},
            MetaType::Creator => self.analysis_creator(node),
            MetaType::MethodCall => self.analysis_method_call(node),
            _ => ()
        }
    }

    fn exit(&mut self, node: &Metadata) {
        let top = self.stack.top();
        if top.is_none() {
            return;
        }

        let ty = top.unwrap().get_type();
        if ty.is_none() {
            panic!("[ERROR] invalid top node of stack. element type not found: {:?}", top);
        }

        if let Some((category, element)) = match node.get_node_type() {
            MetaType::PackageDeclaration => {
                if let Elements::Package(_) = ty.unwrap() {
                    Some((ElementCategories::Package, self.stack.pop()))
                } else {
                    None
                }
            },
            MetaType::ImportDeclaration => {
                if let Elements::Import(_) = ty.unwrap() {
                    Some((ElementCategories::Imports, self.stack.pop()))
                } else {
                    None
                }
            }
            MetaType::ClassDeclaration => {
                if let Elements::Class(_, _, _, _, _, _) = ty.unwrap() {
                    Some((ElementCategories::Classes, self.stack.pop()))
                } else {
                    None
                } 
            },
            MetaType::InterfaceDeclaration => {
                if let Elements::Interface(_, _, _, _, _) = ty.unwrap() {
                    Some((ElementCategories::Interfaces, self.stack.pop()))
                } else {
                    None
                } 
            },
            MetaType::EnumDeclaration => None,
            MetaType::FieldDeclaration => {
                if let Elements::Field(_, _, _, _, _) = ty.unwrap() {
                    Some((ElementCategories::Fields, self.stack.pop()))
                } else {
                    None
                }
            },
            MetaType::MethodDeclaration | MetaType::InterfaceMethodDeclaration => {
                if let Elements::Method(_, _, _, _, _, _) = ty.unwrap() {
                    Some((ElementCategories::Methods, self.stack.pop()))
                } else {
                    None
                }
            },
            MetaType::ConstructorDeclaration => {
                if let Elements::Constructor(_, _, _, _) = ty.unwrap() {
                    Some((ElementCategories::Constructors, self.stack.pop()))
                } else {
                    None
                }
            },
            MetaType::Creator => {
                if let Elements::CreatorCall(_, _) = ty.unwrap() {
                    Some((ElementCategories::CreatorCalls, self.stack.pop()))
                } else {
                    None
                }
            },
            MetaType::MethodCall => {
                if let Elements::MethodCall(_, _, _, _) = ty.unwrap() {
                    Some((ElementCategories::MethodCalls, self.stack.pop()))
                } else {
                    None
                }
            },
            _ => None
        } {
            if let Some(element) = element {
                if category == ElementCategories::Classes || category == ElementCategories::Interfaces {
                    self.add_element(category, element)
                } else {
                    self.add_element_to_parent(category, element.clone());
                    if self.stack.empty() {
                        self.add_element(category, element)
                    }
                }
            }
        }
    }
    
    fn analysis_package_declaration(&mut self, node: &Metadata) {
        assert_eq!(node.get_node_type(), MetaType::PackageDeclaration);
        for member in node.get_members() {
            match member.get_node_type() {
                MetaType::QualifiedName => {
                    let mut idents: Vec<String> = Vec::new();
                    for ident in member.get_members() {
                        idents.push(ident.get_attr().as_ref().unwrap().to_string());
                    }
                    let ty = Elements::Package(idents);
                    self.push_to_stack(JavaElement::new(ty));
                },
                _ => ()
            }
        }
    }

    fn analysis_import_declaration(&mut self, node: &Metadata) {
        assert_eq!(node.get_node_type(), MetaType::ImportDeclaration);
        for member in node.get_members() {
            match member.get_node_type() {
                MetaType::QualifiedName => {
                    let mut idents: Vec<String> = Vec::new();
                    for ident in member.get_members() {
                        idents.push(ident.get_attr().as_ref().unwrap().to_string());
                    }
                    let type_name = idents.swap_remove(idents.len() - 1);
                    let ty = Elements::Import(TypeDescriptor {
                        package: idents, 
                        name: vec![type_name]
                    });
                    self.push_to_stack(JavaElement::new(ty));
                },
                _ => ()
            }
        }
    }

    fn analysis_class_declaration(&mut self, node: &Metadata) {
        assert_eq!(node.get_node_type(), MetaType::ClassDeclaration);

        let mut annotations: Vec<String> = Vec::new();
        let mut modifiers: Vec<String> = Vec::new();
        let mut ident: Option<String> = None;
        let mut extends: Vec<TypeDescriptor> = Vec::new();
        let mut implements: Vec<TypeDescriptor> = Vec::new();

        for member in node.get_members() {
            match member.get_node_type() {
                MetaType::Annotation => if let Some(attr) = member.get_attr() {
                    annotations.push(attr.to_string());
                },
                MetaType::Modifier => if let Some(attr) = member.get_attr() {
                    modifiers.push(attr.to_string());
                },
                MetaType::Identifier => if let Some(attr) = member.get_attr() {
                    ident = Some(attr.to_string());
                },
                MetaType::TypeType => if let Some(attr) = member.get_attr() {
                    if let Some(extend) = match self.get_dependency_type_info_from_imports(attr.as_str()) {
                        Some(td) => Some(TypeDescriptor {
                            package: td.package.clone(), 
                            name: vec![attr.to_string()]
                        }),
                        None => Some(TypeDescriptor {
                            package: Vec::new(), 
                            name: vec![attr.to_string()]
                        })
                    } {
                        extends.push(extend);
                    }
                },
                MetaType::TypeList => if let Some(attr) = member.get_attr() {
                    let implement = match self.get_dependency_type_info_from_imports(attr.as_str()) {
                        Some(td) => TypeDescriptor { package: td.package.clone(), name: vec![attr.to_string()]},
                        None => TypeDescriptor {package: Vec::new(), name: vec![attr.to_string()]}
                    };
                    implements.push(implement)
                },
                _ => ()
            }
        }

        if let Some(package_name) = self.get_package_name() {
            let ty = match self.get_type_name() {
                Some(type_name) => Elements::Class(
                        TypeDescriptor { package: package_name.clone(), name: type_name }, annotations, modifiers, ident.unwrap(), extends, implements.clone()),
                None => Elements::Class(
                    TypeDescriptor { package: package_name.clone(), name: Vec::new() }, annotations, modifiers, ident.unwrap(), extends, implements.clone())
            };
            self.push_to_stack(JavaElement::new(ty));
        }
    }

    fn analysis_interface_declaration(&mut self, node: &Metadata) {
        assert_eq!(node.get_node_type(), MetaType::InterfaceDeclaration);

        let mut annotations: Vec<String> = Vec::new();
        let mut modifiers: Vec<String> = Vec::new();
        let mut ident: Option<String> = None;
        let mut extends: Vec<TypeDescriptor> = Vec::new();

        for member in node.get_members() {
            match member.get_node_type() {
                MetaType::Annotation => if let Some(attr) = member.get_attr() {
                    annotations.push(attr.to_string());
                },
                MetaType::Modifier => if let Some(attr) = member.get_attr() {
                    modifiers.push(attr.to_string());
                },
                MetaType::Identifier => if let Some(attr) = member.get_attr() {
                    ident = Some(attr.to_string());
                },
                MetaType::TypeType => if let Some(attr) = member.get_attr() {
                    extends.push(match self.get_dependency_type_info_from_imports(attr.as_str()) {
                        Some(td) => TypeDescriptor {
                            package: td.package.clone(), 
                            name: vec![attr.to_string()]
                        },
                        None => TypeDescriptor {
                            package: Vec::new(), 
                            name: vec![attr.to_string()]
                        }
                    });
                },
                _ => ()
            }
        }

        if let Some(package_name) = self.get_package_name() {
            let ty = match self.get_type_name() {
                Some(type_name) => Elements::Interface(
                        TypeDescriptor { package: package_name.clone(), name: type_name }, annotations, modifiers, ident.unwrap(), extends),
                None => Elements::Interface(
                    TypeDescriptor { package: package_name.clone(), name: Vec::new() }, annotations, modifiers, ident.unwrap(), extends)
            };
            self.push_to_stack(JavaElement::new(ty));
        }
    }

    fn analysis_field_declaration(&mut self, node: &Metadata) {
        assert_eq!(node.get_node_type(), MetaType::FieldDeclaration);

        let mut modifiers: Vec<String> = Vec::new();
        let mut ty: Option<TypeDescriptor> = None;
        let mut variable_id: Option<String> = None;
        let mut variable_init: Option<String> = None;

        for member in node.get_members() {
            match member.get_node_type() {
                MetaType::Modifier => if let Some(attr) = member.get_attr() {
                    modifiers.push(attr.to_string());
                },
                MetaType::TypeType => if let Some(attr) = member.get_attr() {
                    match self.get_dependency_type_info_from_imports(attr.as_str()) {
                        Some(td) => ty = Some(TypeDescriptor {package: td.package.clone(), name: vec![attr.to_string()]}),
                        None => ty = Some(TypeDescriptor {package: Vec::new(), name: vec![attr.to_string()]})
                    }
                },
                MetaType::VariableDeclarator => {
                    if member.get_members().len() == 0 {
                        variable_id = Some(member.get_attr().as_ref().unwrap().to_string());
                        continue;
                    }

                    for child in member.get_members() {
                        match child.get_node_type() {
                            MetaType::VariableDeclaratorId => variable_id = Some(child.get_attr().as_ref().unwrap().to_string()),
                            MetaType::VariableInitializer => if let Some(attr) = child.get_attr() {
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
            Some(type_name) => Elements::Field(TypeDescriptor { package: package, name: type_name }, modifiers.clone(), ty, variable_id, variable_init),
            None => Elements::Field(TypeDescriptor { package: package, name: Vec::new() }, modifiers.clone(), ty, variable_id, variable_init)
        };

        self.push_to_stack(JavaElement::new(ty));
    }

    fn analysis_method_declaration(&mut self, node: &Metadata) {
        assert!(node.get_node_type() == MetaType::MethodDeclaration || node.get_node_type() == MetaType::InterfaceMethodDeclaration);

        let mut annotation: Option<String> = None;
        let mut modifiers: Vec<String> = Vec::new();
        let mut ret_type: Option<TypeDescriptor> = None;
        let mut name: Option<String> = None;
        let mut params: Vec<ParamDescriptor> = Vec::new();

        for member in node.get_members() {
            match member.get_node_type() {
                MetaType::Annotation => if let Some(attr) = member.get_attr() {
                    annotation = Some(attr.as_str().to_string());
                },
                MetaType::Modifier => if let Some(attr) = member.get_attr() {
                    modifiers.push(attr.to_string());
                },
                MetaType::TypeTypeOrVoid => if let Some(attr) = member.get_attr() {
                    let package = match self.get_dependency_type_info_from_imports(attr.as_str()) {
                        Some(td) => td.package.clone(),
                        None => Vec::new()
                    };
                    ret_type = Some(TypeDescriptor { package: package, name: vec![attr.to_string()] });
                },
                MetaType::Identifier => if let Some(attr) = member.get_attr() {
                    name = Some(attr.clone());
                },
                MetaType::FormalParameters => for child in member.get_members() {
                    match child.get_node_type() {
                        MetaType::FormalParameter => {
                            let mut modifiers: Vec<String> = Vec::new();
                            let mut ty: Option<TypeDescriptor> = None;
                            let mut ident: Option<String> = None;
                            for item in child.get_members() {
                                match item.get_node_type() {
                                    MetaType::VariableModifier => modifiers.push(item.get_attr().as_ref().unwrap().to_string()),
                                    MetaType::TypeType => {
                                        let mut idents: Vec<String> = Vec::new();
                                        for member in item.get_members() {
                                            match member.get_node_type() {
                                                MetaType::Identifier | MetaType::TypeIdentifier | MetaType::ClassOrInterfaceType | MetaType::PrimitiveType => 
                                                    idents.push(member.get_attr().as_ref().unwrap().clone()),
                                                _ => ()
                                            }
                                        }

                                        let package = match self.get_dependency_type_info_from_imports(&idents.first().unwrap().as_str()) {
                                            Some(td) => td.package.clone(),
                                            None => Vec::new()
                                        };
                                        ty = Some(TypeDescriptor { package: package, name: idents })
                                    },
                                    MetaType::VariableDeclaratorId => ident = Some(item.get_attr().as_ref().unwrap().to_string()),
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
            Some(type_name) => Elements::Method(TypeDescriptor { package: package, name: type_name }, annotation, modifiers, ret_type.unwrap(), name.unwrap().to_string(), params),
            None => Elements::Method(TypeDescriptor { package: package, name: Vec::new() }, annotation, modifiers, ret_type.unwrap(), name.unwrap().to_string(), params),
        };

        self.push_to_stack(JavaElement::new(ty));
    }

    fn analysis_method_call(&mut self, node: &Metadata) {
        assert_eq!(node.get_node_type(), MetaType::MethodCall);

        let mut cast: Option<String> = None;
        let mut idents: Vec<String> = Vec::new();
        let mut params: Vec<String> = Vec::new();

        for child in node.get_members() {
            match child.get_node_type() {
                MetaType::TypeType => cast = Some(child.get_attr().as_ref().unwrap().to_string()),
                MetaType::Identifier => idents.push(child.get_attr().as_ref().unwrap().to_string()),
                MetaType::ExpressionList => if child.get_members().len() > 0 {
                    for param_node in child.get_members() {
                        if param_node.get_node_type() != MetaType::Separator {
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
        let mut caller: Option<Caller> = None;
        if idents.is_empty() {
            caller = Some(Caller {
                ty: TypeDescriptor {
                    package: match self.get_package_name() {
                        Some(pkg) => pkg.clone(),
                        None => Vec::new()
                    },
                    name: match self.get_type_name() {
                        Some(name) => name,
                        None => Vec::new()
                    }
                },
                name: Some(String::from("self"))
            })
        } else {
            caller = self.get_caller(idents);
        }

        if caller.is_none() {
            caller = Some(Caller {
                ty: TypeDescriptor { package: Vec::new(), name: Vec::new() },
                name: None
            })
        }
        let ty = Elements::MethodCall(cast, caller.unwrap(), ident, params);
        self.push_to_stack(JavaElement::new(ty));
    }

    fn analysis_creator(&mut self, node: &Metadata) {
        assert_eq!(node.get_node_type(), MetaType::Creator);

        let mut creator_name: Vec<String> = Vec::new();
        let mut rests: Vec<String> = Vec::new();

        for child in node.get_members() {
            match child.get_node_type() {
                MetaType::CreatedName => for ident in child.get_members() {
                    creator_name.push(ident.get_attr().as_ref().unwrap().to_string())
                },
                MetaType::ClassCreatorRest => if let Some(arguments) = child.get_members().front() {
                    assert_eq!(arguments.get_node_type(), MetaType::Arguments);
                    if arguments.get_members().len() > 0 {
                        for arg in arguments.get_members() {
                            rests.push(arg.get_attr().as_ref().unwrap().to_string());
                        }
                    }
                },
                MetaType::ArrayCreatorRest => if child.get_members().len() > 0 {
                    for item in child.get_members() {
                        rests.push(item.get_attr().as_ref().unwrap().to_string());
                    }
                },
                _ => ()
            }
        }

        let package = match self.get_dependency_type_info_from_imports(creator_name.get(0).unwrap()) {
            Some(td) => td.package.clone(),
            None => Vec::new()
        };

        let ty = Elements::CreatorCall(TypeDescriptor { package: package, name: creator_name }, rests);
        self.push_to_stack(JavaElement::new(ty));
    }

    fn analysis_constructor(&mut self, node: &Metadata) {
        assert_eq!(node.get_node_type(), MetaType::ConstructorDeclaration);

        let mut modifiers: Vec<String> = Vec::new();
        let mut ident: Option<String> = None;
        let mut params: Vec<ParamDescriptor> = Vec::new();

        for child in node.get_members() {
            match child.get_node_type() {
                MetaType::Modifier => modifiers.push(child.get_attr().as_ref().unwrap().to_string()),
                MetaType::Identifier => ident = Some(child.get_attr().as_ref().unwrap().to_string()),
                MetaType::FormalParameters => if child.get_members().len() > 0 {
                    for param in child.get_members() {
                        match param.get_node_type() {
                            MetaType::FormalParameter => {
                                let mut param_modifiers: Vec<String> = Vec::new();
                                let mut ty: Option<TypeDescriptor> = None;
                                let mut name: Option<String> = None;

                                for part in param.get_members() {
                                    match part.get_node_type() {
                                        MetaType::VariableModifier => param_modifiers.push(part.get_attr().as_ref().unwrap().to_string()),
                                        MetaType::TypeType => {
                                            let type_name = part.get_attr().as_ref().unwrap().as_str();
                                            let package_name = match self.get_dependency_type_info_from_imports(type_name) {
                                                Some(td) => td.package.clone(),
                                                None => Vec::new()
                                            };
                                            ty = Some(TypeDescriptor {
                                                package: package_name,
                                                name: vec![type_name.to_string()]
                                            });
                                        },
                                        MetaType::VariableDeclaratorId => name = Some(part.get_attr().as_ref().unwrap().to_string()),
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
            Some(type_name) => Elements::Constructor(TypeDescriptor { package: package, name: type_name }, modifiers, ident.unwrap().to_string(), params),
            None => Elements::Constructor(TypeDescriptor { package: package, name: Vec::new() }, modifiers, ident.unwrap().to_string(), params),
        };

        self.push_to_stack(JavaElement::new(ty));
    }

    fn add_element(&mut self, category: ElementCategories, element: JavaElement) {
        match self.elements.get_mut(&category) {
            Some(eles) => _ = eles.push(element),
            None => _ = self.elements.insert(category, vec![element]),
        }
    }

    fn push_to_stack(&mut self, element: JavaElement) {
        self.stack.push(element);
    }

    fn add_element_to_parent(&mut self, category: ElementCategories, element: JavaElement) {
        if self.stack.len() <= 0 {
            return;
        }

        if let Some(top) = self.stack.top_mut() {
            top.add_member(category, element);
        }
    }

    fn get_package_name(&self) -> Option<&Vec<String>> {
        if let Some(packages) = self.elements.get(ElementCategories::Package.as_ref()) {
            if let Elements::Package(package_name) = packages.get(0).unwrap().get_type().unwrap() {
                return Some(package_name)
            }
        }
        eprintln!("[WARNING]: package not found");
        None
    }

    fn get_type_name(&self) -> Option<Vec<String>> {
        if self.stack.empty() {
            return None;
        }

        let mut parents: Vec<String> = Vec::new();
        for index in 0..self.stack.len() {
            if let Some(jv) = self.stack.get_by_index(index) {
                if let Elements::Class(_, _, _, name, _, _) = jv.get_type().unwrap() {
                    parents.push(name.to_string())
                }
            }
        }

        Some(parents)
    }

    fn get_dependency_type_info_from_imports(&self, short_name: &str) -> Option<&TypeDescriptor> {
        let re = Regex::new(r"[\[|<].*[\]|>]").unwrap();
        let short_name = re.replace_all(short_name, "");
        
        if let Some(imports) = self.elements.get(&ElementCategories::Imports) {
            for import in imports {
                if let Some(ty) = import.get_type() {
                    if let Elements::Import(package) = ty {
                        if let Some(last_name) = package.get_last_name() {
                            if last_name == short_name {
                                return Some(package);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    fn get_caller(&self, caller_name: Vec<String>) -> Option<Caller> {
        let cn = utils::vec_join::<String>(&caller_name, ".").unwrap();
        let mut index = self.stack.len();
        while index > 0 {
            index -= 1;
            if let Some(stack_item) = self.stack.get_by_index(index) {
                if let Some(item_type) = stack_item.get_type() {
                    match item_type {
                        // ancestors, annotation, modifiers, return type, function name, params(variable(modifier, type, name))
                        Elements::Method(_, _, _, _, _, params) => {
                            
                            for param in params {
                                if param.name == cn {
                                    return Some(Caller { ty: TypeDescriptor { package: Vec::new(), name: vec![param.name.clone()] }, name: Some(cn) })
                                }
                            }
                        },
                        // ancestors, annotations, modifiers, name, extends, implements
                        Elements::Class(_, _, _, _, _, _) => {
                            if let Some(fields) = stack_item.get_member_by_category(ElementCategories::Fields) {
                                for field in fields {
                                    if let Some(ty) = field.get_type() {
                                        match ty {
                                            // ancestors, modifiers, field type, field name, field value
                                            Elements::Field(ancestors, _, field_type, field_name, _) => {
                                                if let Some(name) = field_name {
                                                    if name == &cn {
                                                        return Some(Caller { ty: match field_type {
                                                            Some(ft) => ft.clone(),
                                                            None => TypeDescriptor { package: Vec::new(), name: Vec::new() }
                                                        }, name: Some(cn)});
                                                    }
                                                }
                                            },
                                            _ => return None
                                        }
                                    }
                                }
                            }
                        },
                        _ => return None
                    }
                }
            }
        }
        None
    }
}

pub struct JavaDataCleanPolicy;

impl<'a> JavaDataCleanPolicy {
    pub fn analysis(&mut self, node: &Metadata) -> LanguageDataExtractorResult<HashMap<ElementCategories, Vec<JavaElement>>> {
        assert_eq!(node.get_node_type(), MetaType::File);

        let mut node_listener = JavaDataCleanListener{
            elements: HashMap::new(),
            stack: Stack::new(),
        };
        JavaDataCleanPolicy::node_tree_walker(&node, &mut node_listener);

        let data = node_listener.results();
        std::mem::drop(node_listener);

        Ok(data)
    }

    fn node_tree_walker(node: &Metadata, listener: &mut JavaDataCleanListener) {
        listener.enter(&node);
        for child in node.get_members() {
            JavaDataCleanPolicy::node_tree_walker(child, listener);
        }
        listener.exit(&node);
    }
}

impl LanguageDataExtractorPolicy for JavaDataCleanPolicy {
    fn new() -> Self {
        JavaDataCleanPolicy {}
    }

    fn execute(&mut self, metadata: &PathBuf, output: &PathBuf) -> LanguageDataExtractorResult<PathBuf> {
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
            let context: Metadata = serde_json::from_value(v).unwrap();
            // end solve issue

            if let Ok(jv) = self.analysis(&context) {
                if !output.exists() {
                    _ = fs::create_dir_all(&output);
                }

                let output_file_path = output.join(format!("{}.{}", metadata.file_stem().unwrap().to_str().unwrap(), ELEMENT_FILE_EXTENSION));
                if let Ok(mut f) = File::create(&output_file_path) {
                    let _ = f.write_all(serde_json::to_string(&jv).unwrap().as_bytes());
                    let _ = f.flush();
                }
                return Ok(output_file_path);
            }
        }
        Err(LanguageDataExtractorPolicyError {})
    }
}
