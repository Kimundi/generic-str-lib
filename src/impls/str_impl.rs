use super::super::{Pattern, LeftMatcher, Matcher};
use super::super::OffsetSlice;

struct StrMatcher<'a, 'b> {
    cursor: OffsetSlice<'a>,
    buf: &'b str
}
impl<'a, 'b> Pattern<'a, StrMatcher<'a, 'b>> for &'b str {
    fn into_matcher(self, s: &'a str) -> StrMatcher<'a, 'b> {
        StrMatcher {
            cursor: OffsetSlice::new(s),
            buf: self
        }
    }
    fn is_contained_in(self, s: &str) -> bool {
        self.into_matcher(s).next_match().is_some()
    }
}
impl<'a, 'b> LeftMatcher<'a> for StrMatcher<'a, 'b> {
    fn get_haystack(&self) -> &'a str {
        self.cursor.original_str()
    }

    fn next_match(&mut self) -> Option<(uint, &'a str)> {
        let StrMatcher { ref mut cursor, buf } = *self;
        cursor.find_front(buf)
    }
}
impl<'a, 'b> Matcher<'a> for StrMatcher<'a, 'b> {
    fn next_match_back(&mut self) -> Option<(uint, &'a str)> {
        let StrMatcher { ref mut cursor, buf } = *self;
        cursor.find_back(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::StrSlice_;
    use std::prelude::{Vec, Iterator, DoubleEndedIterator};

    #[test]
    fn test1() {
        let s = "abcbdef";
        iter_eq!(s._match_indices("c"), [(2u, "c")]);
        iter_eq!(s._match_indices("c").rev(), [(2u, "c")]);
    }

    #[test]
    fn test2() {
        let s = "abcbdef";
        iter_eq!(s._match_indices("b"), [(1u, "b"), (3, "b")]);
        iter_eq!(s._match_indices("b").rev(), [(3, "b"), (1u, "b")]);
    }

    #[test]
    fn test3() {
        let s = "ศไทย中华Việt Nam; Mary had a little lamb, Little lamb";
        iter_eq!(s._match_indices("am"), [(26, "am"), (49u, "am"), (62, "am")]);
        iter_eq!(s._match_indices("am").rev(), [(62, "am"), (49u, "am"), (26, "am")]);

        iter_eq!(s._match_indices("中"), [(12u, "中")]);
        iter_eq!(s._match_indices("中").rev(), [(12u, "中")]);
    }
}
