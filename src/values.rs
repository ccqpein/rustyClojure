#[derive(Debug)]
pub enum Value {
    Nil,
    Integer(i64),
    String(String),
    List(Vec<Value>),
}

pub type Argvs = Vec<Value>;

// add integer
pub fn add_integer(x: &Vec<Value>) -> Value {
    let mut result = 0;
    for xx in x {
        if let Value::Integer(i) = xx {
            result += i;
        } else {
            panic!();
        }
    }
    return Value::Integer(result);
}
