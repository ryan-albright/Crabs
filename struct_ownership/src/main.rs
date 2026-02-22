// struct User{
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64
// }

struct Point{
    x: i32, 
    y: i32
}

fn main() {
    // this will toss an error as lifetimes were not assigned
    // let user1 = User {
    //     active: true,
    //     username: "someusername123",
    //     email: "someone@example.com",
    //     sign_in_count: 1,
    // };

    let mut p = Point{x: 0, y: 0};

    let x = &mut p.x; // x now has read and ownership permission of p.x

    // if we tried to use p.x here, an error would be generated as 
    // p.x does not have ownership permission
    // println!("{}", p.x);

    *x += 1; // *x now has write permission to p.x

    println!("{}", p.x);
}
