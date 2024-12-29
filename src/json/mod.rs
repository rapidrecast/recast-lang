#[cfg(test)]
mod test;

use crate::ast::RapidRecastDefinition;
use crate::{ParseRRDL, SaveRRDL};
use std::io::Read;

pub struct JsonRRDL {}

impl<'self_life, 'input_life> ParseRRDL<'self_life, 'input_life> for JsonRRDL {
    fn parse_rrdl(
        &'self_life self,
        input: &'input_life str,
    ) -> Result<RapidRecastDefinition<'input_life>, ()> {
        serde_json::from_str(input).map_err(|e| eprintln!("Failed at le serde json lol: {}", e))
    }
}

impl<R: Read> SaveRRDL<R> for JsonRRDL {
    fn save_rrdl<'param>(&'param self, definition: &'param RapidRecastDefinition) -> R {
        let res = serde_json::to_vec(definition).unwrap();
        res
    }
}
