#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn square(side: u32) -> Self {
        Self {
            height: side,
            width: side
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height * self.width > other.height * other.width
    }

    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main(){
    let rec1: Rectangle = Rectangle { height: 20, width: 30 };
    let rec2: Rectangle = Rectangle { height: 40, width: 80 };
    let square: Rectangle = Rectangle::square(5);

    dbg!(&square);
    println!("Are of rectangle is {} and the struct used is {rec1:#?}",rec1.area());
    println!("rec2 is bigger than rec1: {}", rec2.can_hold(&rec1));
}

