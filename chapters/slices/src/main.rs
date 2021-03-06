fn main() {
    let mut s = String::from("hello world");

    // Lets create a function to return ending index of the first word
    let word_index = first_word(&s); // word will get the value 5
    println!("word_index:{}", word_index);

    // Now lets create a function to return slice
    let word = first_word_slice(&s); 
    println!("word:{}", word);

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    // println!("word:{}", &(s.as_str())[..word]);
    // println!("the first word is: {}", word);
}

fn first_word(s: &String) -> usize {

    let bytes = s.as_bytes();

    for (i, &letter) in bytes.iter().enumerate() {
        if letter == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}