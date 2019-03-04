use rusty_clojure::*;
use std::collections::HashMap;
use std::rc::Rc;

#[test]
fn eval_expression() {
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
