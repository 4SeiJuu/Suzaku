use super::meta::Metadata;

pub trait LanguageMetaReorganizePolicy {
    fn new() -> Self where Self: Sized;
    fn reorganize(&mut self, meta: &mut Metadata) -> Vec<Metadata>;
}