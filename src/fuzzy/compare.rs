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

/// Deep comparison using word-by-word scoring with positional bonus
pub fn deep_compare(s1: &str, s2: &str, min_coef: f32) -> f32 {
    let s1 = s1.trim().to_lowercase();
    let s2 = s2.trim().to_lowercase();
    if s1.is_empty() || s2.is_empty() { return 0.0; }

    // split by words:
    let words1: Vec<&str> = s1.split_whitespace().collect();
    let words2: Vec<&str> = s2.split_whitespace().collect();
    
    // get len's:
    let s1_len = words1.len();
    let s2_len = words2.len();
    let max_len = s1_len.max(s2_len);
    
    let mut scores = vec![0f32; max_len];
    let mut covered = vec![false; max_len];
    
    // compare words (s1 vs s2):
    for (i, word1) in words1.iter().take(max_len).enumerate() {
        let mut best_score = 0.0f32;
        
        for (j, word2) in words2.iter().enumerate() {
            let coef = compare(word1, word2);
            let mut score = coef / 5.0;
            
            // add position bonus: |i - j| <= 1
            if (i as i32 - j as i32).abs() <= 1 {
                score += 1.0;
            }
            
            // update best score:
            best_score = best_score.max(score);

            // add coverage score:
            if coef >= min_coef {
                covered[j] = true;
            }
        }
        
        scores[i] = best_score.min(6.0);
    }
    
    // calc compare scores:
    let max_score = 6.0 * max_len as f32;
    let compare_coef = scores.iter().sum::<f32>() / max_score;

    // calc coverage scores:
    let covered_count = covered.into_iter().filter(|&x| x).count();
    let coverage_coef = (covered_count as f32 / s2_len as f32).min(1.0);
    
    compare_coef + coverage_coef
}
