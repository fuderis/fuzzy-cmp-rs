[![github]](https://github.com/fuderis/fuzzy-cmp-rs)&ensp;
[![crates-io]](https://crates.io/crates/fuzzy-cmp)&ensp;
[![docs-rs]](https://docs.rs/fuzzy-cmp)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Fuzzy Compare 

Fast, lightweight fuzzy string comparison library for Rust with multiple similarity strategies:
byte-level, word-level, Levenshtein-based, and hybrid scoring.<br>

Designed for speed-first matching with optional deeper accuracy when needed.

## Features:

* Very fast byte-level comparison (O(n))
* Word-based fuzzy matching (no heavy allocations in hot path)
* Levenshtein distance integration
* Hybrid scoring (fast pre-check + smart fallback logic)
* No heavy dependencies (only optional distance crate)
* Case-insensitive matching built-in

## Comparison Methods:

1. Fast byte comparison

Simple and extremely fast similarity check.

Best for:
* autocomplete
* filtering
* quick ranking

```rust
fast_compare(a, b)
```

Returns a value in range:

```
0.0 → completely different
1.0 → identical
```

2. Word-based deep comparison

Token-aware fuzzy matching without heavy allocations.

```rust
fast_deep_compare(a, b)
```

Best for:
* search ranking
* partial phrase matching
* noisy user input

3. Levenshtein similarity [feature lev]

Classic edit-distance-based accuracy scoring.

```rust
lev_compare(a, b)
```

Best for:
* typo correction
* spelling similarity
* canonical matching

4. Hybrid comparison (recommended) [feature hybrid]

Combines multiple strategies:
* exact match shortcut
* substring detection
* token fuzzy scoring
* Levenshtein fallback

```rust
hybrid_compare(a, b)
```

Best for:
* search engines
* recommendation systems
* fuzzy deduplication
* NLP preprocessing pipelines

## Examples:

```rust
use fuzzy_compare::*;

fn main() {
    let a = "hello world";
    let b = "helo wrld";

    println!("fast: {}", fast_compare(a, b));
    println!("deep: {}", fast_deep_compare(a, b));

    #[cfg(feature = "lev")]
    println!("lev: {}", lev_compare(a, b));

    #[cfg(feature = "hybrid")]
    println!("hybrid: {}", hybrid_compare(a, b));
}
```

## Performance Notes:

* **fast_compare:** is pure byte scan → extremely fast
* **fast_deep_compare:** avoids heavy allocations in inner loop (except token split once per input)
* **hybrid_compare:** short-circuits aggressively for speed (Levenshtein is only used when needed)

| Method            | Speed      | Accuracy | Use case           |
| ----------------- | ---------- | -------- | ------------------ |
| fast_compare      | ⚡⚡⚡⚡⚡ | Low      | filtering, ranking |
| fast_deep_compare | ⚡⚡⚡⚡   | Medium   | search             |
| lev_compare       | ⚡⚡       | High     | typo correction    |
| hybrid_compare    | ⚡⚡⚡     | High     | production default |

## Contributing:

Pull requests welcome. Focus areas:

* SIMD optimization
* allocation reduction in token logic
* configurable thresholds for hybrid scoring
* Unicode-aware improvements

## License & Feedback:

> This library distributed under the [MIT](https://github.com/fuderis/fuzzy-cmp-rs/blob/main/LICENSE.md) license.

You can contact me via [GitHub](https://github.com/fuderis) or send a message to my [E-Mail](mailto:synapdrake@ya.ru).
This library is actively evolving, and your suggestions and feedback are always welcome!
