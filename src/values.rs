use super::eval::*;

#[derive(Debug, Clone)]
pub enum PreValue {
    Expr(Expression),
    Value(Value),
}

impl PreValue {
    pub fn eval(&self, context: &Context) -> Value {
        match self {
            PreValue::Expr(ex) => ex.eval(context),
            PreValue::Value(val) => {
                match val {
                    Value::List(vv) => vv.iter().map(|x| x.eval()),
                    _ => val.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Integer(i64),
    String(String),
    List(Vec<PreValue>),
}

// add integer
pub fn add_integer(x: &Value) -> Value {
    let mut result = 0;
    let values = if let Value::List(values) = x {
        values
    } else {
        panic!();
    };

    for xx in values {
        if let PreValue::Value(Value::Integer(i)) = xx {
            result += i;
        } else {
            panic!();
        }
    }
    return Value::Integer(result);
}
