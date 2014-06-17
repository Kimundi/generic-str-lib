extern crate regex;
use super::{Pattern, LeftMatcher};
use self::regex::{Regex, FindMatches};

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
