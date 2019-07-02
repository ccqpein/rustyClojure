use super::scan::Token;
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};

type SExpressionNum = i64;

// hashtable to store all SExpression number : SExpression
pub type SExpressionTable<'a> = HashMap<SExpressionNum, &'a SExpression>;

// hashtable to store each SExpression and its parent scope number
pub type DependencyTable = HashMap<SExpressionNum, SExpressionNum>;

// lazy_static! {
//     static ref comment_table: HashMap<&'static str, &'static str> = {
//         let mut m = HashMap::new();
//         m.insert(";", "\n");
//         m.insert("#|", "|#");
//         m
//     };
// }

pub struct CommentMarkPair {
    commentsPair: HashMap<String, String>, // start-end
}

impl CommentMarkPair {
    pub fn new() -> Self {
        CommentMarkPair {
            commentsPair: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, val: String) {
        self.commentsPair.insert(key, val);
    }

    pub fn if_comment(&self, this: &Token) -> Option<&String> {
        for (k, v) in self.commentsPair.iter() {
            if this.starts_with(k) {
                return Some(v);
            }
        }
        None
    }

    fn find_end(&self, start: &str) -> Option<&String> {
        self.commentsPair.get(start)
    }
}

#[derive(Debug)]
pub enum ExpressionNode {
    Nil,
    SExpression(SExpression),
    Symbol(String),

    Comments(String),
}

#[derive(Debug)]
pub struct SExpression {
    pub id: SExpressionNum,

    pub expression: Vec<ExpressionNode>,
}

impl SExpression {
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

    // give comment mark and end mark, return comment picker
    fn pick_comment_out_func(
        end_mark: String,
    ) -> impl Fn(&Vec<Token>, &mut usize) -> ExpressionNode {
        move |content: &Vec<Token>, ind: &mut usize| -> ExpressionNode {
            let mut cache = vec![];

            loop {
                cache.push(content[*ind].clone());
                *ind += 1;
                if content[*ind] == end_mark {
                    cache.push(content[*ind].clone());
                    *ind += 1;
                    break;
                }
            }

            ExpressionNode::Comments(cache.join(""))
        }
    }

    fn pick_comment_out(
        content: &Vec<Token>,
        ind: &mut usize,
        end_mark: &String,
    ) -> ExpressionNode {
        let mut cache = vec![];

        loop {
            cache.push(content[*ind].clone());
            *ind += 1;
            if content[*ind] == *end_mark {
                cache.push(content[*ind].clone());
                *ind += 1;
                break;
            }
        }

        ExpressionNode::Comments(cache.join(""))
    }

    // id is total number of scopes
    pub fn from_tokens(
        id: &mut i64,
        content: &Vec<Token>,
        ind: usize,
        comment_mark: &CommentMarkPair,
    ) -> Result<SExpression> {
        //check first token
        if &content[ind] != "(" {
            return Err(Error::new(ErrorKind::InvalidInput, "Wrong input."));
        }

        Self::find_wrap_parentheses(ind, content)?;

        let mut result = SExpression {
            id: *id,
            expression: vec![],
        };
        *id += 1;

        //clean first "("
        let mut ind_inner = ind + 1;
        loop {
            match content[ind_inner].as_str() {
                "(" => {
                    let end = Self::find_wrap_parentheses(ind_inner, content)?;

                    result
                        .expression
                        .push(ExpressionNode::SExpression(Self::from_tokens(
                            id,
                            content,
                            ind_inner,
                            comment_mark,
                        )?));

                    ind_inner = end;
                    continue;
                }
                ")" => {
                    break;
                }
                "\n" => {
                    ind_inner += 1;
                }
                _ => {
                    let end_mark = comment_mark.if_comment(&content[ind_inner]);
                    if let Some(end_) = end_mark {
                        result.expression.push(Self::pick_comment_out(
                            content,
                            &mut ind_inner,
                            end_,
                        ))
                    } else {
                        result
                            .expression
                            .push(ExpressionNode::Symbol(content[ind_inner].clone()));
                        ind_inner += 1;
                    }
                }
            }
        }

        Ok(result)
    }

    fn add_to_scope_table<'a, 'b: 'a>(&'b self, target: &mut SExpressionTable<'a>) {
        target.insert(self.id, self);
    }

