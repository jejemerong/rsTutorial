fn main() {
    let length1 = 50;
    let width1 = 30;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(length1, width1)
    );
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}

#![allow(unused)]
fn main() {
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

/**
    method 
    impl 안에는 self 인자를 받는 함수를 따로 정의할 수 있다. 
*/
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
}
