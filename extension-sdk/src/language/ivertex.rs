use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Hash, Serialize)]
pub enum VertexRelationship {
    Package,
    Imports,
    Class,
    Interface,
    Annotation,
    Fields,
    Methods,
    MethodCalls,
}

impl AsRef<VertexRelationship> for VertexRelationship {
    fn as_ref(&self) -> &VertexRelationship {
        self
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum VertexType {
    Package(String),                                                // package name
    Class(Vec<String>, String, String, String, Vec<String>),        // modifiers, package, name, extends, implements
    Interface(Vec<String>, String, String),                         // modifiers, package, name
    Enum(Vec<String>, String, String, String),                      // package, name, item
    Annotation,
    Record,
    Field(String, String, Vec<String>, String, String, String),           // package, type name, modifiers, field type, field name, field value
    Method(String, String, Vec<String>, String, Vec<(String, String)>),   // package, type name, modifiers, return type, function name, params
    MethodCall(String, String)                                            // package, type name, method name 
}

impl AsRef<VertexType> for VertexType {
    fn as_ref(&self) -> &VertexType {
        self
    }
}

pub trait Vertex {
    fn new(ty: &VertexType) -> Self where Self: Sized;
    fn get_type(&self) -> &Option<VertexType>;
    fn get_member_by_relationship(&self, relationship: VertexRelationship) -> Option<&Vec<VertexType>>;
}