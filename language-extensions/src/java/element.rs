use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use suzaku_extension_sdk::{
    language::{
        element::{
            IElement,
            ElementCategories,
            Elements
        }
    },
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JavaElement {
    ty: Option<Elements>,
    members: HashMap<ElementCategories, Vec<Box<Self>>>
}

impl<'a> JavaElement {
    pub fn add_member(&mut self, category: ElementCategories, ty: Self) {
        match self.members.get_mut(&category) {
            Some(types) => types.push(Box::new(ty)),
            None => _ = self.members.insert(category, vec![Box::new(ty)]),
        }
    }
}

impl IElement for JavaElement {
    fn new(ty: Elements) -> Self where Self: Sized {
        JavaElement {
            ty: Some(ty.clone()),
            members: HashMap::new()
        }
    }

    fn set_type(&mut self, ty: Option<Elements>) {
        self.ty = ty;
    }

    fn get_type(&self) -> Option<&Elements> {
        match self.ty.as_ref() {
            Some(value) => Some(&value),
            None => None
        }
    }

    fn get_type_mut(&mut self) -> Option<&mut Elements> {
        match self.ty.as_mut() {
            Some(value) => Some(value),
            None => None
        }
    }

    fn get_member_by_category(&self, category: ElementCategories) -> Option<&Vec<Box<Self>>> {
        self.members.get(&category)
    }

    fn get_member_by_category_mut(&mut self, category: ElementCategories) -> Option<&mut Vec<Box<Self>>> {
        self.members.get_mut(&category)
    }
}