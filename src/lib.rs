use std::collections::HashMap;
use std::rc::Rc;

type FuncTable = HashMap<String, fn(&Node) -> Node>;

pub struct Context {
    pub funcs: FuncTable,
}

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Integer(i64),
    String(String),
    List(Vec<Value>),
}

#[derive(Debug, Clone)]
pub enum Node {
    Nil,
    Expr(String),
    Value(Value),
    NodeT(NodeTree),

    //special enum for eval
    Argvs(Vec<Node>),
}

#[derive(Debug, Clone)]
pub struct NodeTree {
    first: Rc<Node>,
    tail: Rc<Option<NodeTree>>,
}

impl NodeTree {
    fn eval(&self, context: &Context) -> Node {
        match self.first.as_ref() {
            Node::Expr(f) => match self.tail.as_ref() {
                Some(temp) => match temp.eval(context) {
                    Node::Argvs(v) => context.funcs.get(f).unwrap()(&Node::Argvs(v)),
                    _ => Node::Value(Value::Nil),
                },
                None => Node::Nil,
            },

            Node::Value(v) => {
                let mut temp = vec![Node::Value(v.clone())];
                let tail = match self.tail.as_ref() {
                    Some(nt) => nt.eval(context),
                    None => Node::Nil,
                };

                match tail {
                    Node::Argvs(mut vn) => temp.append(&mut vn),
                    Node::Value(v) => temp.append(&mut vec![Node::Value(v)]),
                    _ => (),
                }
                Node::Argvs(temp)
            }

            Node::NodeT(n) => {
                let temp_node = NodeTree {
                    first: Rc::new(n.eval(context)),
                    tail: self.tail.clone(),
                };
                temp_node.eval(context)
            }

            Node::Argvs(vec_node) => {
                let mut newvec = vec_node.clone();
                let mut tail = vec![match self.tail.as_ref() {
                    Some(nt) => nt.eval(context),
                    None => Node::Nil,
                }];

                newvec.append(&mut tail);
                Node::Argvs(newvec)
            }

            Node::Nil => Node::Nil,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_expression2() {
        let mut context = Context {
            funcs: HashMap::new(),
        };

        //(+ (+ 1 1) 1)
        //add function
        fn add_test(v: &Node) -> Node {
            let mut result = 0;
            let values = if let Node::Argvs(values) = v {
                values
            } else {
                panic!()
            };

            println!("values is {:?}", values);
            for xx in values {
                if let Node::Value(Value::Integer(i)) = xx {
                    result += i;
                } else {
                    //panic!();
                }
            }
            println!("result is : {}", result);
            return Node::Value(Value::Integer(result));
        }

        context.funcs.insert("Add".to_string(), add_test);

        //(+ 1)
        // let nt = NodeTree {
        //     first: Rc::new(Node::Expr("Add".to_string())),
        //     tail: Rc::new(Some(NodeTree {
        //         first: Rc::new(Node::Value(Value::Integer(1))),
        //         tail: Rc::new(None),
        //     })),
        // };

        // match nt.eval(&context) {
        //     Node::Value(Value::Integer(v)) => {
        //         println!("{}", v);
        //         assert_eq!(v, 1)
        //     }
        //     _ => {}
        // }

        //(+ 1 1)
        let nt2 = NodeTree {
            first: Rc::new(Node::Expr("Add".to_string())),
            tail: Rc::new(Some(NodeTree {
                first: Rc::new(Node::Value(Value::Integer(1))),
                tail: Rc::new(Some(NodeTree {
                    first: Rc::new(Node::Value(Value::Integer(1))),
                    tail: Rc::new(None),
                })),
            })),
        };

        // match nt2.eval(&context) {
        //     Node::Value(Value::Integer(v)) => {
        //         println!("{}", v);
        //         assert_eq!(v, 2)
        //     }
        //     _ => {}
        // }

        //(+ 1 (+ 1 1))
        let nt3 = NodeTree {
            first: Rc::new(Node::Expr("Add".to_string())),
            tail: Rc::new(Some(NodeTree {
                first: Rc::new(Node::Value(Value::Integer(1))),
                tail: Rc::new(Some(nt2)),
            })),
        };

        println!("nt3 is {:?}", nt3);

        match nt3.eval(&context) {
            Node::Value(Value::Integer(v)) => {
                println!("{}", v);
                assert_eq!(v, 3)
            }
            _ => {}
        }
    }
}
