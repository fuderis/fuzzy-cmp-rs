fn main() {
    let s1 = "hello world";
    let s2 = "hello my friend";
    let min_coef = 0.4;
    let coef = fuzzy_cmp::deep_compare(s1, s2, min_coef);

    println!("Coefficient: {min_coef}");
    println!("String1: {s1}");
    println!("String2: {s2}");
    println!("Result: {coef} or {:.2}%", coef * 100.0);
}
