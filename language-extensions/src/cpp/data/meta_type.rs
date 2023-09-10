use std::str::FromStr;

use serde:: {
    Serialize,
    Deserialize
};

use strum::Display;
use suzaku_codegen::MetaType;

pub struct MetaTypeConvertError {}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize, MetaType, Eq, Display)]
pub enum MetaType {
}

impl FromStr for MetaType {
    type Err = MetaTypeConvertError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            _ => Err(MetaTypeConvertError{})
        }
    }
}