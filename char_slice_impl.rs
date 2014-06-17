use super::{Pattern, LeftMatcher, Matcher};
use std::str::CharOffsets;

struct CharSliceLeftMatcher<'a, 'b> {
    str: &'a str,
    chars: CharOffsets<'a>,
    slice: &'b [char]
}
impl<'a, 'b> Pattern<'a, CharSliceLeftMatcher<'a, 'b>> for &'b [char] {
    fn into_matcher(self, s: &'a str) -> CharSliceLeftMatcher<'a, 'b> {
        CharSliceLeftMatcher {
            str: s,
            chars: s.char_indices(),
            slice: self
        }
    }
    fn is_contained_in(self, s: &str) -> bool {
        self.into_matcher(s).next_match().is_some()
    }
}
impl<'a, 'b> LeftMatcher<'a> for CharSliceLeftMatcher<'a, 'b> {
    fn get_haystack(&self) -> &'a str {
        self.str
    }

    fn next_match(&mut self) -> Option<(uint, uint)> {
        loop {
            match self.chars.next() {
                Some((i, c)) if self.slice.contains(&c) => {
                    return Some((i, i + c.len_utf8_bytes()))
                }
                Some(_) => continue,
                None => break,
            }
        }
        None
    }
}
impl<'a, 'b> Matcher<'a> for CharSliceLeftMatcher<'a, 'b> {
    fn next_match_back(&mut self) -> Option<(uint, uint)> {
        loop {
            match self.chars.next_back() {
                Some((i, c)) if self.slice.contains(&c) => {
                    return Some((i, i + c.len_utf8_bytes()))
                }
                Some(_) => continue,
                None => break,
            }
        }
        None
    }
}
#[cfg(test)]
mod tests {
    use super::super::StrExt;
    use std::prelude::{Vec, Iterator, DoubleEndedIterator, Str, Vector};

    #[test]
    fn test1() {
        let s = "abcbdef";
        assert_eq!(s._match_indices(['c'].as_slice()).collect::<Vec<_>>(),
                    vec![(2u, 3u)]);
    }

    #[test]
    fn test2() {
        let s = "abcbdef";
        assert_eq!(s._match_indices(['b'].as_slice()).collect::<Vec<_>>(),
                    vec![(1u, 2u), (3, 4)]);
    }

    #[test]
    fn test3() {
        let s = "ศไทย中华Việt Nam; Mary had a little lamb, Little lamb";
        assert_eq!(s._match_indices(['m', 'r', 'd'].as_slice()).collect::<Vec<_>>(),
                    vec![(27, 28), (32, 33), (37, 38), (50u, 51u), (63, 64)]);

        assert_eq!(s._matches(['m', 'r', 'd'].as_slice()).collect::<Vec<_>>(),
                    vec!["m", "r", "d", "m", "m"]);

        assert_eq!(s._match_indices(['中'].as_slice()).collect::<Vec<_>>(),
                    vec![(12u, 15u)]);
    }

    #[test]
    fn test1_rev() {
        let s = "abcbdef";
        assert_eq!(s._match_indices(['c'].as_slice()).rev().collect::<Vec<_>>(),
                    vec![(2u, 3u)]);
    }

    #[test]
    fn test2_rev() {
        let s = "abcbdef";
        assert_eq!(s._match_indices(['b'].as_slice()).rev().collect::<Vec<_>>(),
                    vec![(3u, 4u), (1, 2)]);
    }

    #[test]
    fn test3_rev() {
        let s = "ศไทย中华Việt Nam; Mary had a little lamb, Little lamb";
        assert_eq!(s._match_indices(['m', 'r', 'd'].as_slice()).rev().collect::<Vec<_>>(),
                    vec![(63, 64), (50u, 51u), (37, 38), (32, 33), (27, 28)]);

        assert_eq!(s._matches(['m', 'r', 'd'].as_slice()).rev().collect::<Vec<_>>(),
                    vec!["m", "m", "d", "r", "m"]);

        assert_eq!(s._match_indices(['中'].as_slice()).rev().collect::<Vec<_>>(),
                    vec![(12u, 15u)]);
    }
}
