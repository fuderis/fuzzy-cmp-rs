// use crate::prelude::*;

/// Returns levenshtein distance between strings
pub fn distance(s1: &str, s2: &str) -> usize {
    distance::levenshtein(&s1, &s2)
}

/// Compares 2 string by levenshtein distance
pub fn compare(s1: &str, s2: &str) -> f32 {
    let s1 = s1.trim().to_lowercase();
    let s2 = s2.trim().to_lowercase();
    
    if s1 == s2 { return 1.0; }
    else if s1.is_empty() || s2.is_empty() { return 0.0; }
    
    let dist = distance(&s1, &s2);
    let max_len = s1.chars().count().max(s2.chars().count()).max(1);
    let coof = 1.0 - (dist as f32 / max_len as f32);

    coof.max(0.0).min(1.0)
}

/// Search string in strings array with sorting by coof (0.0 - 0%, 1.0 - 100%)
pub fn search(v: &[impl AsRef<String>], s: &str, min_coof: f32) -> Vec<(f32, String)> {
    let mut results: Vec<(f32, String)> = v.iter()
        .map(|vs| {
            let vs = vs.as_ref();
            let coof = compare(&vs, s);
            
            (coof, vs.clone())
        })
        .filter(|(c, _)| *c >= min_coof)
        .collect();

    results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    results
}

/// Search string in custom type array with sorting by coof (0.0 - 0%, 1.0 - 100%)
pub fn search_filter<T, F>(v: &[T], s: &str, min_coof: f32, extract: F) -> Vec<(f32, T)>
where 
    T: Clone,
    F: Fn(&T) -> &str,
{
    let mut results: Vec<(f32, T)> = v.iter()
        .map(|item| {
            let vs = extract(item);
            let coof = compare(vs, s);

            (coof, item.clone())
        })
        .filter(|(c, _)| *c >= min_coof)
        .collect();

    results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    results
}
