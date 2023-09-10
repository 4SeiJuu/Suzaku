use std::{str::FromStr, fmt::Debug};

use serde::Serialize;

use super::meta::{
    IMetaType,
    Metadata
};

pub trait LanguageMetaReorganizePolicy<M>
where M: IMetaType + ToString + FromStr + Serialize + Debug + Eq + Clone {
    fn new() -> Self where Self: Sized;
    fn reorganize(&mut self, meta: &mut Metadata<M>) -> Vec<Metadata<M>>;
}