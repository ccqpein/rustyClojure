use super::scan::Token;
use lazy_static::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};

type ScopeNum = i64;
type ScopeTable<'a> = HashMap<ScopeNum, &'a Scope>;
type DependencyTable = HashMap<ScopeNum, Vec<ScopeNum>>;

// lazy_static! {
//     static ref scope_table: HashMap<ScopeNum, Scope> = HashMap::new();
//     static ref dependency_table: HashMap<ScopeNum, Vec<ScopeNum>> = HashMap::new();
// }

#[derive(Debug)]
pub enum ExpressionNode {
    Nil,
    Scope(Scope),
    Value(String),
}

#[derive(Debug)]
pub struct Scope {
    pub id: ScopeNum,

    pub expression: Vec<ExpressionNode>,
}

impl Scope {
    pub fn from_tokens(id: i64, content: &mut Vec<Token>) -> Result<Scope> {
        let mut result = Scope {
            id: id,
            expression: vec![],
        };

        let leng = content.len();

        //check first token
        if leng < 1 || &content[0] != "(" {
            return Err(Error::new(ErrorKind::InvalidInput, "Wrong input."));
        }

        //clean first "("
        content.drain(..1);
        loop {
            match content[0].as_str() {
                "(" => {
                    result
                        .expression
                        .push(ExpressionNode::Scope(Self::from_tokens(
                            result.id + 1,
                            content,
                        )?));
                    continue;
                }
                ")" => {
                    if id == 0 {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            "Found unmatched \")\" ",
                        ));
                    }
                    content.drain(..1);
                    break;
                }
                //:= this part can write some value checker
                _ => result
                    .expression
                    .push(ExpressionNode::Value(content[0].clone())),
            }
            content.drain(..1);
        }

        Ok(result)
    }

    //pub fn new_scope_table(&self) -> Result<ScopeTable> {}

    fn add_to_scope_table<'b>(&'b self, target: &mut ScopeTable<'b>) {
        target.insert(self.id, self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn scope_recursive_test() {
        let mut testcase0 = vec![
            String::from("("),
            String::from("defun"),
            String::from("test"),
            String::from("("),
            String::from("a"),
            String::from(")"),
            String::from("("),
            String::from("print"),
            String::from("\"a\""),
            String::from(")"),
            String::from(")"),
        ];

        println!("{:#?}", Scope::from_tokens(0, &mut testcase0));

        //if more ) at endding
        let mut testcase1 = vec![
            String::from("("),
            String::from("defun"),
            String::from("test"),
            String::from("("),
            String::from("a"),
            String::from(")"),
            String::from("("),
            String::from("print"),
            String::from("\"a\""),
            String::from(")"),
            String::from(")"),
            String::from(")"),
        ];

        if let Err(e) = Scope::from_tokens(0, &mut testcase1) {
            println!("{:?}", e.description());
        } else {
            panic!();
        }
    }
}
