// use crate::prelude::*;
use super::*;

/// Search string in strings array with sorting by coof (0.0 - 0%, 1.0 - 100%)
pub fn search<S>(v: &[S], s: &str, min_coef: f32, deep: bool) -> Vec<(f32, String)>
where
    S: AsRef<str>
{
    let mut results: Vec<(f32, String)> = v.iter()
        .map(|vs| {
            let vs = vs.as_ref().to_string();
            let coof = if deep { deep_compare(&vs, s, min_coef) }else{ compare(&vs, s) };
            
            (coof, vs.clone())
        })
        .filter(|(c, _)| *c >= min_coef)
        .collect();

    results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    results
}

/// Search string in custom type array with sorting by coof (0.0 - 0%, 1.0 - 100%)
pub fn search_filter<T, F>(v: &[T], s: &str, min_coef: f32, deep: bool, extract: F) -> Vec<(f32, T)>
where 
    T: Clone,
    F: Fn(&T) -> &str
{
    let mut results: Vec<(f32, T)> = v.iter()
        .map(|item| {
            let vs = extract(item);
            let coof = if deep { deep_compare(&vs, s, min_coef) }else{ compare(&vs, s) };

            (coof, item.clone())
        })
        .filter(|(c, _)| *c >= min_coef)
        .collect();

    results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    results
}
