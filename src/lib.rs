#![feature(macro_rules)]

pub use self::traits::{Matcher, LeftMatcher, Pattern};
pub use self::matches::{Matches, MatchIndices};
pub use self::splits::{Splits, NSplits, RNSplits, TermSplits};

// Helper macro for unit tests
macro_rules! iter_eq {
    ($i:expr, $s:expr) => {
        {
            let i: Vec<_> = $i.collect();
            let s = $s;
            assert_eq!(i.as_slice(), s.as_slice());
        }
    }
}

// Matcher definitions
mod traits;

// Matcher implementations
mod impls;

#[cfg(test)]
mod tests;

// StrSlice support implementations
mod matches;
mod splits;

pub trait StrSlice_<'a> {
    fn _contains<M, P: Pattern<'a, M>>(self, pat: P) -> bool;

    fn _matches<M, P: Pattern<'a, M>>(self, pat: P) -> Matches<M>;
    fn _match_indices<M, P: Pattern<'a, M>>(self, pat: P) -> MatchIndices<M>;

    fn _split<M, P: Pattern<'a, M>>(self, pat: P) -> Splits<M>;
    fn _split_terminator<M, P: Pattern<'a, M>>(self, pat: P) -> TermSplits<M>;
    fn _splitn<M, P: Pattern<'a, M>>(self, pat: P, n: uint) -> NSplits<M>;
    fn _rsplitn<M, P: Pattern<'a, M>>(self, pat: P, n: uint) -> RNSplits<M>;

    fn _starts_with<M: LeftMatcher<'a>, P: Pattern<'a, M>>(self, pat: P) -> bool;
    fn _ends_with<M: Matcher<'a>, P: Pattern<'a, M>>(self, pat: P) -> bool;

    fn _trim_matches<M: Matcher<'a>, P: Pattern<'a, M>>(self, pat: P) -> &'a str;
    fn _trim_left_matches<M: LeftMatcher<'a>, P: Pattern<'a, M>>(self, pat: P) -> &'a str;
    fn _trim_right_matches<M: Matcher<'a>, P: Pattern<'a, M>>(self, pat: P) -> &'a str;

    fn _find<M: LeftMatcher<'a>, P: Pattern<'a, M>>(self, pat: P) -> Option<uint> {
        self._match_indices(pat).next().map(|(a, _)| a)
    }
    fn _rfind<M: Matcher<'a>, P: Pattern<'a, M>>(self, pat: P) -> Option<uint> {
        self._match_indices(pat).rev().next().map(|(a, _)| a)
    }
}

impl<'a> StrSlice_<'a> for &'a str {
    fn _contains<M, P: Pattern<'a, M>>(self, pat: P) -> bool {
        pat.is_contained_in(self)
    }

    fn _matches<M, P: Pattern<'a, M>>(self, pat: P) -> Matches<M> {
        Matches::new(self, pat)
    }
    fn _match_indices<M, P: Pattern<'a, M>>(self, pat: P) -> MatchIndices<M> {
        MatchIndices::new(self, pat)
    }
    fn _split<M, P: Pattern<'a, M>>(self, pat: P) -> Splits<M> {
        Splits::new(self, pat)
    }
    fn _split_terminator<M, P: Pattern<'a, M>>(self, pat: P) -> TermSplits<M> {
        TermSplits::new(self, pat)
    }
    fn _splitn<M, P: Pattern<'a, M>>(self, pat: P, n: uint) -> NSplits<M> {
        NSplits::new(self, pat, n)
    }
    fn _rsplitn<M, P: Pattern<'a, M>>(self, pat: P, n: uint) -> RNSplits<M> {
        RNSplits::new(self, pat, n)
    }
    fn _starts_with<M: LeftMatcher<'a>, P: Pattern<'a, M>>(self, pat: P) -> bool {
        self._match_indices(pat).next()
            .map(|(a, _)| a == 0).unwrap_or(false)
    }
    fn _ends_with<M: Matcher<'a>, P: Pattern<'a, M>>(self, pat: P) -> bool {
        self._match_indices(pat).rev().next()
            .map(|(a, s)| a + s.len() == self.len()).unwrap_or(false)
    }
    fn _trim_left_matches<M: LeftMatcher<'a>, P: Pattern<'a, M>>(self, pat: P) -> &'a str {
        let mut i = 0;
        for (a, b) in self._match_indices(pat).map(|(a, s)| (a, a + s.len())) {
            if a == i {
                i = b;
            } else {
                break;
            }
        }
        self.slice_from(i)
    }
    fn _trim_right_matches<M: Matcher<'a>, P: Pattern<'a, M>>(self, pat: P) -> &'a str {
        let mut i = self.len();
        for (a, b) in self._match_indices(pat).rev().map(|(a, s)| (a, a + s.len())) {
            if b == i {
                i = a;
            } else {
                break;
            }
        }
        self.slice_to(i)
    }
    fn _trim_matches<M: Matcher<'a>, P: Pattern<'a, M>>(self, pat: P) -> &'a str {
        let mut match_indices = self._match_indices(pat).map(|(a, s)| (a, a + s.len()));
        let mut i = 0;
        let mut possible_end_match = None;
        for (a, b) in match_indices {
            if a == i {
                i = b;
            } else {
                possible_end_match = Some((a, b));
                break;
            }
        }
        let mut j = self.len();
        for (a, b) in match_indices.rev().chain(possible_end_match.move_iter()) {
            if b == j {
                j = a;
            } else {
                break;
            }
        }
        self.slice(i, j)
    }
}

pub struct OffsetSlice<'a> {
    slice: &'a str,
    start: uint,
    end: uint,
}

impl<'a> OffsetSlice<'a> {
    #[inline]
    pub fn new(s: &'a str) -> OffsetSlice<'a> {
        OffsetSlice {
            slice: s,
            start: 0,
            end: s.len()
        }
    }

    #[inline]
    pub fn find_front(&mut self, buf: &str) -> Option<(uint, &'a str)> {
        use std::str::raw;

        while self.start + buf.len() <= self.end {
            let start = self.start;
            self.start += 1;

            unsafe {
                // This can slice between utf8 boundaries, but `eq` on
                // strings just compares bytes, so it should be fine.
                let buf_eq = raw::slice_unchecked(self.slice, start, start + buf.len());
                if buf_eq == buf {
                    return Some((start, buf_eq));
                }
            }
        }
        None
    }

    #[inline]
    pub fn find_back(&mut self, buf: &str) -> Option<(uint, &'a str)> {
        use std::str::raw;

        while self.start + buf.len() <= self.end {
            let end = self.end;
            self.end -= 1;

            unsafe {
                // This can slice between utf8 boundaries, but `eq` on
                // strings just compares bytes, so it should be fine.
                let buf_eq = raw::slice_unchecked(self.slice, end - buf.len(), end);
                if buf_eq == buf {
                    return Some((end - buf.len(), buf_eq));
                }
            }
        }
        None
    }

    #[inline]
    pub fn original_str(self) -> &'a str {
        self.slice
    }
}

pub struct Utf8Char {
    chr: [u8, ..4],
    len: u8
}

impl Utf8Char {
    #[inline]
    pub fn new(chr: char) -> Utf8Char {
        let mut buf = [0, ..4];
        Utf8Char { len: chr.encode_utf8(buf.as_mut_slice()) as u8, chr: buf }
    }

    #[inline]
    pub fn as_str<'a>(&'a self) -> &'a str {
        use std::str::raw;
        unsafe {
            let s = raw::from_utf8(self.chr);
            raw::slice_unchecked(s, 0, self.len as uint)
        }
    }
}
