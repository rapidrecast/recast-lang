use crate::ast::{RapidRecastDefinition, Version};

pub const SCHEMA_BARE_MINIMUM: &'static RapidRecastDefinition = &RapidRecastDefinition {
    id: Default::default(),
    language_version: Version {},
    file_version: Version {},
    name: None,
    description: None,
    ast: vec![],
};
