fn main() {
    let files = vec![
        "All Fires",
        "Five Finger Death Punch"
    ];
    let query = "five finger";
    let min_coef = 0.5;
    let results = fuzzy_cmp::search(&files, query, min_coef, true); // deep=true
    
    println!("Deep file search (coef {min_coef}):");
    println!("Search files: {files:#?}");
    println!("Search query: {query}");

    println!("Results: ");
    for (coef, file) in results.iter().take(3) {
        println!("  {:.2}% → {}", coef * 100.0, file);
    }
}
