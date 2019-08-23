use parser::parse_file;
use parser::tables::*;
use std::collections::HashMap;
use std::io::Result;

// generator traits
// generator impl

trait GoCode {
    fn to_string(&self) -> String;
}

#[derive(Debug)]
struct FuncCode {
    name: String,
    argvs: Vec<String>,
    returnV: Vec<String>,
    body: Vec<String>,
}

impl FuncCode {
    fn new() -> Self {
        FuncCode {
            name: String::new(),
            argvs: vec![],
            returnV: vec![],
            body: vec![],
        }
    }
}

impl GoCode for FuncCode {
    fn to_string(&self) -> String {
        String::new()
    }
}

// func keyword of Go
struct Func {}

trait GoTemplate {
    fn expand(&self, x: &Vec<ExpressionNode>) -> Result<Box<dyn GoCode>>;
}

impl GoTemplate for Func {
    fn expand(&self, x: &Vec<ExpressionNode>) -> Result<Box<dyn GoCode>> {
        //:= start to impl here
        let mut f = FuncCode::new();
        for (ind, n) in x.iter().enumerate() {
            match n {
                ExpressionNode::Symbol(s) => {
                    match ind {
                        // function name
                        0 => f.name = s.clone(),
                        _ => (),
                    }
                }
                ExpressionNode::SExpression(e) => {
                    match ind {
                        1 => f.argvs = vec![], //:= TODO: use ind to determine which is which
                        _ => (),
                    }
                }
                _ => (),
            }
        }

        Ok(Box::new(f))
    }
}

struct GoTemplates {}

impl GoTemplates {
    fn template_expand(&self, x: &SExpression) -> Result<Box<dyn GoCode>> {
        let keyw = x.expression[0].clone();
        let templ = self.find_template(keyw)?;

        let bodys = x
            .expression
            .clone()
            .drain(1..)
            .collect::<Vec<ExpressionNode>>();
        templ.expand(&bodys)
    }

    fn find_template(&self, _keyw: ExpressionNode) -> Result<impl GoTemplate> {
        Ok(Func {})
    }
}

// struct GoGenerator {
//     keywords_and_temp: HashMap<ExpressionNode, String>, //:= TODO: need find template module for this
//     user_space_definition: HashMap<ExpressionNode, String>, //:= TODO: maybe not string
// }

// impl GoGenerator {}

// Tests below
#[cfg(test)]
mod test {

    use super::*;
    use parser::parse_file;
    use parser::scan::scan_str;
    use parser::tables::*;

    fn test_func() {
        let mut comment_key_pair = CommentMarkPair::new();
        comment_key_pair.insert(String::from(";"), String::from("\n"));

        let mut start_id = 0;
        let testcase0 = scan_str("(defun test (a b) (print a) (print b))").unwrap();
        let expression0 =
            SExpression::from_tokens(&mut start_id, &testcase0, 0, &comment_key_pair).unwrap();

        let templates = GoTemplates {};
        //println!("{:?}", templates.template_expand(&expression0));
    }
}
