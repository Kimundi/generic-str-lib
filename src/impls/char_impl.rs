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
        assert_eq!(s._match_indices('c').collect::<Vec<_>>(),
                    vec![(2u, 3u)]);
        assert_eq!(s._match_indices('c').rev().collect::<Vec<_>>(),
                    vec![(2u, 3u)]);
    }

    #[test]
    fn test2() {
        let s = "abcbdef";
        assert_eq!(s._match_indices('b').collect::<Vec<_>>(),
                    vec![(1u, 2u), (3, 4)]);
        assert_eq!(s._match_indices('b').rev().collect::<Vec<_>>(),
                    vec![(3, 4), (1u, 2u)]);
    }

    #[test]
    fn test3() {
        let s = "ศไทย中华Việt Nam; Mary had a little lamb, Little lamb";
        assert_eq!(s._match_indices('a').collect::<Vec<_>>(),
                    vec![(26, 27), (31, 32), (36, 37), (39, 40), (49u, 50u), (62, 63)]);
        assert_eq!(s._match_indices('a').rev().collect::<Vec<_>>(),
                    vec![(62, 63), (49u, 50u), (39, 40), (36, 37), (31, 32), (26, 27)]);

        assert_eq!(s._match_indices('中').collect::<Vec<_>>(),
                    vec![(12u, 15u)]);
        assert_eq!(s._match_indices('中').rev().collect::<Vec<_>>(),
                    vec![(12u, 15u)]);
    }
}
