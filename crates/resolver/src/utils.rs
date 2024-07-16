use std::collections::HashMap;

use frontend::parser::utils::lex_and_parse;

use crate::resolver::{RevResResolv, Resolver};


pub fn lex_parse_resolve(code: &str) -> Result<HashMap<String, usize>, Vec<RevResResolv>> {
    let nodes = lex_and_parse(code).unwrap();
    let mut resolver = Resolver::default();
    match resolver.resolve(&nodes) {
        Ok(h) => Ok(h),
        Err(e) => Err(e),
    }
}
