#[derive(Debug)]
pub enum Error {
    InsufficientBuffer,
    InvalidCharBoundary,
    EmptyBuffer,
    TagNotEqual,
    InvalidPWord,
    TrimingError,
}

#[derive(Debug)]
pub struct Split<'s> {
    pub left: &'s str,
    pub right: &'s str,
}

impl<'s> Split<'s> {
    pub fn new(left: &'s str, right: &'s str) -> Split<'s> {
        Split { left: left, right: right }
    }
}

pub fn tag<'b, 'v>(buffer: &'b str, value: &'v str) -> Result<Split<'b>, Error> {
    let rtake = take(buffer, value.len());

    match rtake {
        Err(e) => Err(e),
        Ok(Split {left, right}) if left == value => Ok(Split::new(left, right)),
        Ok(_) => Err(Error::TagNotEqual),
    }
}

#[inline]
pub fn take(buffer: &str, count : usize) -> Result<Split, Error> {
    if buffer.len() < count {
        return Err(Error::InsufficientBuffer);
    } else if !buffer.is_char_boundary(count) {
        return Err(Error::InvalidCharBoundary);
    }
    Ok(Split::new(&buffer[..count], &buffer[count..]))
}

#[inline]
pub fn skip(buffer: &str, count : usize) -> Result<Split, Error> {
    take(buffer, count)
}

// Stolen from NOM
/// Tests if byte is ASCII alphabetic: A-Z, a-z
#[inline]
pub fn is_alphabetic(chr:u8) -> bool {
  (chr >= 0x41 && chr <= 0x5A) || (chr >= 0x61 && chr <= 0x7A)
}

// Stolen from NOM
/// Tests if byte is ASCII digit: 0-9
#[inline]
pub fn is_digit(chr: u8) -> bool {
  chr >= 0x30 && chr <= 0x39
}

#[inline]
pub fn is_blank(chr: char) -> bool {
    chr == ' ' || chr == '\t' || chr == '\r' || chr == '\n'
}

/// Captures characters until a no pword char is found
/// Note pword starts with A-Z or a-z or `_` and can have A-Z, a-z, `_`
/// and 0-9 after the first char
pub fn pword<'w>(buffer: &'w str) -> Result<Split, Error> {
    let bytes = buffer.as_bytes();
    let mut i : usize = 0;

    // first char must be alpha or '_'
    if bytes.len() == 0 {
        return Err(Error::EmptyBuffer);
    } else if !is_alphabetic(bytes[0]) && bytes[i] != b'_' {
        return Err(Error::InvalidPWord);
    } else {
        i += 1;
    }

    loop {
        if i > bytes.len() {
            return Ok(Split::new(&buffer[0..i], &buffer[i..]));
        }
                
        if !is_digit(bytes[i]) && !is_alphabetic(bytes[i]) && bytes[i] != b'_' {
            return Ok(Split::new(&buffer[0..i], &buffer[i..]));
        } else {
            i += 1;
        }
    }
}
/// Takes bytes while blank (space, tab, \n \r)
pub fn blank<'a, 'b>(buffer: &'a str) -> Result<Split, Error> {
    triml(buffer)
}
/// Takes bytes until the end of the buffer or until an `ending` is found
/// Result does not include ending
pub fn until<'a, 'b>(buffer: &'a str, ending: &[&'b str]) -> Result<Split<'a>, Error> {
    let mut i = 0;
    let mut temp = buffer;
    loop {
        for end in ending {
            let temp = &buffer[i..];
            if temp.starts_with(end) {
                return Ok(Split::new(&buffer[0..i], &buffer[i..]));
            }
        }
        if i < temp.len() {
            i += 1;
        } else {
            break;
        }
    }
    return Ok(Split::new(&buffer[..], &buffer[i..]));
}

#[inline]
pub fn trim(buffer: &str) -> Result<&str, Error> {
    let result = triml(buffer);

    if let Ok(Split { left, right }) = result {
        let result = trimr(right);
        if let Ok(Split { left, right }) = result {
            return Ok(left);
        }
    }
    Err(Error::TrimingError)
}

#[inline]
pub fn trimr(buffer: &str) -> Result<Split, Error> {
    let mut count_ws = 0;
    let mut rchars = buffer.chars().rev();

    while let Some(c) = rchars.next() {
        if !is_blank(c) {
            break;
        }
        count_ws += 1;
    }
    let result_count = buffer.len() - count_ws;
    Ok(Split::new(&buffer[..result_count], &buffer[result_count..]))
}

#[inline]
pub fn triml(buffer: &str) -> Result<Split, Error> {
    let mut count_ws = 0;
    let mut rchars = buffer.chars();

    while let Some(c) = rchars.next() {
        if !is_blank(c) {
            break;
        }
        count_ws += 1;
    }
    Ok(Split::new(&buffer[0..count_ws], &buffer[count_ws..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STRING: &'static str = "\t\r \nn͈̰̎u͔n͈̰̎i̙̮͚̦c͚̉o̼̩̰͗d͔̆̓ͥé\n\t   \t\r\r";

    #[test]
    fn triml_test() {
        assert_eq!("n͈̰̎u͔n͈̰̎i̙̮͚̦c͚̉o̼̩̰͗d͔̆̓ͥé\n\t   \t\r\r",
                   triml(TEST_STRING));
    }
    #[test]
    fn trimr_test() {
        assert_eq!("\t\r \nn͈̰̎u͔n͈̰̎i̙̮͚̦c͚̉o̼̩̰͗d͔̆̓ͥé",
                   trimr(TEST_STRING));
    }
    #[test]
    fn trim_test() {
        assert_eq!("n͈̰̎u͔n͈̰̎i̙̮͚̦c͚̉o̼̩̰͗d͔̆̓ͥé",
                   trim(TEST_STRING));
    }
}