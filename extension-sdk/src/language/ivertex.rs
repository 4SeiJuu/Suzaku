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
pub struct TypeDescriptor {
    pub package: Vec<String>,
    pub name: String
}

#[derive(Debug, Clone, Serialize)]
pub struct ParamDescriptor {
    pub modifiers: Vec<String>,
    pub ty: TypeDescriptor,
    pub name: String
}

#[derive(Debug, Clone, Serialize)]
pub enum VertexType {
    // package name
    Package(Vec<String>),
    // package name, type name
    Import(TypeDescriptor),
    // package, ancestor types, annotations, modifiers, name, extends, implements
    Class(Vec<String>, Vec<String>, Vec<String>, Vec<String>, String, Option<TypeDescriptor>, Vec<TypeDescriptor>),

    Interface,
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

pub trait IVertex {
    fn new(ty: VertexType) -> Self where Self: Sized;
    fn get_type(&self) -> Option<&VertexType>;
    fn get_member_by_category(&self, category: VertexCategories) -> Option<&Vec<Box<Self>>>;
}