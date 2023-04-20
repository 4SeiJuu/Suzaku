use std::collections::HashMap;

use serde::{
    Serialize,
    Deserialize
};

use suzaku_extension_sdk::element::{
    Elements, 
    ToSignature
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphVertex {
    pub package: Option<Elements>,
    pub ty: Option<Elements>,
    pub defines: Vec<Elements>,
    pub callouts: Vec<Elements>
}

impl GraphVertex {
    pub fn get_name(&self) -> String {
        self.ty.as_ref().unwrap().to_string()
    }

    pub fn get_signature(&self) -> String {
        if let Some(element_type) = &self.ty {
            return element_type.to_signature();
        }
        if let Some(pkg) = &self.package {
            return pkg.to_signature();
        }
        String::from("")
    }

    pub fn get_package_name(&self) -> String {
        match &self.package {
            Some(p) => p.to_string(),
            None => String::from("others")
        }
    }

    pub fn to_graphviz_node(&self) -> String {
        let node_name = self.get_signature();
        let label = self.get_signature();
        let group = self.get_package_name();

        format!("{} [label={}, group={}];", node_name, label, group)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub from: GraphVertex,
    pub to: GraphVertex
}

impl GraphEdge {
    pub fn to_graphviz_edge(&self) -> String {
        format!("{} -> {}", self.from.get_signature(), self.to.get_signature())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphData {
    pub depends: Vec<GraphEdge>,
    pub elements: HashMap<String, GraphVertex>
}