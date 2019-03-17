fn scaner<'a>(input: &'a str) -> Result<Vec<&'a str>> {
    let mut cache: Vec<char> = vec![];
    let mut result: Vec<&'a str> = vec![];
    for c in input.chars() {
        match c {
            '\n' | ' ' => result.push,
            c => cache.push(c),
        }
    }

    vec![""]
}
