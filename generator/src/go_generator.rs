use parser::parse_file;
use parser::tables::*;
use std::collections::HashMap;
use std::io::Result;

// generator traits
// generator impl

struct GoCode {}

struct GoTemplate {}

impl GoTemplate {
    fn template_expand(&self, &Vec<SExpression>) -> Result<GoCode> {
        //:= from here
    }
}

struct GoTemplates {}

struct GoGenerator {
    keywords_and_temp: HashMap<ExpressionNode, String>, //:= TODO: need find template module for this
    user_space_definition: HashMap<ExpressionNode, String>, //:= TODO: maybe not string
}

impl GoGenerator {}

//:= TODO: need tamplate engine to finish this
// impl super::Generator for GoGenerator {
//     type Template = String;
//     type Keyword = ExpressionNode;

//     type Result = String;

//     fn keyword_template(&self, k: &Self::Keyword) -> String {
//         self.keywords_and_temp.get(k).unwrap().to_string()
//     }

//     //:= fake implenment
//     fn match_template(&self, t: &String, se: &SExpression) -> String {
//         String::new()
//     }
// }
