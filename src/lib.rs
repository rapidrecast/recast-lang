//! The RapidRecast Definition Language is a custom language used to configure RapidRecast.
//! This crate provides a lexer and parser for the language.
//! It also provides the interface to turn JSON, YAML, TOML, and XML into the language.

#![deny(missing_docs)]
#![deny(warnings)]

use crate::ast::RapidRecastDefinition;
use std::io::Read;

pub mod ast;
pub mod json;
pub mod rrdl;
#[cfg(test)]
mod test;

/// The trait defining how a parser behaves.
/// You can also create your own implementations.
pub trait ParseRRDL<'self_life, 'input_life> {
    /// Given a string, return the AST.
    ///
    /// TODO use a sync and async reader?
    fn parse_rrdl(
        &'self_life self,
        input: &'input_life str,
    ) -> Result<RapidRecastDefinition<'input_life>, ()>;
}

/// Save a provided RapidRecast Definition Language AST into a specified output.
pub trait SaveRRDL<READ: Read> {
    /// Given a definition, save it to an output
    fn save_rrdl<'param>(&'param self, definition: &'param RapidRecastDefinition) -> READ;
}
