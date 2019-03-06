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
    Float(f64),
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
    pub first: Rc<Node>,
    pub tail: Rc<Option<NodeTree>>,
}

impl NodeTree {
    pub fn eval(&self, context: &Context) -> Node {
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
