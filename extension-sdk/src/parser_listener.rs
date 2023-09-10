use std::{str::FromStr, fmt::Debug};

use serde::Serialize;
use super::stack::Stack;
use super::meta::{
    Metadata, IMetaType
};
use super::reorganzier::LanguageMetaReorganizePolicy;
use super::parser::LanguageParserListener;

pub struct ParserListener<M>
where M: IMetaType + ToString + FromStr + Serialize + Debug + Eq + Clone {
    stack: Stack<Metadata<M>>,
    reorganizer: Option<Box<dyn LanguageMetaReorganizePolicy<M>>>
}

impl<M> LanguageParserListener<M> for ParserListener<M>
where M: IMetaType + ToString + FromStr + Serialize + Debug + Eq + Clone {
    fn new(root_node: Metadata<M>, reorganizer: Option<Box<dyn LanguageMetaReorganizePolicy<M>>>) -> Self {
        let mut st = Stack::new();
        st.push(root_node);
        ParserListener {
            stack: st,
            reorganizer: reorganizer
        }
    }

    fn results(&mut self) -> Option<Metadata<M>> {
        self.stack_mut().pop()
    }


    fn stack(&self) -> &Stack<Metadata<M>> {
        &&self.stack
    }

    fn stack_mut(&mut self) -> &mut Stack<Metadata<M>> {
        &mut self.stack
    }

    fn reorganizer(&mut self) -> Option<&mut dyn LanguageMetaReorganizePolicy<M>> {
        match &mut self.reorganizer {
            Some(ref mut reorg) => Some(reorg.as_mut()),
            None => None
        }
    }
}
