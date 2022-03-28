pub fn invoke(){
    let rect1 = Rectangle { width: 5, length: 10 };
    println!("rect1: {:#?}", rect1);
    println!("rect1 area: {}", rect1.area());
    let rect2 = Rectangle { width: 5, length: 5 };
    let rect3 = Rectangle { width: 15, length: 10 };
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3: {}", rect1.can_hold(&rect3));
    let square = Rectangle::square(10);
    println!("square: {:?}", square);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32
}

impl Rectangle {
    fn area(&self) -> u32 { self.width * self.length }
    fn can_hold(&self, other: &Rectangle) -> bool { self.width >= other.width && self.length >= other.length }
    fn square(length: u32) -> Rectangle { Rectangle { width: length, length } }
}