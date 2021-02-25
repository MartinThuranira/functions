fn main() {
    let x = five();
    println!("The value of x is: {}", x);
    println!("Hello, world!");
    another_function();
    five();
}
fn another_function() {
    println!("Another function.");
}
fn five() -> i32 {
    5
}
