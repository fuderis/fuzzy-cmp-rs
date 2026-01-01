#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let people = vec![
        Person { name: "Alice".to_owned(), age: 30 },
        Person { name: "Bob".to_owned(), age: 25 },
        Person { name: "Alicia".to_owned(), age: 28 },
    ];

    let results = fuzzy_cmp::search_filter(&people, "Ali", 0.6, |p: &Person| &p.name);
    let best = &results[0];
    
    println!("Best result: {best:?}");
    assert_eq!(best.1, Person { name: "Alice".to_owned(), age: 30 });
}
