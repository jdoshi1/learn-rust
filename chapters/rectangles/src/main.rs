#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // --- 1
    let width = 30;
    let height = 50;
    println!("The area of rectangle is: {}", area(width, height));

    // --- 2
    let rec1 = (30, 50);
    println!("The area of rectangle is: {}", area2(rec1));


    // --- 3
    let rec3 = Rectangle {
        width:30, 
        height:50
    };
    println!("The area of rectangle is: {}", area3(&rec3));
    println!("rect3.width: {}", rec3.width);
    println!("rect3: {:?}", rec3);
    println!("rect3: {:#?}", rec3);

    // --- 4
    let rec4 = Rectangle { 
        width:30, 
        height:50
    };
    println!("The area of rectangle4 is: {}", rec4.area());

    // --- 5
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // --- 6
    let sq = Rectangle::square(20);
    println!("rec (sq) {}*{} = {}", sq.width, sq.height, sq.area());
}

fn area(width: u32, height: u32) -> u32 {
    width*height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}
