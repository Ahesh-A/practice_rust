use std::io;

fn fib(n: u32) -> u32 {
    if n == 1 || n == 2 {return 1;}
   
    return fib(n - 1) + fib(n - 2);
}
fn main() {
    let mut n = String::new();
    println!("Enter a number :");
    io::stdin().read_line(& mut n).expect("Failed to read input");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            return;
        }
    };
    println!("{}", fib(n));

}
