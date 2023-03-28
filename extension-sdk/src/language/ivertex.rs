use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Clone, Copy)]
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

#[derive(Debug, Clone, Serialize)]
pub enum VertexType {
    // package name
    Package(Vec<String>),
    // package name, type name
    Import(Vec<String>, String),
    // package, parent, annotations, modifiers, name, extends, implements
    Class(Vec<String>, Option<String>, Vec<String>, Vec<String>, String, Option<(Vec<String>, String)>, Vec<(Vec<String>, String)>),
    // package, parent type, modifiers, name, 
    Interface(Vec<String>, Option<String>, Vec<String>, String),
    // package, name, item
    Enum(Vec<String>, String, String, String),
    Annotation,
    Record,
    // package, type name, modifiers, field type, field name, field value
    Field(Vec<String>, Option<String>, Vec<String>, Option<(Vec<String>, String)>, Option<String>, Option<String>), 
    // package, type name, annotation, modifiers, return type, function name, params(variable(modifier, type, name))
    Method(Vec<String>, String, Option<String>, Vec<String>, String, String, Vec<(Option<String>, String, String)>),
    // TODO: how to due with cascade methods call. eg: a.b().c()
    // cast, caller, method name, params((annotation, type, name))
    MethodCall(Option<String>, Option<String>, String, Vec<String>),
    // package, name, rest
    Creator(Vec<String>, Vec<String>, Vec<String>),
    // package, type, modifiers, ident, params(modifiers, type, name)
    Constructor(Vec<String>, Option<String>, Vec<String>, String, Vec<(Vec<String>, String, String)>),
}

impl AsRef<VertexType> for VertexType {
    fn as_ref(&self) -> &VertexType {
        self
    }
}

pub trait IVertex {
    fn new(ty: VertexType) -> Self where Self: Sized;
    fn get_type(&self) -> Option<&VertexType>;
    fn get_member_by_category(&self, category: VertexCategories) -> Option<&Vec<Box<Self>>>;
}