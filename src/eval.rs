use super::values::*;

use std::collections::HashMap;

type FuncTable = HashMap<String, fn(&Argvs) -> Value>;

struct Context {
    funcs: FuncTable,
}

struct Expression {
    first: String,
    tail: Vec<Value>,
}

impl Expression {
    fn eval(&self, context: Context) -> Value {
        context.funcs.get(&self.first).unwrap()(&self.tail)
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
            tail: vec![
                Value::Integer(1),
                Value::Integer(1),
                Value::Integer(1),
                Value::Integer(1),
                Value::Integer(1),
            ],
        };

        if let Value::Integer(a) = exp.eval(context) {
            assert!(5 == a);
        } else {
            panic!();
        }
    }
}
