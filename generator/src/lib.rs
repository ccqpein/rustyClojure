use parser::parse_file;
use parser::tables;
mod go_generator;

trait Generator {
    type Template;
    type Keyword;

    type Result;

    // return template for specific keyword
    fn keyword_template(&self, k: &Self::Keyword) -> Self::Template;

    // give template and SExpression, return code
    fn match_template(&self, t: &Self::Template, se: &tables::SExpression) -> Self::Result;
}
