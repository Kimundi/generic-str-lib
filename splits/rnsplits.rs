use super::super::{Pattern, LeftMatcher, Matcher, Splits, StrSlice_};

pub struct RNSplits<M> {
    splits: Splits<M>,
    count: uint
}

impl<M> RNSplits<M> {
    pub fn new<'a, P: Pattern<'a, M>>(s: &'a str, pat: P, n: uint) -> RNSplits<M> {
        RNSplits {
            splits: s._split(pat),
            count: n
        }
    }
}

impl<'a, M: Matcher<'a>> Iterator<&'a str> for RNSplits<M> {
    fn next(&mut self) -> Option<&'a str> {
        if self.count == 0 {
            let end = self.splits.prev_end;
            let s = self.splits.matcher.get_haystack();
            if self.splits.finished {
                None
            } else {
                self.splits.finished = true;
                Some(s.slice_to(end))
            }
        } else {
            self.count -= 1;
            self.splits.next_back()
        }
    }
}

#[test]
fn test_rsplitn100() {
    let s = "asädfadfsdfa";
    let v: Vec<&str> = s._rsplitn("a", 100).collect();
    assert_eq!(v, vec!["", "dfsdf", "sädf", ""]);

    let s = "asädfaaaaadfsdfa";
    let v: Vec<&str> = s._rsplitn("a", 100).collect();
    assert_eq!(v, vec!["", "dfsdf", "", "", "", "", "sädf", ""]);

    let s = "fffababafff";
    let v: Vec<&str> = s._rsplitn("ab", 100).collect();
    assert_eq!(v, vec!["afff", "", "fff"]);
}

#[test]
fn test_rsplitn_overlapping100() {
    let s = "asädfaaaaadfsdfa";
    let v: Vec<&str> = s._rsplitn("aa", 100).collect();
    assert_eq!(v, vec!["dfsdfa", "asädf"]);

    let s = "fffababafff";
    let v: Vec<&str> = s._rsplitn("aba", 100).collect();
    assert_eq!(v, vec!["fff", "fff"]);
}

#[test]
fn test_rsplitn2() {
    let s = "asädfadfsdfa";
    let v: Vec<&str> = s._rsplitn("a", 2).collect();
    assert_eq!(v, vec!["", "dfsdf", "asädf"]);

    let s = "asädfaaaaadfsdfa";
    let v: Vec<&str> = s._rsplitn("a", 2).collect();
    assert_eq!(v, vec!["", "dfsdf", "asädfaaaa"]);

    let s = "fffababafff";
    let v: Vec<&str> = s._rsplitn("ab", 2).collect();
    assert_eq!(v, vec!["afff", "", "fff"]);
}

#[test]
fn test_rsplitn_overlapping2() {
    let s = "asädfaaaaadfsdfa";
    let v: Vec<&str> = s._rsplitn("aa", 2).collect();
    assert_eq!(v, vec!["dfsdfa", "asädf"]);

    let s = "fffababafff";
    let v: Vec<&str> = s._rsplitn("aba", 2).collect();
    assert_eq!(v, vec!["fff", "fff"]);
}

#[test]
fn test_rsplitn0() {
    let s = "asädfadfsdfa";
    let v: Vec<&str> = s._rsplitn("a", 0).collect();
    assert_eq!(v, vec![s]);

    let s = "asädfaaaaadfsdfa";
    let v: Vec<&str> = s._rsplitn("a", 0).collect();
    assert_eq!(v, vec![s]);

    let s = "fffababafff";
    let v: Vec<&str> = s._rsplitn("ab", 0).collect();
    assert_eq!(v, vec![s]);
}

#[test]
fn test_rsplitn_overlapping0() {
    let s = "asädfaaaaadfsdfa";
    let v: Vec<&str> = s._rsplitn("aa", 0).collect();
    assert_eq!(v, vec![s]);

    let s = "fffababafff";
    let v: Vec<&str> = s._rsplitn("aba", 0).collect();
    assert_eq!(v, vec![s]);
}
