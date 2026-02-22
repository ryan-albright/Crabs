
fn main() {
    let x = 5;
    let x = x + 1;
    println!("x is {x}");
    {
        let x = x * 2;
        println!("x is {x}");
    }
    println!("x is {x}");
}
