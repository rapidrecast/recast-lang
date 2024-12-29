use std::borrow::Cow;
use crate::ast::{RapidRecastDefinition, Version};

pub const SCHEMA_BARE_MINIMUM: &'static RapidRecastDefinition = &RapidRecastDefinition {
    id: Cow::Borrowed("unique-schema-id-123"),
    language_version: Version {
        major: 1,
        minor: 2,
        patch: 3,
    },
    file_version: Version {
        major: 4,
        minor: 5,
        patch: 6,
    },
    name: Some(Cow::Borrowed("some name")),
    description: Some(Cow::Borrowed("some description")),
    ast: vec![],
};
