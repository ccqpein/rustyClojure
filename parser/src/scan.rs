use std::string::FromUtf8Error;
pub type Token = String;

pub fn scan_str(input: &str) -> Result<Vec<Token>, FromUtf8Error> {
    let mut result: Vec<String> = vec![];

    let mut all_bytes = input.as_bytes().to_vec();

    while all_bytes.len() != 0 {
        for ind in 0..all_bytes.len() {
            match all_bytes[ind] {
                b'(' | b')' | b'[' | b']' | b'{' | b'}' | b'\n' => {
                    if ind != 0 {
                        let w = all_bytes.drain(0..ind).collect::<Vec<_>>();
                        result.push(String::from_utf8(w)?);
                    }
                    let w = all_bytes.drain(..1).collect::<Vec<_>>();
                    result.push(String::from_utf8(w)?);
                    break;
                }
                b' ' | b'\t' => {
                    if ind != 0 {
                        let w = all_bytes.drain(0..ind).collect::<Vec<_>>();
                        result.push(String::from_utf8(w)?);
                    }
                    all_bytes.drain(..1);
                    break;
                }
                _ => (),
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    //use std::fs::File;
    use std::io;
    //use std::io::Read;

    #[test]
    fn test_scaner() -> io::Result<()> {
        // let filepath = "./rw.lisp";
        // let mut cont = String::new();
        // File::open(filepath)?.read_to_string(&mut cont)?;

        let cont = "(defun test (a)
  (print \"a\"))";
        println!("{:?}", scan_str(cont));
        assert_eq!(scan_str(cont).unwrap().len(), 12);

        Ok(())
    }

    #[test]
    fn test_comments_inside() {
        let cont = "(defun test (a)
;aaaaaaa
; ddd
;; asd
(print \"a\"))
)";
        println!("{:?}", scan_str(cont));
    }
}
