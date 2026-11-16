fn main() {
    println!("{}", fib(0, 1, 1, 100));
}

fn fib(x: u128, y: u128, i: u32, n: u32) -> u128 {
    if i < n { fib(y, x + y, i + 1, n) } else { x }
}
