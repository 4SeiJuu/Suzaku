use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use suzaku_extension_sdk::{
    language::{
        ivertex::{
            IVertex,
            VertexCategories,
            VertexType
        }
    },
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JavaVertex {
    ty: Option<VertexType>,
    members: HashMap<VertexCategories, Vec<Box<Self>>>
}

impl<'a> JavaVertex {
    pub fn add_member(&mut self, category: VertexCategories, ty: Self) {
        match self.members.get_mut(&category) {
            Some(types) => types.push(Box::new(ty)),
            None => _ = self.members.insert(category, vec![Box::new(ty)]),
        }
    }
}

impl IVertex for JavaVertex {
    fn new(ty: VertexType) -> Self where Self: Sized {
        JavaVertex {
            ty: Some(ty.clone()),
            members: HashMap::new()
        }
    }

    fn set_type(&mut self, ty: Option<VertexType>) {
        self.ty = ty;
    }

    fn get_type(&self) -> Option<&VertexType> {
        match self.ty.as_ref() {
            Some(value) => Some(&value),
            None => None
        }
    }

    fn get_type_mut(&mut self) -> Option<&mut VertexType> {
        match self.ty.as_mut() {
            Some(value) => Some(value),
            None => None
        }
    }

    fn get_member_by_category(&self, category: VertexCategories) -> Option<&Vec<Box<Self>>> {
        self.members.get(&category)
    }

    fn get_member_by_category_mut(&mut self, category: VertexCategories) -> Option<&mut Vec<Box<Self>>> {
        self.members.get_mut(&category)
    }
}