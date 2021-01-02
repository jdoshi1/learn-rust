use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{:?}", map);
    // println!("{}", field_value);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("Blue");
    let score = scores.get(&String::from("Blue"));
    match score {
        Some(score) => println!("Score:{}", score),
        None => println!("Not found"),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // println!("{:?}", scores);
    scores.entry(String::from("Yellow")).or_insert(20);
    scores.entry(String::from("Blue")).or_insert(60);
    scores.entry(String::from("Pink")).or_insert(30);
    println!("{:?}", scores);


    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
