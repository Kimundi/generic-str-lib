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

    fn next_match(&mut self) -> Option<(uint, &'a str)> {
        loop {
            match self.chars.next() {
                Some((i, c)) if (self.pred)(c) => {
                    let a = i;
                    let b = i + c.len_utf8_bytes();
                    return Some((a, self.str.slice(a, b)))
                }
                Some(_) => continue,
                None => break,
            }
        }
        None
    }
}
impl<'a, 'b> Matcher<'a> for CharClPredMatcher<'a, 'b> {
    fn next_match_back(&mut self) -> Option<(uint, &'a str)> {
        loop {
            match self.chars.next_back() {
                Some((i, c)) if (self.pred)(c) => {
                    let a = i;
                    let b = i + c.len_utf8_bytes();
                    return Some((a, self.str.slice(a, b)))
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
        iter_eq!(s._match_indices(f), [(2u, "c")]);
    }

    #[test]
    fn test2() {
        let s = "abcbdef";
        let f = |c: char| c == 'b';
        iter_eq!(s._match_indices(f), [(1u, "b"), (3, "b")]);
    }

    #[test]
    fn test3() {
        let s = "ศไทย中华Việt Nam; Mary had a little lamb, Little lamb";
        let f = |c: char| c == 'm' || c == 'r' || c == 'd';
        iter_eq!(s._match_indices(f),
                 [(27, "m"), (32, "r"), (37, "d"), (50u, "m"), (63, "m")]);

        let f = |c: char| c == 'm' || c == 'r' || c == 'd';
        iter_eq!(s._matches(f), ["m", "r", "d", "m", "m"]);

        let f = |c: char| c == '中';
        iter_eq!(s._match_indices(f), [(12u, "中")]);
    }

    #[test]
    fn test1_rev() {
        let s = "abcbdef";
        let f = |c: char| c == 'c';
        iter_eq!(s._match_indices(f).rev(), [(2u, "c")]);
    }

    #[test]
    fn test2_rev() {
        let s = "abcbdef";
        let f = |c: char| c == 'b';
        iter_eq!(s._match_indices(f).rev(), [(3u, "b"), (1, "b")]);
    }

    #[test]
    fn test3_rev() {
        let s = "ศไทย中华Việt Nam; Mary had a little lamb, Little lamb";
        let f = |c: char| c == 'm' || c == 'r' || c == 'd';
        iter_eq!(s._match_indices(f).rev(),
                 [(63, "m"), (50u, "m"), (37, "d"), (32, "r"), (27, "m")]);

        let f = |c: char| c == 'm' || c == 'r' || c == 'd';
        iter_eq!(s._matches(f).rev(), ["m", "m", "d", "r", "m"]);

        let f = |c: char| c == '中';
        iter_eq!(s._match_indices(f).rev(), [(12u, "中")]);
    }
}
