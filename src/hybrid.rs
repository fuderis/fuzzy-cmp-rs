use crate::*;

/// Hybrid fuzzy compare:
/// - fast stage first
/// - Levenshtein fallback only if needed
#[inline(always)]
pub fn hybrid_compare(a: &str, b: &str) -> f32 {
    // exact:
    if a.eq_ignore_ascii_case(b) {
        return 1.0;
    }

    // substring:
    if a.contains(b) || b.contains(a) {
        return 0.95;
    }

    // token fuzzy:
    let b_tokens: Vec<&str> = b.split_whitespace().collect();

    let mut matched = 0.0;
    let mut seen: f32 = 0.0;

    for at in a.split_whitespace() {
        let mut best: f32 = 0.0;

        for bt in &b_tokens {
            let score = if at.eq_ignore_ascii_case(bt) {
                1.0
            } else {
                fast_compare(at, bt).max(lev_compare(at, bt))
            };

            best = best.max(score);
        }

        // hard thresholding (removes noise accumulation)
        if best >= 0.6 {
            matched += 1.0;
        }

        seen += 1.0;
    }

    matched / seen.max(1.0)
}
