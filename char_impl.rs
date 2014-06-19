use super::{Pattern, LeftMatcher, Matcher, Fragment};
use super::util::OffsetSlice;

struct Utf8Char {
    chr: [u8, ..4],
    len: u8
}

impl Utf8Char {
    #[inline]
    fn new(chr: char) -> Utf8Char {
        let mut buf = [08, ..4];
        let len = chr.encode_utf8(buf.as_mut_slice());
        Utf8Char { chr: buf, len: len as u8 }
    }

    #[inline]
    fn as_str<'a>(&'a self) -> &'a str {
        unsafe {
            ::std::mem::transmute(::std::raw::Slice {
                data: &self.chr as *_ as *u8,
                len: self.len as uint
            })
        }
    }

    #[inline]
    fn as_bytes<'a>(&'a self) -> &'a [u8] {
        self.as_str().as_bytes()
    }

}

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

    fn next_match(&mut self) -> Option<(uint, uint)> {
        let CharMatcher { ref mut cursor, chr } = *self;
        cursor.find_front(chr.as_bytes())
    }
}
impl<'a> Matcher<'a> for CharMatcher<'a> {
    fn next_match_back(&mut self) -> Option<(uint, uint)> {
        let CharMatcher { ref mut cursor, chr } = *self;
        cursor.find_back(chr.as_bytes())
    }
}

impl<'a> Fragment<'a, CharMatcher<'a>> for char {
    fn write_fragment(self, f: |&str|) {
        f(Utf8Char::new(self).as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::super::StrExt;
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
