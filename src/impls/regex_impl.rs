extern crate regex;
use super::super::{Pattern, LeftMatcher};
use self::regex::{Regex, FindMatches};

struct RegexMatcher<'a, 'b> {
    str: &'a str,
    regex: FindMatches<'b, 'a>
}
impl<'a, 'b> Pattern<'a, RegexMatcher<'a, 'b>> for &'b Regex {
    fn into_matcher(self, s: &'a str) -> RegexMatcher<'a, 'b> {
        RegexMatcher {
            str: s,
            regex: self.find_iter(s)
        }
    }
    fn is_contained_in(self, s: &str) -> bool {
        self.is_match(s)
    }
}
impl<'a, 'b> LeftMatcher<'a> for RegexMatcher<'a, 'b> {
    fn get_haystack(&self) -> &'a str {
        self.str
    }

    fn next_match(&mut self) -> Option<(uint, &'a str)> {
        self.regex.next().map(|(a, b)| (a, self.str.slice(a, b))) // TODO
    }
}
#[cfg(test)]
mod tests {
    use super::super::super::StrSlice_;
    use super::regex::Regex;
    use std::prelude::{Vec, Iterator};


    #[test]
    fn test1() {
        let s = "abcbdef";
        let r = Regex::new("c").unwrap();
        iter_eq!(s._match_indices(&r), [(2u, "c")]);
    }

    #[test]
    fn test2() {
        let s = "abcbdef";
        let r = Regex::new("b").unwrap();
        iter_eq!(s._match_indices(&r), [(1u, "b"), (3, "b")]);
    }

    #[test]
    fn test3() {
        let s = "ศไทย中华Việt Nam; Mary had a little lamb, Little lamb";
        let r = Regex::new("a[mrd]").unwrap();
        iter_eq!(s._match_indices(&r),
                 [(26, "am"), (31, "ar"), (36, "ad"), (49u, "am"), (62, "am")]);
        iter_eq!(s._matches(&r), ["am", "ar", "ad", "am", "am"]);

        let r = Regex::new("中").unwrap();
        iter_eq!(s._match_indices(&r), [(12u, "中")]);
    }

    #[test]
    fn splitn() {
        let re = Regex::new(r"\d+").unwrap();
        let s = "cauchy123plato456tyler789binx";
        iter_eq!(s._splitn(&re, 2), ["cauchy", "plato", "tyler789binx"]);
    }

    #[test]
    fn split() {
        let re = Regex::new(r"\d+").unwrap();
        let s = "cauchy123plato456tyler789binx";
        iter_eq!(s._split(&re), ["cauchy", "plato", "tyler", "binx"]);
    }

    #[test]
    fn test_starts_with() {
        assert!("foobar"._starts_with(&{Regex::new("fo+").unwrap()}));
        assert!(!"foobar"._starts_with(&{Regex::new("[ob]").unwrap()}));
    }
}
