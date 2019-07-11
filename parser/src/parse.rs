use super::scan::Token;
use super::scan::*;
use super::tables::{
    new_dependency_table, new_expression_table, CommentMarkPair, DependencyTable, SExpression,
    SExpressionTable,
};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Result;

lazy_static! {
    static ref filetype_comment_table: HashMap<&'static str, CommentMarkPair> = {
        let mut fm = HashMap::new();
        let mut m = CommentMarkPair::new();

        m.insert(String::from(";"), String::from("\n"));
        m.insert(String::from("#|"), String::from("|#"));

        fm.insert("lisp", m);
        fm
    };
}

#[derive(Debug)]
pub struct ParserTables<'a> {
    pub expression_table: SExpressionTable<'a>,
    pub dependency_table: DependencyTable,
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

    //:= TODO: for more languages instead of just for lisp
    let comment_key_pair = if let Some(p) = filetype_comment_table.get("lisp") {
        p
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Cannot find comment key pair.",
        ));
    };

    Ok(SExpression::from_tokens(
        &mut start_id,
        &tokens,
        0,
        &comment_key_pair,
    )?)
}

pub fn make_parser_table<'a>(se: &'a SExpression) -> Result<ParserTables<'a>> {
    Ok(ParserTables {
        expression_table: new_expression_table(se)?,
        dependency_table: new_dependency_table(se)?,
    })
}
