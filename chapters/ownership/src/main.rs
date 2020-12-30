// fn print_type_of<T>(_: &T) {
//     std::any::type_name::<T>()
// }

fn main() {
    let s = String::from("Hello World");

    let hello = &s[0..5];
    let world = &s[6..11];
    let middle = &s[4..7];

    println!("hello = {}, world = {}, middle = {}", hello, world, middle);

    let s1 = String::from("hello1");
    let mut s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let s3 = "hello2";
    let s4 = s3;
    println!("s3 = {}, s4 = {}", s3, s4);


    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward
    // println!("{}", s);
    println!("x: {}", x);
    // let mut len: u32 = 1;
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}