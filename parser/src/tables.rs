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
        println!("{}, {}, {:?}", leng, id, content);
        if leng < 1 || &content[0] != "(" {
            return Err(Error::new(ErrorKind::InvalidInput, "Wrong input."));
        }

        //clean first "("
        content.drain(..1);
        loop {
            if content.len() == 0 {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Found unmatched \"(\" ",
                ));
            }
            println!("{:?}", content);
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
                    if id == 0 && content.len() > 1 {
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

    // pub fn new_scope_table<'a, 'b: 'a>(&'b self) -> Result<ScopeTable<'a>> {
    //     let mut result: ScopeTable<'a> = HashMap::new();
    //     self.add_to_scope_table(&mut result);
    //     Ok(result)
    // }

    fn add_to_scope_table<'a, 'b: 'a>(&'b self, target: &mut ScopeTable<'a>) {
        target.insert(self.id, self);
    }

    fn all_children_scopes(&self) -> Vec<&ExpressionNode> {
        self.expression
            .iter()
            .filter(|x| {
                if let ExpressionNode::Scope(_) = x {
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<_>>()
    }
}

fn new_scope_table<'a>(scope: &'a Scope) -> Result<ScopeTable<'a>> {
    let mut result: ScopeTable<'a> = HashMap::new();
    result.insert(scope.id, scope);

    let mut scope_search_stack = scope.all_children_scopes();

    while scope_search_stack.len() != 0 {
        let first: &Scope = if let Some(f) = scope_search_stack.first() {
            if let ExpressionNode::Scope(s) = f {
                s
            } else {
                break;
            }
        } else {
            break;
        };
        first.add_to_scope_table(&mut result);
        let mut new_children = first.all_children_scopes();
        scope_search_stack.append(&mut new_children);
        scope_search_stack.drain(1..);
    }

    Ok(result)
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

        //println!("testcase0: {:#?}", Scope::from_tokens(0, &mut testcase0));

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

        // if let Err(e) = Scope::from_tokens(0, &mut testcase1) {
        //     println!("testcase1: {:?}", e.description());
        // } else {
        //     panic!();
        // }

        //if more ( at beginning
        let mut testcase2 = vec![
            String::from("("),
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
            //String::from(")"),
        ];

        // if let Err(e) = Scope::from_tokens(0, &mut testcase2) {
        //     println!("testcase2: {:?}", e.description());
        // } else {
        //     println!("testcase2: {:?}", Scope::from_tokens(0, &mut testcase2));
        //     panic!();
        // }

        // if more ( also more )
        let mut testcase3 = vec![
            String::from("("),
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

        if let Err(e) = Scope::from_tokens(0, &mut testcase3) {
            println!("testcase3: {:?}", e.description());
        } else {
            println!("testcase3: {:?}", Scope::from_tokens(0, &mut testcase3));
            panic!();
        }
    }

    //#[test]
    fn scope_table_generate() {
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

        let a = Scope::from_tokens(0, &mut testcase0);
        println!("{:#?}", a);
        //println!("{:#?}", new_scope_table(&a));
    }
}
