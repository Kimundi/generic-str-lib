use std::prelude::{Some, None, Option, String, Iterator, DoubleEndedIterator, Str, Collection, ImmutableVector, ImmutableEqVector};

pub use traits::{Matcher, LeftMatcher, Pattern};
pub use matches::{Matches, MatchIndices};
pub use splits::{Splits, NSplits, RNSplits, TermSplits};

/////////////////////////
// Redirections to current `std::Str::StrSlice`

fn char_indices<'a>(s: &'a str) -> std::str::CharOffsets<'a> {
    use std::prelude::{StrSlice}; s.char_indices()
}

fn as_bytes<'a>(s: &'a str) -> &'a [u8] {
    use std::prelude::{StrSlice}; s.as_bytes()
}

fn slice<'a>(s: &'a str, a: uint, b: uint) -> &'a str {
    use std::prelude::{StrSlice}; s.slice(a, b)
}

fn slice_to<'a>(s: &'a str, a: uint) -> &'a str {
    use std::prelude::{StrSlice}; s.slice_to(a)
}

fn slice_from<'a>(s: &'a str, a: uint) -> &'a str {
    use std::prelude::{StrSlice}; s.slice_from(a)
}

/////////////////////////

mod traits;
mod matches;

mod splits;


/////////////////////////

#[test]
fn test_starts_with() {
    assert!("foobar"._starts_with("foo"));
    assert!("foobar"._starts_with('f'));

    assert!(!"foobar"._starts_with("oba"));
    assert!(!"foobar"._starts_with('o'));
}

#[test]
fn test_end_with() {
    assert!("foobar"._ends_with("bar"));
    assert!("foobar"._ends_with('r'));

    assert!(!"foobar"._ends_with("oob"));
    assert!(!"foobar"._ends_with('b'));
}

/////////////////////////

#[test]
fn test_trim_left() {
    assert_eq!("  ajklasd  "._trim_left_matches(" "), "ajklasd  ");
    assert_eq!("  ajklasd  "._trim_left_matches(' '), "ajklasd  ");
}

#[test]
fn test_trim_right() {
    assert_eq!("  ajklasd  "._trim_right_matches(" "), "  ajklasd");
    assert_eq!("  ajklasd  "._trim_right_matches(' '), "  ajklasd");
}

#[test]
fn test_trim() {
    assert_eq!("  ajklasd  "._trim_matches(" "), "ajklasd");
    assert_eq!("  ajklasd  "._trim_matches(' '), "ajklasd");
}

/////////////////////////
#[test]
fn test_find() {
    assert_eq!("abaaaba"._find("b"), Some(1));
    assert_eq!("abaaaba"._find("a"), Some(0));
    assert_eq!("abaaaba"._find("c"), None);
}

#[test]
fn test_rfind() {
    assert_eq!("abaaaba"._rfind("b"), Some(5));
    assert_eq!("abaaaba"._rfind("a"), Some(6));
    assert_eq!("abaaaba"._rfind("c"), None);
}

/////////////////////////

#[test]
fn test_replace() {
    assert_eq!("a"._replace("?", "b").as_slice(), "a");
    assert_eq!("a"._replace("a", "b").as_slice(), "b");
    assert_eq!("a"._replace('?', "b").as_slice(), "a");
    assert_eq!("a"._replace('a', "b").as_slice(), "b");
}

/////////////////////////

trait StrExt<'a> {
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

    fn _replace<'a, M: LeftMatcher<'a>, P: Pattern<'a, M>>(self, pat: P, to: &str) -> String;
}

impl<'a> StrExt<'a> for &'a str {
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
        slice_from(self, i)
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
        slice_to(self, i)
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
        slice(self, i, j)
    }

    fn _replace<M: LeftMatcher<'a>, P: Pattern<'a, M>>(self, pat: P, to: &str) -> String {
        let mut buf = String::new();
        let mut first = true;
        for segment in self._split(pat) {
            if !first {
                buf.push_str(to);
            }
            first = false;
            buf.push_str(segment);
        }
        buf
    }
}

/////////////////////////

struct OffsetSlice<'a> {
    slice: &'a str,
    start: uint,
    end: uint,
}

