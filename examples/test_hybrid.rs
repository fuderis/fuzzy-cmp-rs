#![cfg(feature = "hybrid")]
use fuzzy_cmp::hybrid_compare;

fn main() {
    let cache = vec![
        "Metallica - Master of Puppets",
        "Metallica - Nothing Else Matters",
        "Linkin Park - Numb (Remastered)",
        "Drake - God's Plan",
        "Metallica - One",
    ];

    let query = "mettalica puppets";

    let mut results: Vec<_> = cache
        .iter()
        .map(|s| (hybrid_compare(s, query), *s))
        .collect();

    results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    let best = results[0].1;

    dbg!(results);

    // hybrid should correctly handle typos + structure
    assert!(best.contains("Metallica"));
}
