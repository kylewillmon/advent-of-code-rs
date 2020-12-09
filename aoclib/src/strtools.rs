
/// Split a str into exactly two substrings. If the delimeter isn't found in the string, (s, "") is returned
pub fn split_once<'a, 'b>(s: &'a str, delimeter: &'b str) -> (&'a str, &'a str) {
    if let Some(index) = s.find(delimeter) {
        let (head, tail) = s.split_at(index);
        let (_, tail) = tail.split_at(delimeter.len());
        (head, tail)
    } else {
        (s, "")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_once() {
        assert_eq!(("a", "b"), split_once("a123b", "123"));
        assert_eq!(("abc", ""), split_once("abc", "123"));
        assert_eq!(("a", "b-c"), split_once("a-b-c", "-"));
        assert_eq!(("a", "-b"), split_once("a--b", "-"));
    }
}