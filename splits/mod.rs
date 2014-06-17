use super::{Pattern, LeftMatcher, Matcher, StrExt};

pub use self::nsplits::NSplits;
pub use self::rnsplits::RNSplits;
pub use self::term_splits::TermSplits;

mod term_splits;
mod nsplits;
mod rnsplits;

pub struct Splits<M> {
    matcher: M,
    finished: bool,
    prev_start: uint,
    prev_end: uint,
}

impl<M> Splits<M> {
    pub fn new<'a, P: Pattern<'a, M>>(s: &'a str, pat: P) -> Splits<M> {
        Splits {
            matcher: pat.into_matcher(s),
            finished: false,
            prev_start: 0,
            prev_end: s.len()
        }
    }
}

fn splits_next<'a, M: LeftMatcher<'a>>(self_: &mut Splits<M>) -> Option<(uint, uint)> {
    if self_.finished {
        return None;
    }
    // In case of overlapping matches, consider them one big seperator
    loop {
        match self_.matcher.next_match() {
            Some((a, b)) => {
                let current_prev_start = self_.prev_start;
                self_.prev_start = b;
                if current_prev_start <= a {
                    return Some((current_prev_start, a));
                }
            }
            None => {
                self_.finished = true;
                return Some((self_.prev_start, self_.matcher.get_haystack().len()));
            }
        }
    }
}

fn splits_next_back<'a, M: Matcher<'a>>(self_: &mut Splits<M>) -> Option<(uint, uint)> {
        if self_.finished {
            return None;
        }
        // In case of overlapping matches, consider them one big seperator
        loop {
            match self_.matcher.next_match_back() {
                Some((a, b)) => {
                    let current_prev_end = self_.prev_end;
                    self_.prev_end = a;
                    if b <= current_prev_end {
                        return Some((b, current_prev_end));
                    }
                }
                None => {
                    self_.finished = true;
                    return Some((0, self_.prev_end));
                }
            }
        }
}

impl<'a, M: LeftMatcher<'a>> Iterator<&'a str> for Splits<M> {
    fn next(&mut self) -> Option<&'a str> {
        let string = self.matcher.get_haystack();
        splits_next(self).map(|(a, b)| {
            string.slice(a, b)
        })
    }
}

impl<'a, M: Matcher<'a>> DoubleEndedIterator<&'a str> for Splits<M> {
    fn next_back(&mut self) -> Option<&'a str> {
        let string = self.matcher.get_haystack();
        splits_next_back(self).map(|(a, b)| {
            string.slice(a, b)
        })
    }
}

#[test]
fn test_split() {
    let s = "asädfadfsdfa";
    let v: Vec<&str> = s._split("a").collect();
    assert_eq!(v, vec!["", "sädf", "dfsdf", ""]);
    let v: Vec<&str> = s._split("a").rev().collect();
    assert_eq!(v, vec!["", "dfsdf", "sädf", ""]);

    let s = "asädfaaaaadfsdfa";
    let v: Vec<&str> = s._split("a").collect();
    assert_eq!(v, vec!["", "sädf", "", "", "", "", "dfsdf", ""]);
    let v: Vec<&str> = s._split("a").rev().collect();
    assert_eq!(v, vec!["", "dfsdf", "", "", "", "", "sädf", ""]);

    let s = "fffababafff";
    let v: Vec<&str> = s._split("ab").collect();
    assert_eq!(v, vec!["fff", "", "afff"]);
    let v: Vec<&str> = s._split("ab").rev().collect();
    assert_eq!(v, vec!["afff", "", "fff"]);

    let s = "a";
    let v: Vec<&str> = s._split("a").collect();
    assert_eq!(v, vec!["", ""]);
    let v: Vec<&str> = s._split("a").rev().collect();
    assert_eq!(v, vec!["", ""]);
}

#[test]
fn test_split_overlapping() {
    let s = "asädfaaaaadfsdfa";
    let v: Vec<&str> = s._split("aa").collect();
    assert_eq!(v, vec!["asädf", "dfsdfa"]);
    let v: Vec<&str> = s._split("aa").rev().collect();
    assert_eq!(v, vec!["dfsdfa", "asädf"]);

    let s = "fffababafff";
    let v: Vec<&str> = s._split("aba").collect();
    assert_eq!(v, vec!["fff", "fff"]);
    let v: Vec<&str> = s._split("aba").rev().collect();
    assert_eq!(v, vec!["fff", "fff"]);
}
