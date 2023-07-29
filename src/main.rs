use std::io::{stdout, Write, BufWriter};
use std::num::ParseIntError;

#[derive(Debug)]
#[derive(PartialEq)]
enum RenbanError {
    InvalidArgs,
    InvalidRange,
    ParseError(ParseIntError)
}

fn create_renban(s: &str) -> Result<Vec<String>, RenbanError> {
    let hyphen_offset = s.find('-').ok_or(RenbanError::InvalidArgs)?;
    let left = &s[0..hyphen_offset];
    let right = &s[hyphen_offset+1..];

    let from = left.parse::<i32>().map_err(|e| RenbanError::ParseError(e))?;
    let to = right.parse::<i32>().map_err(|e| RenbanError::ParseError(e))?;
    if to < from {
        return Err(RenbanError::InvalidRange)
    }

    if !left.starts_with('0') {
        return Ok((from..=to).map(|n| n.to_string()).collect());
    }

    let width = left.len();
    Ok((from..=to).map(|n| format!("{:0width$}", n, width = width)).collect())
}

fn renban1range(s: &str) -> Result<Vec<String>, RenbanError> {
    let left_bracket = s.find('[').ok_or(RenbanError::InvalidArgs)?;
    let right_bracket = s.find(']').ok_or(RenbanError::InvalidArgs)?;

    let left = &s[0..left_bracket];
    let middle = &s[left_bracket+1..right_bracket];
    let right = &s[right_bracket+1..];
    Ok(create_renban(middle)?.iter().map(|n| left.to_owned() + n + right).collect())
}

fn renban(s: &str) -> Result<Vec<String>, RenbanError> {
    let list = renban1range(s)?;
    if list.first().unwrap().find('[').is_none() {
        return Ok(list);
    }

    let mut expanded_list: Vec<String> = Vec::new();
    for i in list {
        expanded_list.extend(renban(&i)?);
    }
    Ok(expanded_list)
}

fn main() {
    let input: String = std::env::args().nth(1).unwrap();

    let mut out = BufWriter::new(stdout().lock());
    let new_line = "\n".as_bytes();
    match renban(&input) {
        Ok(list) => {
            for i in list {
                out.write(i.as_bytes()).unwrap();
                out.write(new_line).unwrap();
            }
        },
        Err(_e) => {
            out.write(input.as_bytes()).unwrap();
        },
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_renban_test() {
        assert_eq!(create_renban("3-5"), Ok(["3", "4", "5"].iter().map(|&s| s.into()).collect()));
        assert_eq!(create_renban("1-1"), Ok(["1"].iter().map(|&s| s.into()).collect()));
        assert_eq!(create_renban("01-03"), Ok(["01", "02", "03"].iter().map(|&s| s.into()).collect()));
        assert_eq!(create_renban("09-10"), Ok(["09", "10"].iter().map(|&s| s.into()).collect()));
        assert_eq!(create_renban("001-001"), Ok(["001"].iter().map(|&s| s.into()).collect()));
        assert_eq!(create_renban("000-000"), Ok(["000"].iter().map(|&s| s.into()).collect()));
        assert_eq!(create_renban("100-101"), Ok(["100", "101"].iter().map(|&s| s.into()).collect()));
        assert_eq!(create_renban("2-1"), Err(RenbanError::InvalidRange));
        assert!(matches!(create_renban("a-b"), Err(RenbanError::ParseError(_))));
        assert_eq!(create_renban("a"), Err(RenbanError::InvalidArgs));
        assert_eq!(create_renban("3"), Err(RenbanError::InvalidArgs));
    }

    #[test]
    fn renban1range_test() {
        assert_eq!(renban1range("http://example.com/img[1-2]s.jpg"), Ok(["http://example.com/img1s.jpg", "http://example.com/img2s.jpg"].iter().map(|&s| s.into()).collect()));
    }

    #[test]
    fn renban_test() {
        assert_eq!(renban("http://example.com/img[1-2][3-4]s.jpg"), Ok(["http://example.com/img13s.jpg", "http://example.com/img14s.jpg", "http://example.com/img23s.jpg", "http://example.com/img24s.jpg"].iter().map(|&s| s.into()).collect()));
    }
}