    fn all_children_scopes(&self) -> Vec<&ExpressionNode> {
        self.expression
            .iter()
            .filter(|x| {
                if let ExpressionNode::SExpression(_) = x {
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<_>>()
    }
}

pub fn new_expression_table<'a>(scope: &'a SExpression) -> Result<SExpressionTable<'a>> {
    let mut result: SExpressionTable<'a> = HashMap::new();
    result.insert(scope.id, scope);

    let mut scope_search_stack = scope.all_children_scopes();

    while !scope_search_stack.is_empty() {
        let first: &SExpression = if let Some(f) = scope_search_stack.first() {
            if let ExpressionNode::SExpression(s) = f {
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

// use recursive to do this
pub fn new_dependency_table(scope: &SExpression) -> Result<DependencyTable> {
    let mut result: DependencyTable = HashMap::new();

    let mut scope_search_stack = vec![scope];
    let mut cache_scope_stack = vec![];

    while !scope_search_stack.is_empty() {
        cache_scope_stack.clear();
        for scope in &scope_search_stack {
            for child in scope.all_children_scopes() {
                match child {
                    ExpressionNode::SExpression(s) => {
                        result.insert(s.id, scope.id);
                        cache_scope_stack.push(s);
                    }
                    _ => (),
                }
            }
        }
        scope_search_stack = cache_scope_stack.clone();
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::super::scan::*;
    use super::*;
    //use std::error::Error;

    //#[test]
    fn scope_recursive_test() {
        let mut comment_key_pair = CommentMarkPair::new();
        comment_key_pair.insert(String::from(";"), String::from("\n"));

        let testcase0 = scan_str("(defun test (a) (print \"a\"))").unwrap();

        let mut start_id = 0;
        match SExpression::from_tokens(&mut start_id, &testcase0, 0, &comment_key_pair) {
            Ok(r) => println!("testcase0 success: {:?}", r),
            Err(e) => println!("testcase0 failed: {}", e),
        }

        //if more ) at endding
        let mut testcase1 = scan_str("(defun a (v) (print \"z\")) (a)").unwrap();
        start_id = 0;
        match SExpression::from_tokens(&mut start_id, &mut testcase1, 0, &comment_key_pair) {
            Ok(r) => println!("testcase1 success: {:#?}", r),
            Err(e) => println!("testcase1 failed: {}", e),
        }

        println!(
            "End parentheses index is {:?}",
            SExpression::find_wrap_parentheses(0, &testcase1) //=> 11, next scope start from 11 too
        );

        match SExpression::from_tokens(&mut start_id, &mut testcase1, 11, &comment_key_pair) {
            Ok(r) => println!("testcase1 second part success: {:#?}", r),
            Err(e) => println!("testcase1 failed: {}", e),
        }
    }

    //#[test]
    fn addtional_start_parentheses() {
        let mut comment_key_pair = CommentMarkPair::new();
        comment_key_pair.insert(String::from(";"), String::from("\n"));

        // //if more ( at beginning
        let testcase2 = scan_str("((defun test (a) (print \"a\"))").unwrap();

        let mut start_id = 0;
        match SExpression::from_tokens(&mut start_id, &testcase2, 0, &comment_key_pair) {
            Ok(r) => println!("testcase2 success: {:?}", r),
            Err(e) => println!("testcase2 failed: {}", e),
        }
    }

    //#[test]
    fn addtional_start_and_end_parentheses() {
        let mut comment_key_pair = CommentMarkPair::new();
        comment_key_pair.insert(String::from(";"), String::from("\n"));

        // //if more ( at beginning
        let testcase3 = scan_str("((defun test (a) (print \"a\")))").unwrap();

        let mut start_id = 0;
        match SExpression::from_tokens(&mut start_id, &testcase3, 0, &comment_key_pair) {
            Ok(r) => println!("testcase3 success: {:#?}", r),
            Err(e) => println!("testcase3 failed: {}", e),
        }
    }

    //#[test]
    fn scope_table_generate() {
        let mut comment_key_pair = CommentMarkPair::new();
        comment_key_pair.insert(String::from(";"), String::from("\n"));

        let testcase0 = scan_str("(defun test (a) (print \"a\"))").unwrap();

        let mut start_id = 0;
        let a = SExpression::from_tokens(&mut start_id, &testcase0, 0, &comment_key_pair).unwrap();
        let scopes_table = new_expression_table(&a);
        //println!("{:#?}", a);
        println!("Here is scope table: {:#?}", scopes_table);
    }

    //#[test]
    fn dependency_table_generate() {
        let mut comment_key_pair = CommentMarkPair::new();
        comment_key_pair.insert(String::from(";"), String::from("\n"));

        let testcase0 = scan_str("(defun test (a) (print (cons 1 2)))").unwrap();

        let mut start_id = 0;
        let a = SExpression::from_tokens(&mut start_id, &testcase0, 0, &comment_key_pair).unwrap();

        let scopes_table = new_expression_table(&a);
        let dependency_table = new_dependency_table(&a);

        println!("Here is scope table: {:#?}", scopes_table);
        println!("Here is dependency table: {:#?}", dependency_table);
    }

    //#[test]
    fn find_comments() {
        let mut comment_key_pair = CommentMarkPair::new();
        comment_key_pair.insert(String::from(";"), String::from("\n"));

        let testcase0 = scan_str(
            "(defun test ()
;aaaaaaa
;; abs
;;bbb

(print \"a\"))",
        )
        .unwrap();
        let mut start_id = 0;
        let a = SExpression::from_tokens(&mut start_id, &testcase0, 0, &comment_key_pair).unwrap();

        println!("{:#?}", a);
    }

    #[test]
    fn find_comments2() {
        let mut comment_key_pair = CommentMarkPair::new();
        comment_key_pair.insert(String::from(";"), String::from("\n"));
        comment_key_pair.insert(String::from("#|"), String::from("|#"));

        let testcase0 = scan_str(
            "(#|
jisdjfijsifjdsi
|#
(defun test ()
;; aaa
(print \"a\")))",
        )
        .unwrap();
        let mut start_id = 0;
        println!("{:?}", testcase0);
        let a = SExpression::from_tokens(&mut start_id, &testcase0, 0, &comment_key_pair).unwrap();

        println!("{:#?}", a);
    }
}
