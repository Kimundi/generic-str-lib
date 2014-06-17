use super::super::{Pattern, LeftMatcher, Matcher, slice, Splits, StrExt};
use super::{splits_next, splits_next_back};

pub struct TermSplits<M> {
    splits: Splits<M>
}

impl<M> TermSplits<M> {
    pub fn new<'a, P: Pattern<'a, M>>(s: &'a str, pat: P) -> TermSplits<M> {
        TermSplits {
            splits: Splits::new(s, pat)
        }
    }
}

impl<'a, M: LeftMatcher<'a>> Iterator<&'a str> for TermSplits<M> {
    fn next(&mut self) -> Option<&'a str> {
        let string = self.splits.matcher.get_haystack();
        loop {
            match splits_next(&mut self.splits) {
                Some((a, b)) if a == b && b == string.len() => continue,
                Some((a, b)) => return Some(slice(string, a, b)),
                None => return None
            }
        }
    }
}

impl<'a, M: Matcher<'a>> DoubleEndedIterator<&'a str> for TermSplits<M> {
    fn next_back(&mut self) -> Option<&'a str> {
        let string = self.splits.matcher.get_haystack();
        loop {
            match splits_next_back(&mut self.splits) {
                Some((a, b)) if a == b && b == string.len() => continue,
                Some((a, b)) => return Some(slice(string, a, b)),
                None => return None
            }
        }
    }
}

#[test]
fn test_split_terminator() {
    let s = "asädfadfsdfa";
    let v: Vec<&str> = s._split_terminator("a").collect();
    assert_eq!(v, vec!["", "sädf", "dfsdf"]);
    let v: Vec<&str> = s._split_terminator("a").rev().collect();
    assert_eq!(v, vec!["dfsdf", "sädf", ""]);

    let s = "asädfaaaaadfsdfa";
    let v: Vec<&str> = s._split_terminator("a").collect();
    assert_eq!(v, vec!["", "sädf", "", "", "", "", "dfsdf"]);
    let v: Vec<&str> = s._split_terminator("a").rev().collect();
    assert_eq!(v, vec!["dfsdf", "", "", "", "", "sädf", ""]);

    let s = "fffababafff";
    let v: Vec<&str> = s._split_terminator("ab").collect();
    assert_eq!(v, vec!["fff", "", "afff"]);
    let v: Vec<&str> = s._split_terminator("ab").rev().collect();
    assert_eq!(v, vec!["afff", "", "fff"]);

    let s = "a";
    let v: Vec<&str> = s._split_terminator("a").collect();
    assert_eq!(v, vec![""]);
    let v: Vec<&str> = s._split_terminator("a").rev().collect();
    assert_eq!(v, vec![""]);
}

#[test]
fn test_split_terminator_overlapping() {
    let s = "asädfaaaaadfsdfa";
    let v: Vec<&str> = s._split_terminator("aa").collect();
    assert_eq!(v, vec!["asädf", "dfsdfa"]);
    let v: Vec<&str> = s._split_terminator("aa").rev().collect();
    assert_eq!(v, vec!["dfsdfa", "asädf"]);

    let s = "fffababafff";
    let v: Vec<&str> = s._split_terminator("aba").collect();
    assert_eq!(v, vec!["fff", "fff"]);
    let v: Vec<&str> = s._split_terminator("aba").rev().collect();
    assert_eq!(v, vec!["fff", "fff"]);
}
