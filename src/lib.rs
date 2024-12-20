//! The RapidRecast Definition Language is a custom language used to configure RapidRecast.
//! This crate provides a lexer and parser for the language.
//! It also provides the interface to turn JSON, YAML, TOML, and XML into the language.

#![deny(missing_docs)]
#![deny(warnings)]

pub mod ast;
pub mod rrdl;

/// The trait defining how a parser behaves.
/// You can also create your own implementations.
pub trait ParseRRDL<'a, 'b> {
    /// Given a string, return the AST.
    ///
    /// TODO use a sync and async reader?
    fn parse_rrdl(&'a self, input: &'b str) -> Result<ast::RapidRecastDefinition<'b>, ()>;
}
