use super::super::{Pattern, LeftMatcher, Matcher};
use std::str::CharOffsets;

struct CharSliceMatcher<'a, 'b> {
    str: &'a str,
    chars: CharOffsets<'a>,
    slice: &'b [char]
}
impl<'a, 'b> Pattern<'a, CharSliceMatcher<'a, 'b>> for &'b [char] {
    fn into_matcher(self, s: &'a str) -> CharSliceMatcher<'a, 'b> {
        CharSliceMatcher {
            str: s,
            chars: s.char_indices(),
            slice: self
        }
    }
    fn is_contained_in(self, s: &str) -> bool {
        self.into_matcher(s).next_match().is_some()
    }
}
impl<'a, 'b> LeftMatcher<'a> for CharSliceMatcher<'a, 'b> {
    fn get_haystack(&self) -> &'a str {
        self.str
    }

    fn next_match(&mut self) -> Option<(uint, &'a str)> {
        loop {
            match self.chars.next() {
                Some((i, c)) if self.slice.contains(&c) => {
                    let a = i;
                    let b = i + c.len_utf8_bytes();
                    return Some((a, self.str.slice(a, b)))                }
                Some(_) => continue,
                None => break,
            }
        }
        None
    }
}
impl<'a, 'b> Matcher<'a> for CharSliceMatcher<'a, 'b> {
    fn next_match_back(&mut self) -> Option<(uint, &'a str)> {
        loop {
            match self.chars.next_back() {
                Some((i, c)) if self.slice.contains(&c) => {
                    let a = i;
                    let b = i + c.len_utf8_bytes();
                    return Some((a, self.str.slice(a, b)))                }
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
    use std::prelude::{Vec, Iterator, DoubleEndedIterator, Str, Vector};

    #[test]
    fn test1() {
        let s = "abcbdef";
        iter_eq!(s._match_indices(['c'].as_slice()), [(2u, "c")]);
    }

    #[test]
    fn test2() {
        let s = "abcbdef";
        iter_eq!(s._match_indices(['b'].as_slice()), [(1u, "b"), (3, "b")]);
    }

    #[test]
    fn test3() {
        let s = "ศไทย中华Việt Nam; Mary had a little lamb, Little lamb";
        iter_eq!(s._match_indices(['m', 'r', 'd'].as_slice()),
                 [(27, "m"), (32, "r"), (37, "d"), (50u, "m"), (63, "m")]);

        iter_eq!(s._matches(['m', 'r', 'd'].as_slice()), ["m", "r", "d", "m", "m"]);
        iter_eq!(s._match_indices(['中'].as_slice()), [(12u, "中")]);
    }

    #[test]
    fn test1_rev() {
        let s = "abcbdef";
        iter_eq!(s._match_indices(['c'].as_slice()).rev(), [(2u, "c")]);
    }

    #[test]
    fn test2_rev() {
        let s = "abcbdef";
        iter_eq!(s._match_indices(['b'].as_slice()).rev(), [(3u, "b"), (1, "b")]);
    }

    #[test]
    fn test3_rev() {
        let s = "ศไทย中华Việt Nam; Mary had a little lamb, Little lamb";
        iter_eq!(s._match_indices(['m', 'r', 'd'].as_slice()).rev(),
                 [(63, "m"), (50u, "m"), (37, "d"), (32, "r"), (27, "m")]);

        iter_eq!(s._matches(['m', 'r', 'd'].as_slice()).rev(), ["m", "m", "d", "r", "m"]);
        iter_eq!(s._match_indices(['中'].as_slice()).rev(), [(12u, "中")]);
    }
}
