use std::num::ParseIntError;

#[derive(Debug)]
#[derive(PartialEq)]
enum ExpandError {
    InvalidArgs,
    InvalidRange,
    ParseError(ParseIntError)
}

fn expand_numbers(s: &str) -> Result<Vec<String>, ExpandError> {
    let hyphen_offset = s.find('-').ok_or(ExpandError::InvalidArgs)?;
    let left = &s[0..hyphen_offset];
    let right = &s[hyphen_offset+1..];

    let from = left.parse::<i32>().map_err(|e| ExpandError::ParseError(e))?;
    let to = right.parse::<i32>().map_err(|e| ExpandError::ParseError(e))?;
    if to < from {
        return Err(ExpandError::InvalidRange)
    }

    if !left.starts_with('0') {
        return Ok((from..=to).map(|n| n.to_string()).collect());
    }

    let width = left.len();
    Ok((from..=to).map(|n| format!("{:0width$}", n, width = width)).collect())
}

fn expand_one(s: &str) -> Result<Vec<String>, ExpandError> {
    let left_bracket = s.find('[').ok_or(ExpandError::InvalidArgs)?;
    let right_bracket = s.find(']').ok_or(ExpandError::InvalidArgs)?;

    let left = &s[0..left_bracket];
    let middle = &s[left_bracket+1..right_bracket];
    let right = &s[right_bracket+1..];
    Ok(expand_numbers(middle)?.iter().map(|n| left.to_owned() + n + right).collect())
}

fn expand_url(url: &str) -> Result<Vec<String>, ExpandError> {
    let list = expand_one(url)?;
    if list.first().unwrap().find('[').is_none() {
        return Ok(list);
    }

    let mut expanded_list: Vec<String> = Vec::new();
    for i in list {
        expanded_list.extend(expand_url(&i)?);
    }
    Ok(expanded_list)
}

fn main() {
    let url: String = std::env::args().nth(1).unwrap();
    match expand_url(&url) {
        Ok(url_list) => print!("{}", url_list.join("\n")),
        Err(_e) => print!("{}", url),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_numbers_test() {
        assert_eq!(expand_numbers("3-5"), Ok(["3", "4", "5"].iter().map(|&s| s.into()).collect()));
        assert_eq!(expand_numbers("1-1"), Ok(["1"].iter().map(|&s| s.into()).collect()));
        assert_eq!(expand_numbers("01-03"), Ok(["01", "02", "03"].iter().map(|&s| s.into()).collect()));
        assert_eq!(expand_numbers("09-10"), Ok(["09", "10"].iter().map(|&s| s.into()).collect()));
        assert_eq!(expand_numbers("001-001"), Ok(["001"].iter().map(|&s| s.into()).collect()));
        assert_eq!(expand_numbers("000-000"), Ok(["000"].iter().map(|&s| s.into()).collect()));
        assert_eq!(expand_numbers("100-101"), Ok(["100", "101"].iter().map(|&s| s.into()).collect()));
        assert_eq!(expand_numbers("2-1"), Err(ExpandError::InvalidRange));
        assert!(matches!(expand_numbers("a-b"), Err(ExpandError::ParseError(_))));
        assert_eq!(expand_numbers("a"), Err(ExpandError::InvalidArgs));
        assert_eq!(expand_numbers("3"), Err(ExpandError::InvalidArgs));
    }

    #[test]
    fn expand_one_test() {
        assert_eq!(expand_one("http://example.com/img[1-2]s.jpg"), Ok(["http://example.com/img1s.jpg", "http://example.com/img2s.jpg"].iter().map(|&s| s.into()).collect()));
    }

    #[test]
    fn expand_url_test() {
        assert_eq!(expand_url("http://example.com/img[1-2][3-4]s.jpg"), Ok(["http://example.com/img13s.jpg", "http://example.com/img14s.jpg", "http://example.com/img23s.jpg", "http://example.com/img24s.jpg"].iter().map(|&s| s.into()).collect()));
    }
}
