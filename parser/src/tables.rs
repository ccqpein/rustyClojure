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
    Symbol(String),
}

#[derive(Debug)]
pub struct Scope {
    pub id: ScopeNum,

    pub expression: Vec<ExpressionNode>,
}

impl Scope {
    // find end index of )}] of start index of ({[
    fn find_wrap_parentheses(start: usize, content: &Vec<Token>) -> Result<usize> {
        if content[start] != "(" {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "First element is not (",
            ));
        }

        let mut stack = vec![&content[start]];
        let mut ind = start + 1;
        while stack.len() != 0 {
            if ind >= content.len() {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Find unmatched start parentheses",
                ));
            }
            match content[ind].as_ref() {
                "(" | "{" | "[" => stack.push(&content[ind]),
                ")" => {
                    if stack.pop().unwrap() != "(" {
                        return Err(Error::new(ErrorKind::InvalidInput, "Find unmatched )"));
                    }
                }
                "}" => {
                    if stack.pop().unwrap() != "{" {
                        return Err(Error::new(ErrorKind::InvalidInput, "Find unmatched }"));
                    }
                }
                "]" => {
                    if stack.pop().unwrap() != "]" {
                        return Err(Error::new(ErrorKind::InvalidInput, "Find unmatched ]"));
                    }
                }
                _ => {}
            }
            ind += 1;
        }
        Ok(ind)
    }

    // content should not have unmatched parentheses
    fn from_tokens(id: &mut i64, content: &Vec<Token>, ind: usize) -> Result<Scope> {
        //check first token
        if &content[ind] != "(" {
            return Err(Error::new(ErrorKind::InvalidInput, "Wrong input."));
        }

        Self::find_wrap_parentheses(ind, content)?;
        *id += 1;

        let mut result = Scope {
            id: *id,
            expression: vec![],
        };

        //clean first "("
        let mut ind_inner = ind + 1;
        loop {
            match content[ind_inner].as_str() {
                "(" => {
                    let end = Self::find_wrap_parentheses(ind_inner, content)?;

                    result
                        .expression
                        .push(ExpressionNode::Scope(Self::from_tokens(
                            id, content, ind_inner,
                        )?));

                    ind_inner = end;
                    continue;
                }
                ")" => {
                    break;
                }
                //:= this part can write some value checker
                _ => {
                    result
                        .expression
                        .push(ExpressionNode::Symbol(content[ind_inner].clone()));
                    ind_inner += 1;
                }
            }
        }

        Ok(result)
    }

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
        scope_search_stack.drain(..1);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::super::scan::*;
    use super::*;
    use std::error::Error;

    #[test]
    fn scope_recursive_test() {
        let testcase0 = scan_str("(defun test (a) (print \"a\"))").unwrap();

        let mut start_id = 0;
        match Scope::from_tokens(&mut start_id, &testcase0, 0) {
            Ok(r) => println!("testcase0 success: {:?}", r),
            Err(e) => println!("testcase0 failed: {}", e),
        }

        //if more ) at endding
        let mut testcase1 = scan_str("(defun a (v) (print \"z\")) (a)").unwrap();
        start_id = 0;
        match Scope::from_tokens(&mut start_id, &mut testcase1, 0) {
            Ok(r) => println!("testcase1 success: {:#?}", r),
            Err(e) => println!("testcase1 failed: {}", e),
        }

        println!(
            "End parentheses index is {:?}",
            Scope::find_wrap_parentheses(0, &testcase1) //=> 11, next scope start from 11 too
        );

        match Scope::from_tokens(&mut start_id, &mut testcase1, 11) {
            Ok(r) => println!("testcase1 second part success: {:#?}", r),
            Err(e) => println!("testcase1 failed: {}", e),
        }
    }

    #[test]
    fn addtional_start_parentheses() {
        // //if more ( at beginning
        let testcase2 = scan_str("((defun test (a) (print \"a\"))").unwrap();

        let mut start_id = 0;
        match Scope::from_tokens(&mut start_id, &testcase2, 0) {
            Ok(r) => println!("testcase2 success: {:?}", r),
            Err(e) => println!("testcase2 failed: {}", e),
        }
    }

    #[test]
    fn scope_table_generate() {
        let testcase0 = scan_str("(defun test (a) (print \"a\"))").unwrap();

        let mut start_id = 0;
        let a = Scope::from_tokens(&mut start_id, &testcase0, 0).unwrap();
        let scopes_table = new_scope_table(&a);
        //println!("{:#?}", a);
        println!("{:#?}", scopes_table);
    }
}
