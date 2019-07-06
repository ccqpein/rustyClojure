use super::scan::Token;
use super::scan::*;
use super::tables::{
    new_dependency_table, new_expression_table, CommentMarkPair, DependencyTable, SExpression,
    SExpressionTable,
};
use std::error::Error;
use std::fs;
use std::io;
use std::io::Result;

#[derive(Debug)]
pub struct ParserTables<'a> {
    pub expressionTable: SExpressionTable<'a>,
    dependencyTable: DependencyTable,
}

pub fn parse_file<'a>(filename: &str) -> Result<SExpression> {
    let contents = fs::read_to_string(filename)?;
    let mut tokens = match scan_str(&contents) {
        Ok(tokens) => tokens,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.description())),
    };

    // give tokens start ( and end )
    tokens.push(String::from(")"));
    tokens.insert(0, String::from("("));

    //dbg!(&tokens);
    let mut start_id: i64 = 0;
    let mut comment_key_pair = CommentMarkPair::new();

    //:= hard code now
    comment_key_pair.insert(String::from(";"), String::from("\n"));
    comment_key_pair.insert(String::from("#|"), String::from("|#"));

    Ok(SExpression::from_tokens(
        &mut start_id,
        &tokens,
        0,
        &comment_key_pair,
    )?)
}

pub fn make_parser_table<'a>(se: &'a SExpression) -> Result<ParserTables<'a>> {
    Ok(ParserTables {
        expressionTable: new_expression_table(se)?,
        dependencyTable: new_dependency_table(se)?,
    })
}
