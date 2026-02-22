
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    // although black and origin contain the same values, they are different types 
    // as they are instances of different structs
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    // structs can also be defined without fields
    let subject = AlwaysEqual;
}
