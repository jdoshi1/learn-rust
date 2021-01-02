fn main() {
    let v: Vec<i32> = Vec::new();
    println!("v:{:?}", v);

    let mut v = Vec::new();
    println!("v:{:?}", v.push(1));
    println!("v:{:?}", v);

    let mut v = vec![1, 2, 3, 4];
    println!("v:{:?}", v);

    v.push(5);
    println!("v:{:?}", v);
    println!("v[2]:{}", v[2]);
    println!("&v[2]:{}", &v[2]);
    let value = v.get(2);
    println!("v:{:?}", value);

    match value {
        Some(v) => println!("vOption:{:?}", v),
        None => println!("There is no third element.")
    }


    let first = &v[0];
    // v.push(6);
    println!("The first element is: {}", first);

    for i in &v {  // for i in v {} -> works too
        print!("{}..", i);
    }

    // for mut i in v {
    //     i += 50;
    //     print!("{}..", i);
    // }

    for i in &mut v {
        *i += 50;
        print!("{}..", *i);
    }

    let mut s1 = String::from("foo");
    let s2 = "bar"; // string slice
    s1.push_str(s2); // string slice - so borrowed ownership
    println!("s2 is {}", s2);
    println!("s1 is {}", s1);

    let s3 = String::from("bar2");
    println!("s2 is {}", s3);

    s1.push_str(&s3); // borrowed ownership of s3
    println!("s1 is {}", s1);


    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // println!("s1 is {}", s1); // s1 lost ownership and hence can't be used anymore
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {}", s);
    // println!("s1 is {}", s1); // s1 lost ownership and hence can't be used anymore
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);
    let s1 = String::from("tic"); // create new s1. Otherwise we cannot use the previous one.
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);

    let hello = String::from("Hola");
    println!("len of hello is {}", hello.len());
    let first = &hello[0..1];
    println!("The first element is: {}", first);

    let hello = String::from("Здравствуйте");
    println!("len of hello is {}", hello.len());
    let first = &hello[0..2];
    println!("The first element is: {}", first);

    let hello = String::from("नमस्ते");
    println!("len of hello is {}", hello.len());
    let first = &hello[0..3];
    println!("The first element is: {}", first);

    for (index, c) in "नमस्ते".chars().enumerate() {
        println!("{} -> {}", index,c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
