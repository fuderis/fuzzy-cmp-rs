use fuzzy_cmp::fast_compare;

fn main() {
    let cache = vec![
        "Metallica - Master of Puppets [Live 2024]",
        "Metallica - Nothing Else Matters",
        "Linkin Park - Numb (Remastered)",
        "Drake - God's Plan",
    ];

    let query = "metallica puppets";

    let mut results: Vec<_> = cache.iter().map(|s| (fast_compare(s, query), *s)).collect();
    results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    // top result should be Metallica track
    assert!(results[0].1.contains("Metallica"));
}
