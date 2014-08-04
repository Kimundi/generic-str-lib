use super::super::{Pattern, LeftMatcher, Matcher};
use super::super::{OffsetSlice, Utf8Char};

struct CharMatcher<'a> {
    cursor: OffsetSlice<'a>,
    chr: Utf8Char
}
impl<'a> Pattern<'a, CharMatcher<'a>> for char {
    fn into_matcher(self, s: &'a str) -> CharMatcher<'a> {
        CharMatcher {
            cursor: OffsetSlice::new(s),
            chr: Utf8Char::new(self)
        }
    }
    fn is_contained_in(self, s: &str) -> bool {
        self.into_matcher(s).next_match().is_some()
    }
}
impl<'a> LeftMatcher<'a> for CharMatcher<'a> {
    fn get_haystack(&self) -> &'a str {
        self.cursor.original_str()
    }

    fn next_match(&mut self) -> Option<(uint, &'a str)> {
        let CharMatcher { ref mut cursor, chr } = *self;
        cursor.find_front(chr.as_str())
    }
}
impl<'a> Matcher<'a> for CharMatcher<'a> {
    fn next_match_back(&mut self) -> Option<(uint, &'a str)> {
        let CharMatcher { ref mut cursor, chr } = *self;
        cursor.find_back(chr.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::StrSlice_;
    use std::prelude::{Vec, Iterator, DoubleEndedIterator};

    #[test]
    fn test1() {
        let s = "abcbdef";
        iter_eq!(s._match_indices('c'), [(2u, "c")]);
        iter_eq!(s._match_indices('c').rev(), [(2u, "c")]);
    }

    #[test]
    fn test2() {
        let s = "abcbdef";
        iter_eq!(s._match_indices('b'), [(1u, "b"), (3, "b")]);
        iter_eq!(s._match_indices('b').rev(), [(3, "b"), (1u, "b")]);
    }

    #[test]
    fn test3() {
        let s = "ศไทย中华Việt Nam; Mary had a little lamb, Little lamb";
        iter_eq!(s._match_indices('a'),
                 [(26, "a"), (31, "a"), (36, "a"), (39, "a"), (49u, "a"), (62, "a")]);
        iter_eq!(s._match_indices('a').rev(),
                 [(62, "a"), (49u, "a"), (39, "a"), (36, "a"), (31, "a"), (26, "a")]);

        iter_eq!(s._match_indices('中'), [(12u, "中")]);
        iter_eq!(s._match_indices('中').rev(), [(12u, "中")]);
    }
}
