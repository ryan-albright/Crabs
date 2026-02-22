fn main() {
    let x: u32 = 7;
    println!("This is an unsigned integer: {x}");

    let y: i32 = -8;
    println!("This is a signed integer: {y}");

    // let product = x + y
    println!("We can cannot perform operations between x and y as one is 
            unsigned and one is signed");

    let a: u8 = 2;
    let b: u8 = 255;
    // let sum = a + b;
    println!("We cannot sum a and b as this causes an overflow error as 
            the maximum number that can be stored is 255 and a + b = 257");
    println!("If we use release mode (--release), then the overflow will 
            be ignored and the sum is {sum}")



}
