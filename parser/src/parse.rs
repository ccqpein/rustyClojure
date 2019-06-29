use super::scan::Token;
use super::scan::*;
use super::tables::{
    new_dependency_table, new_expression_table, DependencyTable, SExpression, SExpressionTable,
};
use std::error::Error;
use std::fs;
use std::io;
use std::io::Result;

struct ParserTables<'a> {
    expressionTable: SExpressionTable<'a>,
    dependencyTable: DependencyTable,
}

pub fn parse_file<'a>(filename: String) -> Result<SExpression> {
    let contents = fs::read_to_string(filename)?;
    let mut tokens = match scan_str(&contents) {
        Ok(tokens) => tokens,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.description())),
    };

    // give tokens start ( and end )
    tokens.push(String::from(")"));
    tokens.insert(0, String::from("("));
    //:= TODO: here
    let mut start_id: i64 = 0;
    Ok(SExpression::from_tokens(&mut start_id, &tokens, 0)?)

    // Ok(ParserTables {
    //     expressionTable: new_expression_table(&result)?,
    //     dependencyTable: new_dependency_table(&result)?,
    // })
}
