use super::StrExt;

#[test]
fn test_starts_with() {
    assert!("foobar"._starts_with("foo"));
    assert!("foobar"._starts_with('f'));

    assert!(!"foobar"._starts_with("oba"));
    assert!(!"foobar"._starts_with('o'));
}

#[test]
fn test_end_with() {
    assert!("foobar"._ends_with("bar"));
    assert!("foobar"._ends_with('r'));

    assert!(!"foobar"._ends_with("oob"));
    assert!(!"foobar"._ends_with('b'));
}

#[test]
fn test_trim_left() {
    assert_eq!("  ajklasd  "._trim_left_matches(" "), "ajklasd  ");
    assert_eq!("  ajklasd  "._trim_left_matches(' '), "ajklasd  ");
}

#[test]
fn test_trim_right() {
    assert_eq!("  ajklasd  "._trim_right_matches(" "), "  ajklasd");
    assert_eq!("  ajklasd  "._trim_right_matches(' '), "  ajklasd");
}

#[test]
fn test_trim() {
    assert_eq!("  ajklasd  "._trim_matches(" "), "ajklasd");
    assert_eq!("  ajklasd  "._trim_matches(' '), "ajklasd");
}

#[test]
fn test_find() {
    assert_eq!("abaaaba"._find("b"), Some(1));
    assert_eq!("abaaaba"._find("a"), Some(0));
    assert_eq!("abaaaba"._find("c"), None);
}

#[test]
fn test_rfind() {
    assert_eq!("abaaaba"._rfind("b"), Some(5));
    assert_eq!("abaaaba"._rfind("a"), Some(6));
    assert_eq!("abaaaba"._rfind("c"), None);
}

#[test]
fn test_replace() {
    assert_eq!("a"._replace("?", "b").as_slice(), "a");
    assert_eq!("a"._replace("a", "b").as_slice(), "b");
    assert_eq!("a"._replace('?', "b").as_slice(), "a");
    assert_eq!("a"._replace('a', "b").as_slice(), "b");
}