impl<'a> OffsetSlice<'a> {
    #[inline]
    fn new(s: &'a str) -> OffsetSlice<'a> {
        OffsetSlice {
            slice: s,
            start: 0,
            end: s.len()
        }
    }

    #[inline]
    fn find_front(&mut self, buf: &[u8]) -> Option<(uint, uint)> {
        while self.start < self.end {
            let start = self.start;
            self.start += 1;

            if as_bytes(self.slice).slice_from(start).starts_with(buf) {
                return Some((start, start + buf.len()));
            }
        }
        None
    }

    #[inline]
    fn find_back(&mut self, buf: &[u8]) -> Option<(uint, uint)> {
        while self.start < self.end {
            let end = self.end;
            self.end -= 1;

            if as_bytes(self.slice).slice_to(end).ends_with(buf) {
                return Some((end - buf.len(), end));
            }
        }
        None
    }
}

/////////////////////////

mod char_matcher {
    use super::{Pattern, LeftMatcher, Matcher, OffsetSlice};
    use std::prelude::{Option, MutableVector, Char};

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
        fn as_slice<'a>(&'a self) -> &'a [u8] {
            unsafe {
                ::std::mem::transmute(::std::raw::Slice {
                    data: &self.chr as *_ as *u8,
                    len: self.len as uint
                })
            }
        }
    }

    struct CharLeftMatcher<'a> {
        cursor: OffsetSlice<'a>,
        chr: Utf8Char
    }
    impl<'a> Pattern<'a, CharLeftMatcher<'a>> for char {
        fn into_matcher(self, s: &'a str) -> CharLeftMatcher<'a> {
            CharLeftMatcher {
                cursor: OffsetSlice::new(s),
                chr: Utf8Char::new(self)
            }
        }
        fn is_contained_in(self, s: &str) -> bool {
            self.into_matcher(s).next_match().is_some()
        }
    }
    impl<'a> LeftMatcher<'a> for CharLeftMatcher<'a> {
        fn get_haystack(&self) -> &'a str {
            self.cursor.slice
        }

        fn next_match(&mut self) -> Option<(uint, uint)> {
            let CharLeftMatcher { ref mut cursor, chr } = *self;
            cursor.find_front(chr.as_slice())
        }
    }
    impl<'a> Matcher<'a> for CharLeftMatcher<'a> {
        fn next_match_back(&mut self) -> Option<(uint, uint)> {
            let CharLeftMatcher { ref mut cursor, chr } = *self;
            cursor.find_back(chr.as_slice())
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
}
mod str_matcher {
    use super::{Pattern, LeftMatcher, Matcher, OffsetSlice};
    use std::prelude::{Option};

    struct StrLeftMatcher<'a, 'b> {
        cursor: OffsetSlice<'a>,
        buf: &'b [u8]
    }
    impl<'a, 'b> Pattern<'a, StrLeftMatcher<'a, 'b>> for &'b str {
        fn into_matcher(self, s: &'a str) -> StrLeftMatcher<'a, 'b> {
            StrLeftMatcher {
                cursor: OffsetSlice::new(s),
                buf: super::as_bytes(self)
            }
        }
        fn is_contained_in(self, s: &str) -> bool {
            self.into_matcher(s).next_match().is_some()
        }
    }
    impl<'a, 'b> LeftMatcher<'a> for StrLeftMatcher<'a, 'b> {
        fn get_haystack(&self) -> &'a str {
            self.cursor.slice
        }

        fn next_match(&mut self) -> Option<(uint, uint)> {
            let StrLeftMatcher { ref mut cursor, buf } = *self;
            cursor.find_front(buf)
        }
    }
    impl<'a, 'b> Matcher<'a> for StrLeftMatcher<'a, 'b> {
        fn next_match_back(&mut self) -> Option<(uint, uint)> {
            let StrLeftMatcher { ref mut cursor, buf } = *self;
            cursor.find_back(buf)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::super::StrExt;
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
}
mod regex_matcher {
    extern crate regex;
    use super::{Pattern, LeftMatcher};
    use self::regex::{Regex, FindMatches};
    use std::prelude::{Option, Iterator};

