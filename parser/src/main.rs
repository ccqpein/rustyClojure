use clap::{App, Arg};
use std::io::Result;

fn main() -> Result<()> {
    let app = App::new("rustyLispParser")
        .arg(
            Arg::with_name("INPUT")
                .short("i")
                .long("input")
                .value_name("FILE")
                .required(true),
        )
        .get_matches();

    match app.value_of("INPUT") {
        Some(i) => {
            let se = parser::parse_file(i)?;
            let tables = parser::make_parser_table(&se)?;
            println!("{:#?}", tables.expression_table.get(&(0 as i64)));
            Ok(())
        }
        None => Ok(()),
    }
}
