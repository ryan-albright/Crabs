#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self { // note the capitalization
        Self {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30, 
        height: 50,
    };

    let rect2 = Rectangle {
        width: 29, 
        height: 47,
    };

    let rect3 = Rectangle {
        width: 30, 
        height: 53,
    };

    println!("Can rect1 hold rext2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // method calls make things easier to read ...
    let mut r = Rectangle { 
        width: 1,
        height: 2
    };
    // let area1 = r.area();
    // let area2 = Rectangle::area(&r);
    // assert_eq!(area1, area2);

    // r.set_width(2);
    // Rectangle::set_width(&mut r, 2);

    // lines 52 - 57 are equivalent to 
    r.area();
    println!("{r:?}");
    r.set_width(2);
    println!("{r:?}");

}