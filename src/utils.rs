pub enum Take<'t> {
    InsufficientBuffer,
    InvalidCharBoundary,
    Split(&'t str, &'t str)
}

pub enum Skip<'s> {
    InsufficientBuffer,
    InvalidCharBoundary,
    Rest(&'s str),
}

#[inline(always)]
pub fn take(buffer: &str, count : usize) -> Take {
    if buffer.len() < count {
        return Take::InsufficientBuffer;
    } else if !buffer.is_char_boundary(count) {
        return Take::InvalidCharBoundary;
    }
    Take::Split(&buffer[..count], &buffer[count..])
}

#[inline(always)]
pub fn skip(buffer: &str, count : usize) -> Skip {
    if buffer.len() < count {
        return Skip::InsufficientBuffer;
    } else if !buffer.is_char_boundary(count) {
        return Skip::InvalidCharBoundary;
    }
    Skip::Rest(&buffer[count..])
}

#[inline(always)]
pub fn is_whitespace(chr: char) -> bool {
    chr == ' ' || chr == '\t' || chr == '\r' || chr == '\n'
}

#[inline(always)]
pub fn trim(buffer: &str) -> &str {
    triml(trimr(buffer))
}

#[inline]
pub fn trimr(buffer: &str) -> &str {
    let mut count_ws = 0;
    let mut rchars = buffer.chars().rev();

    while let Some(c) = rchars.next() {
        if !is_whitespace(c) {
            break;
        }
        count_ws += 1;
    }
    &buffer[..buffer.len() - count_ws]
}

#[inline]
pub fn triml(buffer: &str) -> &str {
    let mut count_ws = 0;
    let mut rchars = buffer.chars();

    while let Some(c) = rchars.next() {
        if !is_whitespace(c) {
            break;
        }
        count_ws += 1;
    }
    &buffer[count_ws..]
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