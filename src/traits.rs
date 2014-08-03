pub trait Pattern<'a, M> {
    fn into_matcher(self, &'a str) -> M;
    fn is_contained_in(self, &str) -> bool;
}

pub trait LeftMatcher<'a> {
    fn get_haystack(&self) -> &'a str;
    fn next_match(&mut self) -> Option<(uint, uint)>;

    fn next_match_str(&mut self) -> Option<&'a str> {
        let string = self.get_haystack();
        self.next_match().map(|(a, b)| string.slice(a, b))
    }
}

pub trait Matcher<'a>: LeftMatcher<'a> {
    fn next_match_back(&mut self) -> Option<(uint, uint)>;

    fn next_match_back_str(&mut self) -> Option<&'a str> {
        let string = self.get_haystack();
        self.next_match_back().map(|(a, b)| string.slice(a, b))
    }
}
