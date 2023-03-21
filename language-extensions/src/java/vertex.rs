use std::collections::{HashMap, LinkedList};
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

#[derive(Debug, Serialize)]
pub struct JavaVertex {
    ty: Option<VertexType>,
    members: HashMap<VertexRelationship, Vec<VertexType>>
}

impl<'a> JavaVertex {
    pub fn add_member(&mut self, relationship: VertexRelationship, ty: &'a VertexType) {
        match self.members.get_mut(&relationship) {
            Some(types) => types.push(ty.clone()),
            None => _ = self.members.insert(relationship, vec![ty.clone()]),
        }
    }
}

impl Vertex for JavaVertex {
    fn new(ty: &VertexType) -> Self where Self: Sized {
        JavaVertex {
            ty: Some(ty.clone()),
            members: HashMap::new()
        }
    }

    fn get_type(&self) -> &Option<VertexType> {
        &self.ty
    }

    fn get_member_by_relationship(&self, relationship: VertexRelationship) -> Option<&Vec<VertexType>> {
        self.members.get(&relationship)
    }
}