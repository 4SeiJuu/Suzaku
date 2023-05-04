use std::{
    path::PathBuf, 
    collections::HashMap,
    fs,
};

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
    utils::vec_join, 
};

use super::{
    parser_policy::ATTR_EXPRESSION,
    super::data::element::JavaElement
};


pub struct JavaDataExtractorListener {
    elements: HashMap<ElementCategories, Vec<JavaElement>>,
    stack: Stack<JavaElement>
}

impl JavaDataExtractorListener {
    pub fn results(&self) -> HashMap<ElementCategories, Vec<JavaElement>> {
        self.elements.clone()
    }

    fn enter(&mut self, node: &Metadata) {
        match node.get_node_type() {
            MetaType::PackageDeclaration => self.analysis_package_declaration(node),
            MetaType::ImportDeclaration => self.analysis_import_declaration(node),
            MetaType::ClassDeclaration => self.analysis_class_declaration(node),
            MetaType::InterfaceDeclaration => self.analysis_interface_declaration(node),
            MetaType::EnumDeclaration => self.analysis_enum_declaration(node),
            MetaType::RecordDeclaration => self.analysis_recrod_declaration(node),
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
            MetaType::EnumDeclaration => {
                if let Elements::Enum(_, _, _, _, _) = ty.unwrap() {
                    Some((ElementCategories::Enums, self.stack.pop()))
                } else {
                    None
                }
            },
            MetaType::RecordDeclaration => {
                if let Elements::Record(_, _, _, _) = ty.unwrap() {
                    Some((ElementCategories::Records, self.stack.pop()))
                } else {
                    None
                }
            }
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
                if category == ElementCategories::Classes 
                || category == ElementCategories::Interfaces 
                || category == ElementCategories::Enums 
                || category == ElementCategories::Records {
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
                        idents.push(ident.get_attr(ATTR_EXPRESSION).as_ref().unwrap().to_string());
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
                        idents.push(ident.get_attr(ATTR_EXPRESSION).as_ref().unwrap().to_string());
                    }
                    let ty = Elements::Import(TypeDescriptor::from(&idents));
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
                MetaType::Annotation => if let Some(attr) = member.get_attr(ATTR_EXPRESSION) {
                    annotations.push(attr.to_string());
                },
                MetaType::Modifier => if let Some(attr) = member.get_attr(ATTR_EXPRESSION) {
                    modifiers.push(attr.to_string());
                },
                MetaType::Identifier => if let Some(attr) = member.get_attr(ATTR_EXPRESSION) {
                    ident = Some(attr.to_string());
                },
                MetaType::TypeType => {
                    let extend_name: Vec<String> = self.get_typetype_name(member);
                    if !extend_name.is_empty() {
                        let extend_td = TypeDescriptor::from(&extend_name);
                        match self.get_dependency_type_info_from_imports(&extend_td) {
                            Some(edtend_full_td) => extends.push(edtend_full_td.clone()),
                            None => extends.push(extend_td)
                        }
                    }
                },
                MetaType::TypeList => {
                    if member.get_members().is_empty() {
                        if let Some(name) = member.get_attr(ATTR_EXPRESSION) {
                            implements.push(TypeDescriptor { package: Vec::new(), name: vec![name.clone()] });
                        }
                    } else {
                        for type_type in member.get_members() {
                            match type_type.get_node_type() {
                                MetaType::TypeType => {
                                    let implement_name = self.get_typetype_name(type_type);
                                    if !implement_name.is_empty() {
                                        let implement_td = TypeDescriptor::from(&implement_name);
                                        match self.get_dependency_type_info_from_imports(&implement_td) {
                                            Some(implement_full_td) => implements.push(implement_full_td.clone()),
                                            None => implements.push(implement_td)
                                        }
                                    }
                                },
                                _ => {}
                            }
                        }
                    }
                },
                _ => ()
            }
        }

        let mut package_name: Vec<String> = match self.get_package() {
            Some(package_name) => package_name.clone(),
            None => Vec::new()
        };

        let ty = match self.get_type() {
            Some(mut type_name) => {
                package_name.append(&mut type_name);
                Elements::Class(TypeDescriptor::from(&package_name), annotations, modifiers, ident.unwrap(), extends, implements.clone())
            },
            None => Elements::Class(
                TypeDescriptor::from(&package_name), annotations, modifiers, ident.unwrap(), extends, implements.clone())
        };

