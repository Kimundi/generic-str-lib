use super::{Pattern, LeftMatcher, Matcher};

pub struct Matches<M> {
    matcher: M
}

impl<M> Matches<M> {
    pub fn new<'a, P: Pattern<'a, M>>(s: &'a str, pat: P) -> Matches<M> {
        let string_matcher = pat.into_matcher(s);
        Matches { matcher: string_matcher }
    }
}

impl<'a, M: LeftMatcher<'a>> Iterator<&'a str> for Matches<M> {
    fn next(&mut self) -> Option<&'a str> {
        self.matcher.next_match().map(|(_, s)| s)
    }
}

impl<'a, M: Matcher<'a>> DoubleEndedIterator<&'a str> for Matches<M> {
    fn next_back(&mut self) -> Option<&'a str> {
        self.matcher.next_match_back().map(|(_, s)| s)
    }
}

pub struct MatchIndices<M> {
    matcher: M
}

impl<M> MatchIndices<M> {
    pub fn new<'a, P: Pattern<'a, M>>(s: &'a str, pat: P) -> MatchIndices<M> {
        let string_matcher = pat.into_matcher(s);
        MatchIndices { matcher: string_matcher }
    }
}

impl<'a, M: LeftMatcher<'a>> Iterator<(uint, &'a str)> for MatchIndices<M> {
    fn next(&mut self) -> Option<(uint, &'a str)> {
        self.matcher.next_match()
    }
}

impl<'a, M: Matcher<'a>> DoubleEndedIterator<(uint, &'a str)> for MatchIndices<M> {
    fn next_back(&mut self) -> Option<(uint, &'a str)> {
        self.matcher.next_match_back()
    }
}
