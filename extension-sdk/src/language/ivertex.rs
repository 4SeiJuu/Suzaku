use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Clone, Copy)]
pub enum VertexRelationship {
    Package,
    Imports,
    Class,
    Interface,
    Annotation,
    Fields,
    Methods,
    MethodCalls,
    CreatorCalls,
    Constructors,
}

impl AsRef<VertexRelationship> for VertexRelationship {
    fn as_ref(&self) -> &VertexRelationship {
        self
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum VertexType {
    // package name
    Package(String),
    // package name, type name
    Import(String, String),
    // modifiers, package, parent, name, extends, implements
    Class(Vec<String>, String, Option<String>, String, Option<String>, Vec<String>),
    // modifiers, package, name
    Interface(Vec<String>, String, String),
    // package, name, item
    Enum(Vec<String>, String, String, String),
    Annotation,
    Record,
    // package, type name, modifiers, field type, field name, field value
    Field(String, String, Vec<String>, Option<String>, Option<String>, Option<String>), 
    // package, type name, annotation, modifiers, return type, function name, params(variable(modifier, type, name))
    Method(String, String, Option<String>, Vec<String>, String, String, Vec<(Option<String>, String, String)>),
    // cast, caller, method name, params((annotation, type, name))
    // TODO: how to due with cascade methods call. eg: a.b().c()
    MethodCall(Option<String>, Option<String>, String, Vec<String>),
    // name, rest
    Creator(Vec<String>, Vec<String>),
    // modifiers, ident, params(modifiers, type, name)
    Constructor(Vec<String>, String, Vec<(Vec<String>, String, String)>),
}

impl AsRef<VertexType> for VertexType {
    fn as_ref(&self) -> &VertexType {
        self
    }
}

pub trait Vertex {
    fn new(ty: VertexType) -> Self where Self: Sized;
    fn get_type(&self) -> Option<&VertexType>;
    fn get_member_by_relationship(&self, relationship: VertexRelationship) -> Option<&Vec<Box<Self>>>;
}