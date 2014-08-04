pub trait Pattern<'a, M> {
    fn into_matcher(self, &'a str) -> M;
    fn is_contained_in(self, &str) -> bool;
}

pub trait LeftMatcher<'a> {
    fn get_haystack(&self) -> &'a str;
    fn next_match(&mut self) -> Option<(uint, &'a str)>;
}

pub trait Matcher<'a>: LeftMatcher<'a> {
    fn next_match_back(&mut self) -> Option<(uint, &'a str)>;
}
