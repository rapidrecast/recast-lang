//! Json parser and writer for RapidRecast Definition Language
#[cfg(test)]
mod test;

use crate::ast::RapidRecastDefinition;
use crate::{ParseRRDL, SaveRRDL};
use std::io::Cursor;

/// A parser for the RapidRecastDefinition Language in JSON
pub struct JsonRRDL {}

impl<'self_life, 'input_life> ParseRRDL<'self_life, 'input_life> for JsonRRDL {
    fn parse_rrdl(
        &'self_life self,
        input: &'input_life str,
    ) -> Result<RapidRecastDefinition<'input_life>, String> {
        serde_json::from_str(input).map_err(|e| format!("Failed at le serde json lol: {}", e))
    }
}

impl SaveRRDL<Cursor<Vec<u8>>> for JsonRRDL {
    fn save_rrdl<'param>(
        &'param self,
        definition: &'param RapidRecastDefinition,
    ) -> Cursor<Vec<u8>> {
        let res = serde_json::to_vec(definition).unwrap();
        Cursor::new(res)
    }
}