    struct RegexLeftMatcher<'a, 'b> {
        str: &'a str,
        regex: FindMatches<'b, 'a>
    }
    impl<'a, 'b> Pattern<'a, RegexLeftMatcher<'a, 'b>> for &'b Regex {
        fn into_matcher(self, s: &'a str) -> RegexLeftMatcher<'a, 'b> {
            RegexLeftMatcher {
                str: s,
                regex: self.find_iter(s)
            }
        }
        fn is_contained_in(self, s: &str) -> bool {
            self.is_match(s)
        }
    }
    impl<'a, 'b> LeftMatcher<'a> for RegexLeftMatcher<'a, 'b> {
        fn get_haystack(&self) -> &'a str {
            self.str
        }

        fn next_match(&mut self) -> Option<(uint, uint)> {
            self.regex.next()
        }
    }
    #[cfg(test)]
    mod tests {
        use super::super::StrExt;
        use super::regex::Regex;
        use std::prelude::{Vec, Iterator};


        #[test]
        fn test1() {
            let s = "abcbdef";
            let r = Regex::new("c").unwrap();
            assert_eq!(s._match_indices(&r).collect::<Vec<_>>(),
                       vec![(2u, 3u)]);
        }

        #[test]
        fn test2() {
            let s = "abcbdef";
            let r = Regex::new("b").unwrap();
            assert_eq!(s._match_indices(&r).collect::<Vec<_>>(),
                       vec![(1u, 2u), (3, 4)]);
        }

        #[test]
        fn test3() {
            let s = "ศไทย中华Việt Nam; Mary had a little lamb, Little lamb";
            let r = Regex::new("a[mrd]").unwrap();
            assert_eq!(s._match_indices(&r).collect::<Vec<_>>(),
                       vec![(26, 28), (31, 33), (36, 38), (49u, 51u), (62, 64)]);
            assert_eq!(s._matches(&r).collect::<Vec<_>>(),
                       vec!["am", "ar", "ad", "am", "am"]);

            let r = Regex::new("中").unwrap();
            assert_eq!(s._match_indices(&r).collect::<Vec<_>>(),
                       vec![(12u, 15u)]);

        }

        #[test]
        fn splitn() {
            let re = Regex::new(r"\d+").unwrap();
            let text = "cauchy123plato456tyler789binx";
            let subs: Vec<&str> = text._splitn(&re, 2).collect();
            assert_eq!(subs, vec!("cauchy", "plato", "tyler789binx"));
        }

        #[test]
        fn split() {
            let re = Regex::new(r"\d+").unwrap();
            let text = "cauchy123plato456tyler789binx";
            let subs: Vec<&str> = text._split(&re).collect();
            assert_eq!(subs, vec!("cauchy", "plato", "tyler", "binx"));
        }

        #[test]
        fn test_starts_with() {
            assert!("foobar"._starts_with(&{Regex::new("fo+").unwrap()}));
            assert!(!"foobar"._starts_with(&{Regex::new("[ob]").unwrap()}));
        }
    }
}
mod char_closure_predicate_matcher {
    use super::{Pattern, LeftMatcher, Matcher};
    use std::str::CharOffsets;
    use std::prelude::{Some, None, Option, DoubleEndedIterator, Iterator, Char};

    struct CharClPredLeftMatcher<'a, 'b> {
        str: &'a str,
        chars: CharOffsets<'a>,
        pred: |char|:'b -> bool
    }
    impl<'a, 'b> Pattern<'a, CharClPredLeftMatcher<'a, 'b>> for |char|:'b -> bool {
        fn into_matcher(self, s: &'a str) -> CharClPredLeftMatcher<'a, 'b> {
            CharClPredLeftMatcher {
                str: s,
                chars: super::char_indices(s),
                pred: self
            }
        }
        fn is_contained_in(self, s: &str) -> bool {
            self.into_matcher(s).next_match().is_some()
        }
    }
    impl<'a, 'b> LeftMatcher<'a> for CharClPredLeftMatcher<'a, 'b> {
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
    impl<'a, 'b> Matcher<'a> for CharClPredLeftMatcher<'a, 'b> {
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
        use super::super::StrExt;
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
}
mod char_function_predicate_matcher {
    use super::{Pattern, LeftMatcher, Matcher};
    use std::str::CharOffsets;
    use std::prelude::{Some, None, Option, DoubleEndedIterator, Char, Iterator};

