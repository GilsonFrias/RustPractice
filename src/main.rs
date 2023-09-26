fn main() {
    let mut x:u8 = 5;
    let mut _string:u8 = "32".parse().expect("Not a number!");
    println!("The value of _string is: {_string}");
    x = 6;
    println!("The value of x is: {x}");
}
