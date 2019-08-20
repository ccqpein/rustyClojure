use parser::parse_file;
use parser::tables::*;
use std::collections::HashMap;
use std::io::Result;

// generator traits
// generator impl

#[derive(Debug)]
struct GoCode {}

trait GoTemplate {
    fn expand(&self, x: &Vec<ExpressionNode>) -> Result<GoCode>;
}

// func keyword of Go
struct Func {}

impl GoTemplate for Func {
    fn expand(&self, x: &Vec<ExpressionNode>) -> Result<GoCode> {
        //:= start to impl here

        Ok(GoCode {})
    }
}

struct GoTemplates {}

impl GoTemplates {
    fn template_expand(&self, x: &SExpression) -> Result<GoCode> {
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
        println!("{:?}", templates.template_expand(&expression0));
    }
}
