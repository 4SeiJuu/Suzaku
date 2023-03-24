use std::collections::HashMap;
use serde::Serialize;

use suzaku_extension_sdk::{
    language::{
        ivertex::{
            Vertex,
            VertexRelationship,
            VertexType
        }
    },
};

#[derive(Debug, Serialize, Clone)]
pub struct JavaVertex {
    ty: Option<VertexType>,
    members: HashMap<VertexRelationship, Vec<Box<Self>>>
}

impl<'a> JavaVertex {
    pub fn add_member(&mut self, relationship: VertexRelationship, ty: Self) {
        match self.members.get_mut(&relationship) {
            Some(types) => types.push(Box::new(ty)),
            None => _ = self.members.insert(relationship, vec![Box::new(ty)]),
        }
    }
}

impl Vertex for JavaVertex {
    fn new(ty: VertexType) -> Self where Self: Sized {
        JavaVertex {
            ty: Some(ty.clone()),
            members: HashMap::new()
        }
    }

    fn get_type(&self) -> Option<&VertexType> {
        match self.ty.as_ref() {
            Some(value) => Some(&value),
            None => None
        }
    }

    fn get_member_by_relationship(&self, relationship: VertexRelationship) -> Option<&Vec<Box<Self>>> {
        self.members.get(&relationship)
    }
}