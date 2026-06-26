/// Levenshtein distance (raw)
#[inline(always)]
pub fn lev_distance(a: &str, b: &str) -> usize {
    distance::levenshtein(a, b)
}

/// Accurate similarity based on Levenshtein distance
#[inline(always)]
pub fn lev_compare(a: &str, b: &str) -> f32 {
    let a = a.trim();
    let b = b.trim();

    if a == b {
        return 1.0;
    }

    if a.is_empty() || b.is_empty() {
        return 0.0;
    }

    let dist = lev_distance(a, b);
    let max_len = a.chars().count().max(b.chars().count()).max(1);
    let score = 1.0 - (dist as f32 / max_len as f32);

    score.clamp(0.0, 1.0)
}
