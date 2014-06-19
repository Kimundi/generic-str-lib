pub use self::traits::{Matcher, LeftMatcher, Pattern, Fragment};
pub use self::matches::{Matches, MatchIndices};
pub use self::splits::{Splits, NSplits, RNSplits, TermSplits};

// Matcher and Fragement defintions
mod traits;

// Matcher and Fragment implementations
mod char_impl;
mod str_impl;
mod regex_impl;
mod char_closure_impl;
mod char_fn_impl;
mod char_slice_impl;

#[cfg(test)]
mod tests;

// StrSlice support implementations
mod matches;
mod splits;

pub trait StrSlice_<'a> {
    fn _contains<'a, M, P: Pattern<'a, M>>(self, pat: P) -> bool;

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
        self._match_indices(pat).next().map(|(a, _)| a == 0).unwrap_or(false)
    }
    fn _ends_with<M: Matcher<'a>, P: Pattern<'a, M>>(self, pat: P) -> bool {
        self._match_indices(pat).rev().next().map(|(_, b)| b == self.len()).unwrap_or(false)
    }
    fn _trim_left_matches<M: LeftMatcher<'a>, P: Pattern<'a, M>>(self, pat: P) -> &'a str {
        let mut i = 0;
        for (a, b) in self._match_indices(pat) {
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
        for (a, b) in self._match_indices(pat).rev() {
            if b == i {
                i = a;
            } else {
                break;
            }
        }
        self.slice_to(i)
    }
    fn _trim_matches<M: Matcher<'a>, P: Pattern<'a, M>>(self, pat: P) -> &'a str {
        let mut match_indices = self._match_indices(pat);
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

pub trait StrAllocating_<'a> {
    fn _replace<'a, M: LeftMatcher<'a>, P: Pattern<'a, M>, F: Fragment>(self, pat: P, with: F) -> String;
}

impl<'a> StrAllocating_<'a> for &'a str {
    fn _replace<M: LeftMatcher<'a>, P: Pattern<'a, M>, F: Fragment>(self, pat: P, with: F) -> String {
        let mut buf = String::new();
        let mut first = true;
        for segment in self._split(pat) {
            if !first {
                with.write_fragment(|s|
                    buf.push_str(s)
                );
            }
            first = false;
            buf.push_str(segment);
        }
        buf
    }
}

pub trait StringExtension {
    fn from_fragment<F: Fragment>(f: F) -> Self;
    fn push<F: Fragment>(&mut self, f: F);
}

impl StringExtension for String {
    fn from_fragment<F: Fragment>(f: F) -> String {
        f.write_fragment(|s|
            String::from_str(s)
        )
    }

    fn push<F: Fragment>(&mut self, f: F) {
        f.write_fragment(|s|
            self.push_str(s)
        )
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
    pub fn find_front(&mut self, buf: &[u8]) -> Option<(uint, uint)> {
        while self.start < self.end {
            let start = self.start;
            self.start += 1;

            if self.slice.as_bytes().slice_from(start).starts_with(buf) {
                return Some((start, start + buf.len()));
            }
        }
        None
    }

    #[inline]
    pub fn find_back(&mut self, buf: &[u8]) -> Option<(uint, uint)> {
        while self.start < self.end {
            let end = self.end;
            self.end -= 1;

            if self.slice.as_bytes().slice_to(end).ends_with(buf) {
                return Some((end - buf.len(), end));
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
        let mut buf = [08, ..4];
        let len = chr.encode_utf8(buf.as_mut_slice());
        Utf8Char { chr: buf, len: len as u8 }
    }

    #[inline]
    pub fn as_str<'a>(&'a self) -> &'a str {
        unsafe {
            ::std::mem::transmute(::std::raw::Slice {
                data: &self.chr as *_ as *u8,
                len: self.len as uint
            })
        }
    }

    #[inline]
    pub fn as_bytes<'a>(&'a self) -> &'a [u8] {
        self.as_str().as_bytes()
    }

}
