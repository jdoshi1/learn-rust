fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    let x = 2.0; // f64

    let y: f32 = 3.0134; // f32

    println!("The value of x y: {} {}", x, y);
    
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("The value of c z heart_eyed_cat: {} {} {}", c, z, heart_eyed_cat);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    let a = [1, 2, 3, 4, 5];

    println!("The value x,y,z, tup.0: {} {} {} {}", x, y, z, tup.0);
    println!("The value a[1]: {}", a[1]);

    let a = [1, 2, 3, 4, 5];
    let index = 1;

    let element = a[index];

    println!("The value of element is: {}", element);

    let q = 6;
    let p = q;
    println!("The value p q: {}, {}", p,q);

    let mut m = 3;
    println!("The value m: {}", m);
    let l = {let  m = 6; m};
    m += 1;
    println!("The value m, l: {}, {}", m, l);
    loop_test();
    liftoff();

    let mut s = String::from("hello!"); // type String; Same as String::from("hello")
    let t = String::new();
    s.push_str("world!");
   
    // let s = "hello!"; // type &str
    println!("The value s, t: {}, {}", s, t);
    // println!("The value t: {}", t.push_str("world!"));

    let name_domain = String::from("substrate.dev");
    let name_state = String::from(" loves Rust!");
    // let v = name_domain + &name_state;
    // println!("The value: {}", v);

    let w = format!("{}{}", name_domain, name_state);
    println!("{} - {}", w, format!("{}{}", name_domain, name_state));

    

    let k = s; // move occurs because `s` has type `String`, which does not implement the `Copy` trait
    println!("The value k: {}", k);
    // println!("The value s: {}", s); // value borrowed by k after move. s does not exist anymore
    let d = "blah";
    
    println!("The value d: {}", d);
}


fn loop_test() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("The result is {}", result);
}

fn liftoff() {
    let mut number = 3;

    while number != 0 {
        print!("{}..", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    for n in (1..4).rev() {
        print!("{}..", n);
    }
    println!("LIFTOFF!!!");
}
