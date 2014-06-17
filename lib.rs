pub use traits::{Matcher, LeftMatcher, Pattern};
pub use matches::{Matches, MatchIndices};
pub use splits::{Splits, NSplits, RNSplits, TermSplits};

mod traits;

mod matches;
mod splits;

mod char_matcher_impl;
mod str_matcher_impl;
mod regex_matcher_impl;
mod char_closure_impl;
mod char_fn_impl;
mod char_slice_impl;

mod util;

#[cfg(test)]
mod tests;

pub trait StrExt<'a> {
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
