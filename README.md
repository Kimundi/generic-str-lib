generic-str-lib
===============

WIP mockup of a more genric Rust str library

See `src/lib.rs` for main user facing change

# Example

```rust
let s = "Foo bar baz. Quuux!";

// Can use char or &str interchangeably
assert!(s._starts_with("Foo"));
assert!(s._ends_with('!'));

assert!(s._contains('.'));
assert!(s._contains("ux!"));

iter_eq!(s._split(' '), ["Foo", "bar", "baz.", "Quuux!"]);
iter_eq!(s._split("ba"), ["Foo ", "r ", "z. Quuux!"]);

// It's implemented for regex as well
static REGEX_1: &'static Regex = &regex!("ba.");
iter_eq!(s._matches(REGEX_1), ["bar", "baz"]);
iter_eq!(s._match_indices(REGEX_1), [(4u, "bar"), (8, "baz")]);

static REGEX_2: Regex = regex!("u+");
assert_eq!(s._find(&REGEX_2), Some(14));

assert!(s._contains(&regex!("Qu+x")));
```

(See `tests/example.rs` for runable version)
