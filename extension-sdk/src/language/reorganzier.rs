use super::meta::Metadata;

pub trait LanguageMetaReorganizePolicy {
    fn new() -> Self;
    fn reorganize(&mut self, meta: &mut Metadata) -> Vec<Metadata>;
}