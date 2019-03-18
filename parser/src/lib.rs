use std::iter::FromIterator;

pub fn scaner(input: &str) -> Result<Vec<String>, String> {
    let mut cache: Vec<char> = vec![];
    let mut result: Vec<String> = vec![];

    for c in input.chars() {
        match c {
            '(' | ')' | '[' | ']' | '{' | '}' | '"' => {
                if cache.len() != 0 {
                    result.push(String::from_iter(cache));
                    cache = vec![];
                }
                result.push(c.to_string());
            }
            '\n' | ' ' | '\t' => {
                if cache.len() != 0 {
                    result.push(String::from_iter(cache));
                    cache = vec![];
                }
            }
            _ => cache.push(c),
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io;
    use std::io::Read;

    #[test]
    fn test_scaner() -> io::Result<()> {
        let filepath = "./rw.clj";
        let mut cont = String::new();
        File::open(filepath)?.read_to_string(&mut cont)?;

        println!("{:#?}", scaner(&cont));

        Ok(())
    }
}
