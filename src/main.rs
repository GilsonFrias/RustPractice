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
    let heart_eyed_cat:char = '😻';
    println!("The value of heartEyedCat is: {heart_eyed_cat}");
    let _: (u8, char, bool) = (32, 'F', false);
    let arr: [i8; 1] = [-9];
    let first = arr[0];
    println!("First element in array arr is: {first}");
    let k = helper_function(100);
    println!("{k}");
    let arg = if k%10!=0 {10} else {16};
    compare(arg);
}

fn helper_function(x: u8) -> u8{
    println!("The passed arg is: {x}");
    {
        return 5
    }
}

fn compare(y: u8){
    const TH:u8 = 16;
    println!("The input param to compare is: {y}");
    if y>TH {
        println!("The value of y is above TH");
    } else if y==TH {
        println!("The value of y is equal to TH");
    } else {
        println!("The value of y is below TH");
    }
}
