/// Fast byte-level similarity (O(n))
#[inline(always)]
pub fn fast_compare(a: &str, b: &str) -> f32 {
    let a = a.trim();
    let b = b.trim();

    if a == b {
        return 1.0;
    }

    if a.is_empty() || b.is_empty() {
        return 0.0;
    }

    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    let min_len = a_bytes.len().min(b_bytes.len());
    let max_len = a_bytes.len().max(b_bytes.len()).max(1);

    let mut same = 0usize;

    for i in 0..min_len {
        if a_bytes[i].eq_ignore_ascii_case(&b_bytes[i]) {
            same += 1;
        }
    }

    same as f32 / max_len as f32
}

/// Fast word-based similarity (no allocations, O(n))
#[inline(always)]
pub fn fast_deep_compare(a: &str, b: &str) -> f32 {
    let a = a.trim();
    let b = b.trim();

    if a.is_empty() || b.is_empty() {
        return 0.0;
    }

    let b_words: Vec<&str> = b.split_whitespace().collect();

    let mut total = 0.0;
    let mut count = 0.0;

    for aw in a.split_whitespace() {
        let mut best: f32 = 0.0;

        let aw_bytes = aw.as_bytes();

        for bw in &b_words {
            let bw_bytes = bw.as_bytes();

            let min_len = aw_bytes.len().min(bw_bytes.len());

            let mut same = 0usize;

            for i in 0..min_len {
                if aw_bytes[i].eq_ignore_ascii_case(&bw_bytes[i]) {
                    same += 1;
                }
            }

            let denom = aw_bytes.len().max(bw_bytes.len()).max(1) as f32;

            let mut score = same as f32 / denom;

            if aw.eq_ignore_ascii_case(bw) {
                score += 0.15;
            }

            best = best.max(score.min(1.0));
        }

        total += best;
        count += 1.0;
    }

    if count == 0.0 { 0.0 } else { total / count }
}