    struct CharFnPredLeftMatcher<'a> {
        str: &'a str,
        chars: CharOffsets<'a>,
        pred: fn(char) -> bool
    }
    impl<'a> Pattern<'a, CharFnPredLeftMatcher<'a>> for fn(char) -> bool {
        fn into_matcher(self, s: &'a str) -> CharFnPredLeftMatcher<'a> {
            CharFnPredLeftMatcher {
                str: s,
                chars: super::char_indices(s),
                pred: self
            }
        }
        fn is_contained_in(self, s: &str) -> bool {
            self.into_matcher(s).next_match().is_some()
        }
    }
    impl<'a> LeftMatcher<'a> for CharFnPredLeftMatcher<'a> {
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
    impl<'a> Matcher<'a> for CharFnPredLeftMatcher<'a> {
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
        use super::super::StrExt;
        use std::prelude::{Vec, Iterator, DoubleEndedIterator};

        #[test]
        fn test1() {
            let s = "abcbdef";
            fn f(c: char) -> bool { c == 'c' }
            assert_eq!(s._match_indices(f).collect::<Vec<_>>(),
                       vec![(2u, 3u)]);
        }

        #[test]
        fn test2() {
            let s = "abcbdef";
            fn f(c: char) -> bool { c == 'b' }
            assert_eq!(s._match_indices(f).collect::<Vec<_>>(),
                       vec![(1u, 2u), (3, 4)]);
        }

        #[test]
        fn test3() {
            let s = "ศไทย中华Việt Nam; Mary had a little lamb, Little lamb";
            fn f(c: char) -> bool { c == 'm' || c == 'r' || c == 'd' }
            assert_eq!(s._match_indices(f).collect::<Vec<_>>(),
                       vec![(27, 28), (32, 33), (37, 38), (50u, 51u), (63, 64)]);

            assert_eq!(s._matches(f).collect::<Vec<_>>(),
                       vec!["m", "r", "d", "m", "m"]);

            fn g(c: char) -> bool { c == '中' }
            assert_eq!(s._match_indices(g).collect::<Vec<_>>(),
                       vec![(12u, 15u)]);
        }

        #[test]
        fn test1_rev() {
            let s = "abcbdef";
            fn f(c: char) -> bool { c == 'c' }
            assert_eq!(s._match_indices(f).rev().collect::<Vec<_>>(),
                       vec![(2u, 3u)]);
        }

        #[test]
        fn test2_rev() {
            let s = "abcbdef";
            fn f(c: char) -> bool { c == 'b' }
            assert_eq!(s._match_indices(f).rev().collect::<Vec<_>>(),
                       vec![(3u, 4u), (1, 2)]);
        }

        #[test]
        fn test3_rev() {
            let s = "ศไทย中华Việt Nam; Mary had a little lamb, Little lamb";
            fn f(c: char) -> bool { c == 'm' || c == 'r' || c == 'd' }
            assert_eq!(s._match_indices(f).rev().collect::<Vec<_>>(),
                       vec![(63, 64), (50u, 51u), (37, 38), (32, 33), (27, 28)]);

            assert_eq!(s._matches(f).rev().collect::<Vec<_>>(),
                       vec!["m", "m", "d", "r", "m"]);

            fn g(c: char) -> bool { c == '中' }
            assert_eq!(s._match_indices(g).rev().collect::<Vec<_>>(),
                       vec![(12u, 15u)]);
        }
    }
}
mod char_slice_matcher {
    use super::{Pattern, LeftMatcher, Matcher};
    use std::str::CharOffsets;
    use std::prelude::{Some, None, Option, DoubleEndedIterator, Char, ImmutableEqVector};
    use std::prelude::{Iterator};

    struct CharSliceLeftMatcher<'a, 'b> {
        str: &'a str,
        chars: CharOffsets<'a>,
        slice: &'b [char]
    }
    impl<'a, 'b> Pattern<'a, CharSliceLeftMatcher<'a, 'b>> for &'b [char] {
        fn into_matcher(self, s: &'a str) -> CharSliceLeftMatcher<'a, 'b> {
            CharSliceLeftMatcher {
                str: s,
                chars: super::char_indices(s),
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
}
