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

### Compare strings:
```rust
fn main() {
    let s1 = "hello world";
    let s2 = "hello my friend";
    let min_coef = 0.4;
    let coef = fuzzy_cmp::deep_compare(s1, s2, min_coef);

    println!("Coefficient: {min_coef}");
    println!("String1: {s1}");
    println!("String2: {s2}");
    println!("Result: {coef} or {:.2}%", coef * 100.0);

    // -> Coefficient: 0.4
    // -> String1: hello world
    // -> String2: hello my friend
    // -> Result: 0.4577778 or 45.78%
}
```

### Deep search:
```rust
fn main() {
    let files = vec![
        "Metallica - Master of Puppets [Live 2024].mp3",
        "Metallica - Master of Pupets [Remix].mp3",
        "Metallica Nothing Else Matters [Live].mp3",
        "Led Zeppelin - Stairway to Heaven.mp3"
    ];
    
    let query = "metallica puppets";
    let min_coef = 0.45;
    let results = fuzzy_cmp::search(&files, query, min_coef, true); // deep=true
    
    println!("Deep file search (coef {min_coef}):");
    println!("Search files: {files:#?}");
    println!("Search query: {query}");

    println!("Results: ");
    for (coef, file) in results.iter().take(3) {
        println!("  {:.2}% → {}", coef * 100.0, file);
    }

    // -> Deep file search (coef 0.45):
    // -> Search files: [
    // ->     "Metallica - Master of Puppets [Live 2024].mp3",
    // ->     "Metallica - Master of Pupets [Remix].mp3",
    // ->     "Metallica Nothing Else Matters [Live].mp3",
    // ->     "Led Zeppelin - Stairway to Heaven.mp3",
    // -> ]
    // -> Search query: metallica puppets
    // -> Results:
    // ->   100.00% → Metallica - Master of Puppets [Live 2024].mp3
    // ->   100.00% → Metallica - Master of Pupets [Remix].mp3
    // ->   61.17% → Metallica Nothing Else Matters [Live].mp3
}
```

### Search struct:
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

    let results = fuzzy_cmp::search_filter(&people, "Ali", 0.6, false, |p: &Person| &p.name);
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
