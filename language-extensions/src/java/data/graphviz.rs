use std::collections::HashMap;

use serde::{
    Serialize,
    Deserialize
};

use suzaku_extension_sdk::{
    element::{
        Elements, 
        ToSignature
    }, 
    utils::vec_join
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphVertex {
    pub ty: Elements,
    pub defines: Vec<Elements>,
    pub callouts: Vec<Elements>
}

impl GraphVertex {
    pub fn new(ty: Elements) -> Self {
        GraphVertex {
            ty: ty,
            defines: Vec::new(),
            callouts: Vec::new()
        }
    }

    pub fn get_package_name(&self) -> String {
        match match &self.ty {
            Elements::Package(pkg) => Some(pkg),
            Elements::Import(td)
            | Elements::Class(td, _, _, _, _, _)
            | Elements::Interface(td, _, _, _, _)
            | Elements::Constructor(td, _, _, _)
            | Elements::Field(td, _, _, _, _)
            | Elements::Method(td, _, _, _, _, _)
            | Elements::CreatorCall(td, _) => Some(&td.package),
            Elements::MethodCall(_, caller, _, _) => Some(&caller.ty.package),
            _ => None
        } {
            Some(pkg) => match vec_join(&pkg, ".") {
                Some(name) => name,
                None => String::from("others")
            },
            None => String::from("others")
        }
    }

    pub fn to_graphviz_node(&self) -> String {
        let node_name = self.to_signature();
        let label = self.to_string();
        let group = self.get_package_name();

        format!("{} [label=\"{}\", group=\"{}\"];", node_name, label, group)
    }
}

impl ToString for GraphVertex {
    fn to_string(&self) -> String {
        self.ty.to_string()
    }
}

impl ToSignature for GraphVertex {
    fn to_signature(&self) -> String {
        self.ty.to_signature()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub from: GraphVertex,
    pub to: GraphVertex
}

impl GraphEdge {
    pub fn to_graphviz_edge(&self) -> String {
        format!("{} -> {}", self.from.to_signature(), self.to.to_signature())
    }
}

impl ToSignature for GraphEdge {
    fn to_signature(&self) -> String {
        format!("{}__{};", self.from.to_signature(), self.to.to_signature())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphData {
    pub depends: HashMap<String, GraphEdge>,
    pub elements: HashMap<String, GraphVertex>
}