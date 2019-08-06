use parser::parse_file;
use parser::tables;
mod go_generator;

trait Generator {
    type Template;
    type Keyword;

    type Result;

    fn keyword_template(&self, k: &Self::Keyword) -> Self::Template;

    fn match_template(&self, t: &Self::Template, se: &tables::SExpression) -> Self::Result;
}
