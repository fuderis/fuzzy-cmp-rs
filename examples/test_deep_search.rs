fn main() {
    let files = vec![
        "Metallica - Master of Puppets [Live 2024].mp3",
        "Metallica - Master of Pupets [Remix].mp3",
        "Metallica Nothing Else Matters [Live].mp3",
        "Led Zeppelin - Stairway to Heaven.mp3",
    ];
    let query = "metallica puppets";
    let min_coef = 0.45;
    let results = fuzzy_cmp::search(&files, query, min_coef, true); // deep=true
    
    println!("Deep file search (coef {min_coef}):");
    println!("Search files: {files:#?}");
    println!("Search query: {query}");

    println!("Results: ");
    for (coef, file) in results.iter().take(3) {
        println!("  {:.2}% → {}", coef * 100.0, file);
    }
}
