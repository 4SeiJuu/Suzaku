use serde::{Serialize, Deserialize};

use crate::utils::vec_join;

use super::super::utils;

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Copy)]
pub enum VertexCategories {
    Package,
    Imports,
    Classes,
    Interfaces,
    Annotations,
    Fields,
    Methods,
    MethodCalls,
    CreatorCalls,
    Constructors,
}

impl AsRef<VertexCategories> for VertexCategories {
    fn as_ref(&self) -> &VertexCategories {
        self
    }
}

pub trait ToSignature {
    fn to_signature(&self) -> String;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDescriptor {
    pub package: Vec<String>,
    pub name: Vec<String>
}

impl TypeDescriptor {
    pub fn get_package_str(&self) -> String {
        match utils::vec_join(&self.package, "::") {
            Some(v) => v,
            None => String::from("")
        }
    }

    pub fn get_name_str(&self) -> String {
        match utils::vec_join(&self.name, "::") {
            Some(v) => v,
            None => String::from("")
        }
    }

    pub fn get_last_name(&self) -> Option<String> {
        match self.name.last(){
            Some(v) => Some(v.clone()),
            None => None
        }
    }
}

impl ToString for TypeDescriptor {
    fn to_string(&self) -> String {
        let name_signature = vec_join(&self.name, "_");
        match utils::vec_join(&self.package, ".") {
            Some(p) => match name_signature {
                Some(n) => format!("{}.{}", p, n),
                None => p
            },
            None => match name_signature {
                Some(n) => n,
                None => String::from("")
            }
        }
    }
}

impl ToSignature for TypeDescriptor {
    fn to_signature(&self) -> String {
        let name_signature = vec_join(&self.name, "_");
        match utils::vec_join(&self.package, "_") {
            Some(p) => match name_signature {
                Some(n) => format!("{}_{}", p, n),
                None => p
            },
            None => match name_signature {
                Some(n) => n,
                None => String::from("")
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamDescriptor {
    pub modifiers: Vec<String>,
    pub ty: TypeDescriptor,
    pub name: String
}

impl ToString for ParamDescriptor {
    fn to_string(&self) -> String {
        format!("{} {}", self.ty.to_string(), self.name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VertexType {
    // package name
    Package(Vec<String>),
    // package name, type name
    Import(TypeDescriptor),
    // ancestors, annotations, modifiers, name, extends, implements
    Class(TypeDescriptor, Vec<String>, Vec<String>, String, Option<TypeDescriptor>, Vec<TypeDescriptor>),
    // ancestors, annotations, modifiers, name, extends
    Interface(TypeDescriptor, Vec<String>, Vec<String>, String, Option<TypeDescriptor>),
    Enum,
    Annotation,
    Record,

    // package, ancestor types, modifiers, field type, field name, field value
    Field(Vec<String>, Vec<String>, Vec<String>, Option<TypeDescriptor>, Option<String>, Option<String>), 
    // package, ancestor types, annotation, modifiers, return type, function name, params(variable(modifier, type, name))
    Method(Vec<String>, Vec<String>, Option<String>, Vec<String>, TypeDescriptor, String, Vec<ParamDescriptor>),
    // TODO: how to due with cascade methods call. eg: a.b().c()
    // cast, caller, method name, params((annotation, type, name))
    MethodCall(Option<String>, Option<String>, String, Vec<String>),
    // package, name, rest
    Creator(Vec<String>, Vec<String>, Vec<String>),
    // package, ancestor types, modifiers, ident, params(modifiers, type, name)
    Constructor(Vec<String>, Vec<String>, Vec<String>, String, Vec<ParamDescriptor>),
}

impl AsRef<VertexType> for VertexType {
    fn as_ref(&self) -> &VertexType {
        self
    }
}

impl ToString for VertexType {
    fn to_string(&self) -> String {
        let vec_to_string = |v: &Vec<String>, sep: &str| -> String {
            match utils::vec_join(v, sep) {
                Some(s) => s,
                None => String::from("")
            }
        };

        let string_option_to_string = |o: &Option<String>| -> String {
            match o {
                Some(n) => n.clone(),
                None => String::from("")
            }
        };

        let type_descriptor_option_to_string = |o: &Option<TypeDescriptor>| -> String {
            match o {
                Some(n) => n.to_string(),
                None => String::from("")
            }
        };

        match self {
            VertexType::Package(idents) => vec_to_string(&idents, "."),
            VertexType::Import(descriptor) => descriptor.to_string(),
            VertexType::Class(ancestors, _, _, name, _, _) => 
                format!("{}.{}::{}", ancestors.get_package_str(), ancestors.get_name_str(), name),
            VertexType::Interface(ancestors, _, _, name, _) => 
                format!("{}.{}::{}", ancestors.get_package_str(), ancestors.get_name_str(), name),
            VertexType::Field(package, ancestors, _, ty, name, value) => match value {
                Some(v) => format!("{} {}.{}::{} = {}", type_descriptor_option_to_string(ty), vec_to_string(&package, "."), vec_to_string(&ancestors, "::"), string_option_to_string(name), v),
                None => format!("{} {}.{}.{}", type_descriptor_option_to_string(ty), vec_to_string(&package, "."), vec_to_string(&ancestors, "::"), string_option_to_string(name))
            },
            VertexType::Method(package, ancestors, _, _, ret_ty, name, params) => {
                let mut param_strs: Vec<String> = Vec::new();
                for param in params {
                    param_strs.push(param.to_string());
                }
                format!("{} {}.{}.{}({})", ret_ty.to_string(), vec_to_string(package, "."), vec_to_string(ancestors, "::"), name, vec_to_string(&param_strs, ", "))
            },
            VertexType::MethodCall(_, caller, name, params) => format!("{}.{}({})", string_option_to_string(caller), name, vec_to_string(params, ", ")),
            VertexType::Creator(package, name, rest) => format!("{}.{}({})", vec_to_string(package, "."), vec_to_string(name, "."), vec_to_string(rest, ", ")),
            VertexType::Constructor(package, ancestors, _, name, params) => {
                let mut param_strs: Vec<String> = Vec::new();
                for param in params {
                    param_strs.push(param.to_string());
                }
                format!("{}.{}::{}({})", vec_to_string(package, "."), vec_to_string(ancestors, "::"), name, vec_to_string(&param_strs, ", "))
            },
            _ => String::from("invalid value")
        }
    }
}

impl ToSignature for VertexType {
    fn to_signature(&self) -> String {
        let vec_to_string = |v: &Vec<String>, sep: &str| -> String {
            match utils::vec_join(v, sep) {
                Some(s) => s,
                None => String::from("")
            }
        };

        let string_option_to_string = |o: &Option<String>| -> String {
            match o {
                Some(n) => n.clone(),
                None => String::from("")
            }
        };

        let type_descriptor_option_to_string = |o: &Option<TypeDescriptor>| -> String {
            match o {
                Some(n) => n.to_string(),
                None => String::from("")
            }
        };

        match self {
            VertexType::Package(idents) => 
                vec_to_string(&idents, "_"),
            VertexType::Import(descriptor) => 
                descriptor.to_signature(),
            VertexType::Class(ancestors, _, _, name, _, _) => 
                format!("CLASS_{}_{}", ancestors.to_signature(), name),
            VertexType::Interface(ancestors, _, _, name, _) => 
                format!("INTERFACE_{}_{}", ancestors.to_signature(), name),
            VertexType::Field(package, ancestors, _, _, name, _) => 
                format!("FIELD_{}_{}_{}", vec_to_string(&package, "_"), vec_to_string(&ancestors, "_"), string_option_to_string(name)),
            VertexType::Method(package, ancestors, _, _, ret_ty, name, _) => 
                format!("METHOD_{}_{}_{}_{}", ret_ty.to_string(), vec_to_string(package, "_"), vec_to_string(ancestors, "_"), name),
            VertexType::MethodCall(_, caller, name, _) => 
                format!("METHOD_CALL_{}_{}", string_option_to_string(caller), name),
            VertexType::Creator(package, name, _) => 
                format!("CREATOR_{}_{}", vec_to_string(package, "_"), vec_to_string(name, "_")),
            VertexType::Constructor(package, ancestors, _, name, _) =>
                format!("CONSTRUCTOR_{}_{}", vec_to_string(package, "_"), vec_to_string(ancestors, "_")),
            _ => String::from("invalid value")
        }
    }
}

pub trait IVertex {
    fn new(ty: VertexType) -> Self where Self: Sized;
    fn get_type(&self) -> Option<&VertexType>;
    fn get_member_by_category(&self, category: VertexCategories) -> Option<&Vec<Box<Self>>>;
}