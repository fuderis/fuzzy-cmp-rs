[![github]](https://github.com/fuderis/rs-fuzzy-cmp)&ensp;
[![crates-io]](https://crates.io/crates/fuzzy-cmp)&ensp;
[![docs-rs]](https://docs.rs/fuzzy-cmp)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Fuzzy Compare

Rust library for fuzzy string matching using Levenshtein distance. Returns results with similarity coefficient (0.0 = 0%, 1.0 = 100%) sorted from best to worst match.


## Features:

* **Performance**: O(n × L²) where L is average string length.
* **Case-insensitive**: With automatic lowercase conversion.
* **Safe**: Handles empty strings, division by zero.
* **Generic**: Works with any `Clone` type via closure.
* **Sorting**: Descending sort by coefficient.


## Examples:

```rust
#[derive(Debug, Clone, Eq, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let people = vec![
        Person { name: "Alice".to_owned(), age: 30 },
        Person { name: "Bob".to_owned(), age: 25 },
        Person { name: "Alicia".to_owned(), age: 28 },
    ];

    let results = fuzzy_cmp::search_filter(&people, "Ali", 0.6, |p: &Person| &p.name);
    let best = &results[0];
    
    println!("Best result: {best:?}");
    assert_eq!(best.1, Person { name: "Alice".to_owned(), age: 30 });
}
```


## Licensing:

Distributed under the MIT license.


## Feedback:

You can [find me here](https://t.me/fuderis), also [see my channel](https://t.me/fuderis_club).
I welcome your suggestions and feedback!

> Copyright (c) 2025 *Bulat Sh.* ([fuderis](https://t.me/fuderis))
