use regex::Regex;
use serde::{
    Serialize, 
    Deserialize
};
use strum::EnumIter;
use inflections::case::is_pascal_case;

use crate::utils::{
    vec_join,
    replace_special_chars
};

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Copy, EnumIter)]
pub enum ElementCategories {
    Package,
    Imports,
    Classes,
    Interfaces,
    Enums,
    Records,
    Annotations,
    Fields,
    Methods,
    MethodCalls,
    CreatorCalls,
    Constructors,
}

impl AsRef<ElementCategories> for ElementCategories {
    fn as_ref(&self) -> &ElementCategories {
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
        match vec_join(&self.package, ".") {
            Some(v) => v,
            None => String::from("")
        }
    }

    pub fn get_name_str(&self) -> String {
        match vec_join(&self.name, "::") {
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

    pub fn is(&self, other: &Self) -> bool {
        let re = Regex::new(r"[\[|<].*[\]|>]").unwrap();

        let name_str = re.replace_all(self.to_string().as_str(), "").to_string();
        let other_str = re.replace_all(other.to_string().as_str(), "").to_string();

        if name_str == other_str || name_str.ends_with(&other_str) {
            return true;
        } 
        
        if let Some(first_name) = other.name.first() {
            if name_str.ends_with(first_name) {
                return true;
            }
        }

        false
    }
}

impl ToString for TypeDescriptor {
    fn to_string(&self) -> String {
        let name_string = vec_join(&self.name, "::");
        match vec_join(&self.package, ".") {
            Some(p) => match name_string {
                Some(n) => format!("{}.{}", p, n),
                None => p
            },
            None => match name_string {
                Some(n) => n,
                None => String::from("")
            }
        }
    }
}

impl ToSignature for TypeDescriptor {
    fn to_signature(&self) -> String {
        let name_signature = vec_join(&self.name, "_");
        replace_special_chars(match vec_join(&self.package, "_") {
            Some(p) => match name_signature {
                Some(n) => format!("{}_{}", p, n),
                None => p
            },
            None => match name_signature {
                Some(n) => n,
                None => String::from("")
            }
        }, vec!["[", "]", "<", ">", "?"], "")
    }
}

impl From<&Vec<String>> for TypeDescriptor {
    fn from(value: &Vec<String>) -> Self {
        let mut package: Vec<String> = Vec::new();
        let mut name: Vec<String> = Vec::new();

        if value.len() == 1 {
            name.push(value.first().unwrap().clone());
        } else {
            for item in value {
                if is_pascal_case(item.as_str()) {
                    name.push(item.clone());
                } else {
                    package.push(item.clone());
                }
            }
        }

        TypeDescriptor { package: package, name: name }
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
pub struct Caller {
    pub ty: TypeDescriptor,
    pub name: Option<String>
}

impl ToString for Caller {
    fn to_string(&self) -> String {
        match &self.name {
            Some(n) => format!("{} {}", self.ty.to_string(), n),
            None => self.ty.to_string()
        }
    }
}

impl ToSignature for Caller {
    fn to_signature(&self) -> String {
        replace_special_chars(match &self.name {
            Some(n) => format!("{}_{}", self.ty.to_signature(), n),
            None => self.ty.to_signature()
        }, vec!["[", "]", "<", ">"], "_")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Elements {
    // package name
    Package(Vec<String>),
    // TypeDescriptor {package name, type name}
    Import(TypeDescriptor),

    // external type which could not be found the defination in source code
    Type(TypeDescriptor),
    // ancestors, annotations, modifiers, name, extends, implements
    Class(TypeDescriptor, Vec<String>, Vec<String>, String, Vec<TypeDescriptor>, Vec<TypeDescriptor>),
    // ancestors, annotations, modifiers, name, extends
    Interface(TypeDescriptor, Vec<String>, Vec<String>, String, Vec<TypeDescriptor>),
    // ancestors, annotations, modifiers, name, members
    Enum(TypeDescriptor, Vec<String>, Vec<String>, String, Vec<String>),
    // ancestors, annotations, modifiers, name
    Record(TypeDescriptor, Vec<String>, Vec<String>, String),

    // ancestors, modifiers, ident, params(modifiers, type, name)
    Constructor(TypeDescriptor, Vec<String>, String, Vec<ParamDescriptor>),
    // ancestors, modifiers, field type, field name, field value
    Field(TypeDescriptor, Vec<String>, Option<TypeDescriptor>, Option<String>, Option<String>), 
    // ancestors, annotation, modifiers, return type, function name, params(variable(modifier, type, name))
    Method(TypeDescriptor, Option<String>, Vec<String>, TypeDescriptor, String, Vec<ParamDescriptor>),
    
    // type, rest
    CreatorCall(TypeDescriptor, Vec<String>),
    // TODO: how to due with cascade methods call. eg: a.b().c()
    // cast, caller, method name, params((annotation, type, name))
    MethodCall(Option<String>, Caller, String, Vec<String>),
}

impl AsRef<Elements> for Elements {
    fn as_ref(&self) -> &Elements {
        self
    }
}

impl ToString for Elements {
    fn to_string(&self) -> String {
        let vec_to_string = |v: &Vec<String>, sep: &str| -> String {
            match vec_join(v, sep) {
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
            Elements::Package(idents) => vec_to_string(&idents, "."),
            Elements::Import(descriptor) => descriptor.to_string(),
            /* types */
            Elements::Type(desc) => desc.to_string(),
            Elements::Class(ancestors, _, _, name, _, _) => 
                format!("{}.{}", ancestors.to_string(), name),
            Elements::Interface(ancestors, _, _, name, _) => 
                format!("{}.{}", ancestors.to_string(), name),
            Elements::Enum(ancestors, _, _, name, _) => 
                format!("{}.{}", ancestors.to_string(), name),
            Elements::Record(ancestors, _, _, name) =>
                format!("{}.{}", ancestors.to_string(), name),
            /* members */
            Elements::Field(ancestors, _, ty, name, value) => match value {
                Some(v) => format!("{} {}::{} = {}", type_descriptor_option_to_string(ty), ancestors.to_string(), string_option_to_string(name), v),
                None => format!("{} {}.{}", type_descriptor_option_to_string(ty), ancestors.to_string(), string_option_to_string(name))
            },
            Elements::Constructor(ancestors, _, name, params) => {
                let mut param_strs: Vec<String> = Vec::new();
                for param in params {
                    param_strs.push(param.to_string());
                }
                format!("{}::{}({})", ancestors.to_string(), name, vec_to_string(&param_strs, ", "))
            },
            Elements::Method(ancestors, _, _, ret_ty, name, params) => {
                let mut param_strs: Vec<String> = Vec::new();
                for param in params {
                    param_strs.push(param.to_string());
                }
                format!("{} {}.{}({})", ret_ty.to_string(), ancestors.to_string(), name, vec_to_string(&param_strs, ", "))
            },
            /* call outs */
            Elements::CreatorCall(creator_type, rest) => format!("{}({})", creator_type.to_string(), vec_to_string(rest, ", ")),
            Elements::MethodCall(_, caller, name, params) => format!("{}.{}({})", caller.to_string(), name, vec_to_string(params, ", ")),
        }
    }
}

impl ToSignature for Elements {
    fn to_signature(&self) -> String {
        let vec_to_string = |v: &Vec<String>, sep: &str| -> String {
            match vec_join(v, sep) {
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

        match self {
            Elements::Package(idents) => 
                replace_special_chars(format!("{}", vec_to_string(&idents, "_")), vec!["[", "]", "<", ">"], "_"),
            Elements::Import(descriptor) => 
                replace_special_chars(format!("{}", descriptor.to_signature()), vec!["[", "]", "<", ">"], "_"),
            /* types */
            Elements::Type(desc) => 
                replace_special_chars(format!("{}", desc.to_signature()), vec!["[", "]", "<", ">"], "_"),
            Elements::Class(ancestors, _, _, name, _, _) => 
                replace_special_chars(format!("{}_{}", ancestors.to_signature(), name), vec!["[", "]", "<", ">"], "_"),
            Elements::Interface(ancestors, _, _, name, _) => 
                replace_special_chars(format!("{}_{}", ancestors.to_signature(), name), vec!["[", "]", "<", ">"], "_"),
            Elements::Enum(ancestors, _, _, name, _) => 
                replace_special_chars(format!("{}_{}", ancestors.to_signature(), name), vec!["[", "]", "<", ">"], "_"),
            Elements::Record(ancestors, _, _, name) =>
                replace_special_chars(format!("{}_{}", ancestors.to_signature(), name), vec!["[", "]", "<", ">"], "_"),
            /* members */
            Elements::Field(ancestors, _, _, name, _) => 
                replace_special_chars(format!("{}_{}", ancestors.to_signature(), string_option_to_string(name)), vec!["[", "]", "<", ">"], "_"),
            Elements::Constructor(ancestors, _, name, _) =>
                replace_special_chars(format!("{}_{}", ancestors.to_signature(), name), vec!["[", "]", "<", ">"], "_"),
            Elements::Method(ancestors, _, _, ret_ty, name, _) => 
                replace_special_chars(format!("{}_{}_{}", ret_ty.to_string(), ancestors.to_signature(), name), vec!["[", "]", "<", ">"], "_"),
            /* call outs */
            Elements::MethodCall(_, caller, name, _) => 
                replace_special_chars(format!("{}_{}", caller.to_signature(), name), vec!["[", "]", "<", ">"], "_"),
            Elements::CreatorCall(creator_type, _) => 
                replace_special_chars(format!("{}", creator_type.to_signature()), vec!["[", "]", "<", ">"], "_"),
            _ => String::from("invalid value")
        }
    }
}

pub trait IElement {
    fn new(ty: Elements) -> Self where Self: Sized;
    fn set_type(&mut self, ty: Option<Elements>);
    fn get_type(&self) -> Option<&Elements>;
    fn get_type_mut(&mut self) -> Option<&mut Elements>;
    fn get_member_by_category(&self, category: ElementCategories) -> Option<&Vec<Box<Self>>>;
    fn get_member_by_category_mut(&mut self, category: ElementCategories) -> Option<&mut Vec<Box<Self>>>;
}