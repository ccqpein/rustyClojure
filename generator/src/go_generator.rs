use parser::parse_file;
use parser::tables::*;
use std::collections::HashMap;

// generator traits
// generator impl

struct GoGenerator {
    keywords_and_temp: HashMap<ExpressionNode, String>, //:= TODO: need find template module for this
}

//:= TODO: need tamplate engine to finish this
// impl Generator for GoGenerator {
//     type Template = String  ;
//     type Keyword = String;

//     type Result = String;
// }
