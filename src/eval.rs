use super::values::*;
use std::rc::Rc;

use std::collections::HashMap;

type FuncTable = HashMap<String, fn(&Value) -> Value>;

pub struct Context {
    funcs: FuncTable,
}

#[derive(Debug, Clone)]
pub struct Expression {
    first: String,
    tail: Rc<PreValue>,
}

impl Expression {
    pub fn eval(&self, context: &Context) -> Value {
        context.funcs.get(&self.first).unwrap()(&self.tail.eval(context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_func() {
        let mut context = Context {
            funcs: HashMap::new(),
        };
    }

    #[test]
    fn eval_expression() {
        let mut context = Context {
            funcs: HashMap::new(),
        };
        context.funcs.insert("Add".to_string(), add_integer);

        let exp = Expression {
            first: "Add".to_string(),
            tail: Rc::new(PreValue::Value(Value::List(vec![
                PreValue::Value(Value::Integer(1)),
                PreValue::Value(Value::Integer(1)),
                PreValue::Value(Value::Integer(1)),
                PreValue::Value(Value::Integer(1)),
                PreValue::Value(Value::Integer(1)),
            ]))),
        };

        if let Value::Integer(a) = exp.eval(&context) {
            assert!(5 == a);
        } else {
            panic!();
        }

        //test expression in expression
        //(+ (+ 1 1 1 1 1) 1)
        let exp1 = Expression {
            first: "Add".to_string(),
            tail: Rc::new(PreValue::Value(Value::List(vec![
                PreValue::Expr(exp),
                PreValue::Value(Value::Integer(1)),
            ]))),
        };

        if let Value::Integer(a) = exp1.eval(&context) {
            assert!(6 == a);
        } else {
            panic!();
        }
    }
}
