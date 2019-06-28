use super::scan::Token;
use super::scan::*;
use super::tables::{DependencyTable, SExpression, SExpressionTable};
use std::fs;
use std::io::Result;

struct ParserTables<'a> {
    expressionTable: SExpressionTable<'a>,
    dependencyTable: DependencyTable,
}

pub fn parse_file<'a>(filename: String) -> Result<ParserTables<'a>> {
    let contents = fs::read_to_string(filename)?;
    let tokens = scan_str(&contents)?;
    //:= TODO: here
    SExpression::from_tokens(0, tokens, 0);
    Ok()
}
