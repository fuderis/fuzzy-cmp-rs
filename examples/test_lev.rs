#![cfg(feature = "lev")]
use fuzzy_cmp::lev_compare;

fn main() {
    let cache = vec![
        "Metallica - Master of Puppets",
        "Metallica - Nothing Else Matters",
        "Linkin Park - Numb",
    ];

    let query = "mettalica pupets"; // typo case

    let mut results: Vec<_> = cache.iter().map(|s| (lev_compare(s, query), *s)).collect();
    results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    // Levenshtein fixes typos → correct match wins
    assert!(results[0].1.contains("Metallica"));
}
