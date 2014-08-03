use super::super::{Pattern, LeftMatcher, Matcher};
use super::super::OffsetSlice;

struct StrMatcher<'a, 'b> {
    cursor: OffsetSlice<'a>,
    buf: &'b [u8]
}
impl<'a, 'b> Pattern<'a, StrMatcher<'a, 'b>> for &'b str {
    fn into_matcher(self, s: &'a str) -> StrMatcher<'a, 'b> {
        StrMatcher {
            cursor: OffsetSlice::new(s),
            buf: self.as_bytes()
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

    fn next_match(&mut self) -> Option<(uint, uint)> {
        let StrMatcher { ref mut cursor, buf } = *self;
        cursor.find_front(buf)
    }
}
impl<'a, 'b> Matcher<'a> for StrMatcher<'a, 'b> {
    fn next_match_back(&mut self) -> Option<(uint, uint)> {
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
        assert_eq!(s._match_indices("c").collect::<Vec<_>>(),
                    vec![(2u, 3u)]);
        assert_eq!(s._match_indices("c").rev().collect::<Vec<_>>(),
                    vec![(2u, 3u)]);
    }

    #[test]
    fn test2() {
        let s = "abcbdef";
        assert_eq!(s._match_indices("b").collect::<Vec<_>>(),
                    vec![(1u, 2u), (3, 4)]);
        assert_eq!(s._match_indices("b").rev().collect::<Vec<_>>(),
                    vec![(3, 4), (1u, 2u)]);
    }

    #[test]
    fn test3() {
        let s = "ศไทย中华Việt Nam; Mary had a little lamb, Little lamb";
        assert_eq!(s._match_indices("am").collect::<Vec<_>>(),
                    vec![(26, 28), (49u, 51u), (62, 64)]);
        assert_eq!(s._match_indices("am").rev().collect::<Vec<_>>(),
                    vec![(62, 64), (49u, 51u), (26, 28)]);

        assert_eq!(s._match_indices("中").collect::<Vec<_>>(),
                    vec![(12u, 15u)]);
        assert_eq!(s._match_indices("中").rev().collect::<Vec<_>>(),
                    vec![(12u, 15u)]);

    }
}
