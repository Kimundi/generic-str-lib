#![feature(phase, macro_rules)]

extern crate generic_str_lib;
extern crate regex;
#[phase(plugin)] extern crate regex_macros;

use generic_str_lib::StrSlice_;
use regex::Regex;

macro_rules! iter_eq {
    ($i:expr, $s:expr) => {
        {
            let i: Vec<_> = $i.collect();
            let s = $s;
            assert_eq!(i.as_slice(), s.as_slice());
        }
    }
}

#[test]
fn test_all() {
    let s = "Foo bar baz. Quuux!";

    // Can use char or &str interchangeably
    assert!(s._starts_with("Foo"));
    assert!(s._ends_with('!'));

    assert!(s._contains('.'));
    assert!(s._contains("ux!"));

    iter_eq!(s._split(' '), ["Foo", "bar", "baz.", "Quuux!"]);
    iter_eq!(s._split("ba"), ["Foo ", "r ", "z. Quuux!"]);

    // Regular expressions work as well
    // (The Regex is used by reference because that's how its iterators work)
    static REGEX_1: &'static Regex = &regex!("ba.");
    iter_eq!(s._matches(REGEX_1), ["bar", "baz"]);
    iter_eq!(s._match_indices(REGEX_1), [(4u, 7u), (8, 11)]);

    static REGEX_2: Regex = regex!("u+");
    assert_eq!(s._find(&REGEX_2), Some(14));

    assert!(s._contains(&regex!("Qu+x")));

}