        self.push_to_stack(JavaElement::new(ty));
    }

    fn analysis_interface_declaration(&mut self, node: &Metadata) {
        assert_eq!(node.get_node_type(), MetaType::InterfaceDeclaration);

        let mut annotations: Vec<String> = Vec::new();
        let mut modifiers: Vec<String> = Vec::new();
        let mut ident: Option<String> = None;
        let mut extends: Vec<TypeDescriptor> = Vec::new();

        for member in node.get_members() {
            match member.get_node_type() {
                MetaType::Annotation => if let Some(attr) = member.get_attr(ATTR_EXPRESSION) {
                    annotations.push(attr.to_string());
                },
                MetaType::Modifier => if let Some(attr) = member.get_attr(ATTR_EXPRESSION) {
                    modifiers.push(attr.to_string());
                },
                MetaType::Identifier => if let Some(attr) = member.get_attr(ATTR_EXPRESSION) {
                    ident = Some(attr.to_string());
                },
                MetaType::TypeType => {
                    let extend_name = self.get_typetype_name(member);
                    if !extend_name.is_empty() {
                        let extend_td = TypeDescriptor::from(&extend_name);
                        match self.get_dependency_type_info_from_imports(&extend_td) {
                            Some(extend_full_td) => extends.push(extend_full_td.clone()),
                            None => extends.push(extend_td)
                        }
                    }
                    extends.push(TypeDescriptor::from(&extend_name));
                },
                _ => ()
            }
        }

        let mut package_name: Vec<String> = match self.get_package() {
            Some(package_name) => package_name.clone(),
            None => Vec::new()
        };

        let ty = match self.get_type() {
            Some(mut type_name) => {
                package_name.append(&mut type_name);
                Elements::Interface(TypeDescriptor::from(&package_name), annotations, modifiers, ident.unwrap(), extends)
            },
            None => Elements::Interface(
                TypeDescriptor::from(&package_name), annotations, modifiers, ident.unwrap(), extends)
        };

        self.push_to_stack(JavaElement::new(ty));
    }

    fn analysis_enum_declaration(&mut self, node: &Metadata) {
        assert_eq!(node.get_node_type(), MetaType::EnumDeclaration);

        let mut annotations: Vec<String> = Vec::new();
        let mut modifiers: Vec<String> = Vec::new();
        let mut ident: Option<String> = None;
        let mut members: Vec<String> = Vec::new();

        for member in node.get_members() {
            match member.get_node_type() {
                MetaType::Annotation => if let Some(attr) = member.get_attr(ATTR_EXPRESSION) {
                    annotations.push(attr.to_string());
                },
                MetaType::Modifier => if let Some(attr) = member.get_attr(ATTR_EXPRESSION) {
                    modifiers.push(attr.to_string());
                },
                MetaType::Identifier => if let Some(attr) = member.get_attr(ATTR_EXPRESSION) {
                    ident = Some(attr.to_string());
                },
                MetaType::EnumConstants => {
                    for constant in member.get_members() {
                        match constant.get_node_type() {
                            MetaType::EnumConstant => members.push(constant.get_attr(ATTR_EXPRESSION).unwrap().to_string()),
                            _ => ()
                        }
                    } 
                },
                _ => ()
            }
        }

        let mut package_name: Vec<String> = match self.get_package() {
            Some(package_name) => package_name.clone(),
            None => Vec::new()
        };

        let ty = match self.get_type() {
            Some(mut type_name) => {
                package_name.append(&mut type_name);
                Elements::Enum(TypeDescriptor::from(&package_name), annotations, modifiers, ident.unwrap(), members)
            },
            None => Elements::Enum(TypeDescriptor::from(&package_name), annotations, modifiers, ident.unwrap(), members)
        };

        self.push_to_stack(JavaElement::new(ty));
    }

    fn analysis_recrod_declaration(&mut self, node: &Metadata) {
        assert_eq!(node.get_node_type(), MetaType::RecordDeclaration);

        let mut annotations: Vec<String> = Vec::new();
        let mut modifiers: Vec<String> = Vec::new();
        let mut ident: Option<String> = None;

        for member in node.get_members() {
            match member.get_node_type() {
                MetaType::Annotation => if let Some(attr) = member.get_attr(ATTR_EXPRESSION) {
                    annotations.push(attr.to_string());
                },
                MetaType::Modifier => if let Some(attr) = member.get_attr(ATTR_EXPRESSION) {
                    modifiers.push(attr.to_string());
                },
                MetaType::Identifier => if let Some(attr) = member.get_attr(ATTR_EXPRESSION) {
                    ident = Some(attr.to_string());
                },
                _ => ()
            }
        }

        let mut package_name: Vec<String> = match self.get_package() {
            Some(package_name) => package_name.clone(),
            None => Vec::new()
        };

        let ty = match self.get_type() {
            Some(mut type_name) => {
                package_name.append(&mut type_name);
                Elements::Record(TypeDescriptor::from(&package_name), annotations, modifiers, ident.unwrap())
            },
            None => Elements::Record(TypeDescriptor::from(&package_name), annotations, modifiers, ident.unwrap())
        };

        self.push_to_stack(JavaElement::new(ty));
    }

    fn analysis_field_declaration(&mut self, node: &Metadata) {
        assert_eq!(node.get_node_type(), MetaType::FieldDeclaration);

        let mut modifiers: Vec<String> = Vec::new();
        let mut ty: Option<TypeDescriptor> = None;
        let mut variable_id: Option<String> = None;
        let mut variable_init: Option<String> = None;

        for member in node.get_members() {
            match member.get_node_type() {
                MetaType::Modifier => if let Some(attr) = member.get_attr(ATTR_EXPRESSION) {
                    modifiers.push(attr.to_string());
                },
                MetaType::TypeType => {
                    let type_name = self.get_typetype_name(member);
                    if !type_name.is_empty() {
                        let type_td = TypeDescriptor::from(&type_name);
                        ty = Some(match self.get_dependency_type_info_from_imports(&type_td) {
                            Some(type_full_td) => type_full_td.clone(),
                            None => type_td
                        });
                    }
                },
                MetaType::VariableDeclarator => {
                    if member.get_members().len() == 0 {
                        variable_id = Some(member.get_attr(ATTR_EXPRESSION).as_ref().unwrap().to_string());
                        continue;
                    }

                    for child in member.get_members() {
                        match child.get_node_type() {
                            MetaType::VariableDeclaratorId => variable_id = Some(child.get_attr(ATTR_EXPRESSION).as_ref().unwrap().to_string()),
                            MetaType::VariableInitializer => if let Some(attr) = child.get_attr(ATTR_EXPRESSION) {
                                variable_init = Some(attr.to_string());
                            }
                            _ => ()
                        }
                    }
                },
                _ => ()
            }
        }

        let mut package_name = match self.get_package() {
            Some(pn) => pn.clone(),
            None => Vec::new()
        };

        let ty = match self.get_type() {
            Some(mut type_name) => {
                package_name.append(&mut type_name);
                Elements::Field(TypeDescriptor::from(&package_name), modifiers.clone(), ty, variable_id, variable_init)
            },
            None => Elements::Field(TypeDescriptor::from(&package_name), modifiers.clone(), ty, variable_id, variable_init)
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
                MetaType::Annotation => if let Some(attr) = member.get_attr(ATTR_EXPRESSION) {
                    annotation = Some(attr.as_str().to_string());
                },
                MetaType::Modifier => if let Some(attr) = member.get_attr(ATTR_EXPRESSION) {
                    modifiers.push(attr.to_string());
                },
                MetaType::TypeTypeOrVoid => {
                    let type_name = match member.get_members().is_empty() {
                        true => vec![member.get_attr(ATTR_EXPRESSION).unwrap().to_string()],
                        false => {
                            let mut type_type_member: Option<&Metadata> = None;
                            for sub_member in member.get_members() {
                                if sub_member.get_node_type() == MetaType::TypeType {
                                    type_type_member = Some(sub_member);
                                    break;
                                }
                            }

                            match type_type_member {
                                Some(ttm) => self.get_typetype_name(ttm),
                                None => Vec::new()
                            }
                        }
                    };

                    if !type_name.is_empty() {
                        let type_td = TypeDescriptor::from(&type_name);
                        ret_type = Some(match self.get_dependency_type_info_from_imports(&type_td) {
                            Some(type_full_td) => type_full_td.clone(),
                            None => type_td
                        });
                    }
                }
                MetaType::Identifier => if let Some(attr) = member.get_attr(ATTR_EXPRESSION) {
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
                                    MetaType::VariableModifier => modifiers.push(item.get_attr(ATTR_EXPRESSION).as_ref().unwrap().to_string()),
                                    MetaType::TypeType => {
                                        let idents = self.get_typetype_name(item);
                                        if !idents.is_empty() {
                                            let idents_td = TypeDescriptor::from(&idents);
                                            ty = Some(match self.get_dependency_type_info_from_imports(&idents_td) {
                                                Some(idents_full_td) => idents_full_td.clone(),
                                                None => idents_td
                                            });
                                        }
                                    },
                                    MetaType::VariableDeclaratorId => ident = Some(item.get_attr(ATTR_EXPRESSION).as_ref().unwrap().to_string()),
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

        let mut package_name = match self.get_package() {
            Some(pn) => pn.clone(),
            None => Vec::new()
        };

        let ty = match self.get_type() {
            Some(mut type_name) => {
                package_name.append(&mut type_name);
                if ret_type.is_none() {
                    println!("{}", serde_json::to_string(node).unwrap());
                }
                Elements::Method(TypeDescriptor::from(&package_name), annotation, modifiers, ret_type.unwrap(), name.unwrap().to_string(), params)
            },
            None => Elements::Method(TypeDescriptor::from(&package_name), annotation, modifiers, ret_type.unwrap(), name.unwrap().to_string(), params),
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
                MetaType::TypeType => cast = Some(child.get_attr(ATTR_EXPRESSION).as_ref().unwrap().to_string()),
                MetaType::Identifier => idents.push(child.get_attr(ATTR_EXPRESSION).as_ref().unwrap().to_string()),
                MetaType::ExpressionList => if child.get_members().len() > 0 {
                    for param_node in child.get_members() {
                        if param_node.get_node_type() != MetaType::Separator {
                            params.push(param_node.get_attr(ATTR_EXPRESSION).as_ref().unwrap().to_string());
                        }
                    }
                } else {
                    params.push(child.get_attr(ATTR_EXPRESSION).as_ref().unwrap().to_string());
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
            let mut package_name = match self.get_package() {
                Some(pkg) => pkg.clone(),
                None => Vec::new()
            };

            let mut type_name = match self.get_type() {
                Some(name) => name,
                None => Vec::new()
            };

            package_name.append(&mut type_name);
            caller = Some(Caller {
                ty: TypeDescriptor::from(&package_name),
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
                    creator_name.push(ident.get_attr(ATTR_EXPRESSION).as_ref().unwrap().to_string())
                },
                MetaType::ClassCreatorRest => if let Some(arguments) = child.get_members().front() {
                    assert_eq!(arguments.get_node_type(), MetaType::Arguments);
                    if arguments.get_members().len() > 0 {
                        for arg in arguments.get_members() {
                            rests.push(arg.get_attr(ATTR_EXPRESSION).as_ref().unwrap().to_string());
                        }
                    }
                },
                MetaType::ArrayCreatorRest => if child.get_members().len() > 0 {
                    for item in child.get_members() {
                        rests.push(item.get_attr(ATTR_EXPRESSION).as_ref().unwrap().to_string());
                    }
                },
                _ => ()
            }
        }

        let mut creator_td = TypeDescriptor::from(&creator_name);
        if let Some(createor_full_td) = self.get_dependency_type_info_from_imports(&creator_td) {
            creator_td = createor_full_td.clone();
        }

        let ty = Elements::CreatorCall(creator_td, rests);
        self.push_to_stack(JavaElement::new(ty));
    }

    fn analysis_constructor(&mut self, node: &Metadata) {
        assert_eq!(node.get_node_type(), MetaType::ConstructorDeclaration);

        let mut modifiers: Vec<String> = Vec::new();
        let mut ident: Option<String> = None;
        let mut params: Vec<ParamDescriptor> = Vec::new();

        for child in node.get_members() {
            match child.get_node_type() {
                MetaType::Modifier => modifiers.push(child.get_attr(ATTR_EXPRESSION).as_ref().unwrap().to_string()),
                MetaType::Identifier => ident = Some(child.get_attr(ATTR_EXPRESSION).as_ref().unwrap().to_string()),
                MetaType::FormalParameters => if child.get_members().len() > 0 {
                    for param in child.get_members() {
                        match param.get_node_type() {
                            MetaType::FormalParameter => {
                                let mut param_modifiers: Vec<String> = Vec::new();
                                let mut ty: Option<TypeDescriptor> = None;
                                let mut name: Option<String> = None;

                                for part in param.get_members() {
                                    match part.get_node_type() {
                                        MetaType::VariableModifier => param_modifiers.push(part.get_attr(ATTR_EXPRESSION).as_ref().unwrap().to_string()),
                                        MetaType::TypeType => {
                                            let type_name = self.get_typetype_name(part);
                                            let mut type_td = TypeDescriptor::from(&type_name);
                                            if let Some(type_full_td) = self.get_dependency_type_info_from_imports(&type_td) {
                                                type_td = type_full_td.clone();
                                            }
                                            ty = Some(type_td);
                                        },
                                        MetaType::VariableDeclaratorId => name = Some(part.get_attr(ATTR_EXPRESSION).as_ref().unwrap().to_string()),
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

        let mut package_name = match self.get_package() {
            Some(pn) => pn.clone(),
            None => Vec::new()
        };

        let ty = match self.get_type() {
            Some(mut type_name) => {
                package_name.append(&mut type_name);
                Elements::Constructor(TypeDescriptor::from(&package_name), modifiers, ident.unwrap().to_string(), params)
            },
            None => Elements::Constructor(TypeDescriptor::from(&package_name), modifiers, ident.unwrap().to_string(), params),
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

    fn get_typetype_name(&self, type_type: &Metadata) -> Vec<String> {
        let mut typetype_name: Vec<String> = Vec::new();
        if type_type.get_members().is_empty() {
            if let Some(ident_str) = type_type.get_attr(ATTR_EXPRESSION) {
                typetype_name.push(ident_str.clone());
            }
        } else {
            for ident in type_type.get_members() {
                match ident.get_node_type() {
                    // TODO: TypeArguments need be produced
                    MetaType::Identifier | MetaType::TypeIdentifier | MetaType::TypeArguments => {
                        if let Some(ident_str) = ident.get_attr(ATTR_EXPRESSION) {
                            typetype_name.push(ident_str.clone());
                        }
                    },
                    _ => {}
                }
            }
        }
        typetype_name
    }

    fn get_package(&self) -> Option<&Vec<String>> {
        if let Some(packages) = self.elements.get(ElementCategories::Package.as_ref()) {
            if let Elements::Package(package_name) = packages.get(0).unwrap().get_type().unwrap() {
                return Some(package_name)
            }
        }
        eprintln!("[WARNING]: package not found");
        None
    }

    fn get_type(&self) -> Option<Vec<String>> {
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

    fn get_dependency_type_info_from_imports(&self, td: &TypeDescriptor) -> Option<&TypeDescriptor> {
        if let Some(imports) = self.elements.get(&ElementCategories::Imports) {
            for import in imports {
                if let Some(ty) = import.get_type() {
                    if let Elements::Import(package) = ty {
                        if package.is(td) {
                            return Some(package)
                        }
                    }
                }
            }
        }
        None
    }

    fn get_caller(&self, caller_name: Vec<String>) -> Option<Caller> {
        // find variable defination
        let cn = vec_join::<String>(&caller_name, ".").unwrap();
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
                                    return Some(Caller { ty: param.ty.clone(), name: Some(cn) })
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
                                            _ => continue
                                        }
                                    }
                                }
                            }
                        },
                        _ => continue
                    }
                }
            }
        }

        // find type import
        let caller_type = TypeDescriptor::from(&caller_name);
        match self.get_dependency_type_info_from_imports(&caller_type) {
            Some(caller_full_type) => Some(Caller { ty: caller_full_type.clone(), name: None}),
            None => Some(Caller { ty: caller_type.clone(), name: None})
        }
    }
}

pub struct JavaDataCleanPolicy;

impl<'a> JavaDataCleanPolicy {
    pub fn analysis(&mut self, node: &Metadata) -> LanguageDataExtractorResult<HashMap<ElementCategories, Vec<JavaElement>>> {
        assert_eq!(node.get_node_type(), MetaType::File);

        let mut node_listener = JavaDataExtractorListener{
            elements: HashMap::new(),
            stack: Stack::new(),
        };
        JavaDataCleanPolicy::node_tree_walker(&node, &mut node_listener);
        Ok(node_listener.results())
    }

    fn node_tree_walker(node: &Metadata, listener: &mut JavaDataExtractorListener) {
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

    fn execute(&mut self, metadata: &PathBuf) -> LanguageDataExtractorResult<String> {
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

            if let Ok(elements) = self.analysis(&context) {
                return Ok(serde_json::to_string(&elements).unwrap());
            }
        }
        Err(LanguageDataExtractorPolicyError {})
    }
}
