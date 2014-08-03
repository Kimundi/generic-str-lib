use super::super::{Pattern, LeftMatcher, Matcher};
use std::str::CharOffsets;

struct CharClPredMatcher<'a, 'b> {
    str: &'a str,
    chars: CharOffsets<'a>,
    pred: |char|:'b -> bool
}
impl<'a, 'b> Pattern<'a, CharClPredMatcher<'a, 'b>> for |char|:'b -> bool {
    fn into_matcher(self, s: &'a str) -> CharClPredMatcher<'a, 'b> {
        CharClPredMatcher {
            str: s,
            chars: s.char_indices(),
            pred: self
        }
    }
    fn is_contained_in(self, s: &str) -> bool {
        self.into_matcher(s).next_match().is_some()
    }
}
impl<'a, 'b> LeftMatcher<'a> for CharClPredMatcher<'a, 'b> {
    fn get_haystack(&self) -> &'a str {
        self.str
    }

    fn next_match(&mut self) -> Option<(uint, uint)> {
        loop {
            match self.chars.next() {
                Some((i, c)) if (self.pred)(c) => {
                    return Some((i, i + c.len_utf8_bytes()))
                }
                Some(_) => continue,
                None => break,
            }
        }
        None
    }
}
impl<'a, 'b> Matcher<'a> for CharClPredMatcher<'a, 'b> {
    fn next_match_back(&mut self) -> Option<(uint, uint)> {
        loop {
            match self.chars.next_back() {
                Some((i, c)) if (self.pred)(c) => {
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
    use super::super::super::StrSlice_;
    use std::prelude::{Vec, Iterator, DoubleEndedIterator};

    #[test]
    fn test1() {
        let s = "abcbdef";
        let f = |c: char| c == 'c';
        assert_eq!(s._match_indices(f).collect::<Vec<_>>(),
                    vec![(2u, 3u)]);
    }

    #[test]
    fn test2() {
        let s = "abcbdef";
        let f = |c: char| c == 'b';
        assert_eq!(s._match_indices(f).collect::<Vec<_>>(),
                    vec![(1u, 2u), (3, 4)]);
    }

    #[test]
    fn test3() {
        let s = "ศไทย中华Việt Nam; Mary had a little lamb, Little lamb";
        let f = |c: char| c == 'm' || c == 'r' || c == 'd';
        assert_eq!(s._match_indices(f).collect::<Vec<_>>(),
                    vec![(27, 28), (32, 33), (37, 38), (50u, 51u), (63, 64)]);

        let f = |c: char| c == 'm' || c == 'r' || c == 'd';
        assert_eq!(s._matches(f).collect::<Vec<_>>(),
                    vec!["m", "r", "d", "m", "m"]);

        let f = |c: char| c == '中';
        assert_eq!(s._match_indices(f).collect::<Vec<_>>(),
                    vec![(12u, 15u)]);
    }

    #[test]
    fn test1_rev() {
        let s = "abcbdef";
        let f = |c: char| c == 'c';
        assert_eq!(s._match_indices(f).rev().collect::<Vec<_>>(),
                    vec![(2u, 3u)]);
    }

    #[test]
    fn test2_rev() {
        let s = "abcbdef";
        let f = |c: char| c == 'b';
        assert_eq!(s._match_indices(f).rev().collect::<Vec<_>>(),
                    vec![(3u, 4u), (1, 2)]);
    }

    #[test]
    fn test3_rev() {
        let s = "ศไทย中华Việt Nam; Mary had a little lamb, Little lamb";
        let f = |c: char| c == 'm' || c == 'r' || c == 'd';
        assert_eq!(s._match_indices(f).rev().collect::<Vec<_>>(),
                    vec![(63, 64), (50u, 51u), (37, 38), (32, 33), (27, 28)]);

        let f = |c: char| c == 'm' || c == 'r' || c == 'd';
        assert_eq!(s._matches(f).rev().collect::<Vec<_>>(),
                    vec!["m", "m", "d", "r", "m"]);

        let f = |c: char| c == '中';
        assert_eq!(s._match_indices(f).rev().collect::<Vec<_>>(),
                    vec![(12u, 15u)]);
    }
}
