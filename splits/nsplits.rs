use super::super::{Pattern, LeftMatcher, Splits, StrExt};

pub struct NSplits<M> {
    splits: Splits<M>,
    count: uint
}

impl<M> NSplits<M> {
    pub fn new<'a, P: Pattern<'a, M>>(s: &'a str, pat: P, n: uint) -> NSplits<M> {
        NSplits {
            splits: s._split(pat),
            count: n
        }
    }
}

impl<'a, M: LeftMatcher<'a>> Iterator<&'a str> for NSplits<M> {
    fn next(&mut self) -> Option<&'a str> {
        if self.count == 0 {
            let start = self.splits.prev_start;
            let s = self.splits.matcher.get_haystack();
            if self.splits.finished {
                None
            } else {
                self.splits.finished = true;
                Some(s.slice_from(start))
            }
        } else {
            self.count -= 1;
            self.splits.next()
        }
    }
}

#[test]
fn test_splitn100() {
    let s = "asädfadfsdfa";
    let v: Vec<&str> = s._splitn("a", 100).collect();
    assert_eq!(v, vec!["", "sädf", "dfsdf", ""]);

    let s = "asädfaaaaadfsdfa";
    let v: Vec<&str> = s._splitn("a", 100).collect();
    assert_eq!(v, vec!["", "sädf", "", "", "", "", "dfsdf", ""]);

    let s = "fffababafff";
    let v: Vec<&str> = s._splitn("ab", 100).collect();
    assert_eq!(v, vec!["fff", "", "afff"]);
}

#[test]
fn test_splitn_overlapping100() {
    let s = "asädfaaaaadfsdfa";
    let v: Vec<&str> = s._splitn("aa", 100).collect();
    assert_eq!(v, vec!["asädf", "dfsdfa"]);

    let s = "fffababafff";
    let v: Vec<&str> = s._splitn("aba", 100).collect();
    assert_eq!(v, vec!["fff", "fff"]);
}

#[test]
fn test_splitn2() {
    let s = "asädfadfsdfa";
    let v: Vec<&str> = s._splitn("a", 2).collect();
    assert_eq!(v, vec!["", "sädf", "dfsdfa"]);

    let s = "asädfaaaaadfsdfa";
    let v: Vec<&str> = s._splitn('a', 2).collect();
    assert_eq!(v, vec!["", "sädf", "aaaadfsdfa"]);

    let s = "fffababafff";
    let v: Vec<&str> = s._splitn("ab", 2).collect();
    assert_eq!(v, vec!["fff", "", "afff"]);
}

#[test]
fn test_splitn_overlapping2() {
    let s = "asädfaaaaadfsdfa";
    let v: Vec<&str> = s._splitn("aa", 2).collect();
    assert_eq!(v, vec!["asädf", "dfsdfa"]);

    let s = "fffababafff";
    let v: Vec<&str> = s._splitn("aba", 2).collect();
    assert_eq!(v, vec!["fff", "fff"]);
}

#[test]
fn test_splitn0() {
    let s = "asädfadfsdfa";
    let v: Vec<&str> = s._splitn("a", 0).collect();
    assert_eq!(v, vec![s]);

    let s = "asädfaaaaadfsdfa";
    let v: Vec<&str> = s._splitn("a", 0).collect();
    assert_eq!(v, vec![s]);

    let s = "fffababafff";
    let v: Vec<&str> = s._splitn("ab", 0).collect();
    assert_eq!(v, vec![s]);
}

#[test]
fn test_splitn_overlapping0() {
    let s = "asädfaaaaadfsdfa";
    let v: Vec<&str> = s._splitn("aa", 0).collect();
    assert_eq!(v, vec![s]);

    let s = "fffababafff";
    let v: Vec<&str> = s._splitn("aba", 0).collect();
    assert_eq!(v, vec![s]);
}
