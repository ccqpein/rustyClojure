use lazy_static::*;
use std::cell::RefCell;
use std::collections::HashMap;

lazy_static! {
    static ref dependency_table: HashMap<u32, &'static str> = HashMap::new();
}
/*
scope stack

 */

type ScopeNum = i64;

pub enum ExpressionNode {
    Nil,
    Int(i64),
    Scope(Scope),
}

pub struct Scope {
    pub id: ScopeNum,

    pub expression: Vec<ExpressionNode>,

    pub children: Vec<RefCell<Scope>>,
}

impl Scope {
    pub fn new(current_scope: i64) -> Scope {
        Scope {
            id: current_scope + 1,
            children: vec![],
            expression: vec![],
        }
    }
}
