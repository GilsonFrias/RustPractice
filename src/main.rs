fn main() {
    let mut x:u8 = 5;
    let mut _string:u8 = "32".parse().expect("Not a number!");
    println!("The value of _string is: {_string}");
    x = 6;
    println!("The value of x is: {x}");
    let y:u8 = b'A';
    println!("The value of y is: {y}");
    let z:u16 = 0b1111_0000;
    println!("The value of y is: {z}");
    let heartEyedCat:char = '😻';
    println!("The value of heartEyedCat is: {heartEyedCat}");
    let tuple: (u8, char, bool) = (32, 'F', false);
    let arr: [i8; 1] = [-9];
    let first = arr[0];
    println!("First element in array arr is: {first}");
}
