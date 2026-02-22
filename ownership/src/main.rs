fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("World");
    // we could copy m1 and m2 return them, but Rust offers reference 
    // which provides a much neater solution via the & symbol
    greet(&m1, &m2);
    let s = format!("{} {}", m1, m2);
}

fn greet(g1: &String, g2: &String) {
    println!("{} {}!", g1, g2);
}