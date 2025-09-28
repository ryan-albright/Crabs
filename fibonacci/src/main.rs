fn main() {
    let n: u32 = 40;
    let result: u32 = fibonacci_simple(n);
    println!("The {n} fibonacci number is {result}");
}

fn fibonacci_simple (n: u32) -> u32{
    if n == 1 || n == 2 {
        return 1
    }
    else {
        let mut i: u32 = 2;
        let mut sum_n_1: u32 = 1;
        let mut sum_n: u32 = 1;
        while i < n {
            (sum_n, sum_n_1) = (sum_n + sum_n_1, sum_n);
            i += 1;
        }
        return sum_n
    }
}