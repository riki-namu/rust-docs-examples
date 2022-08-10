use std::io;

fn main() {
    // ###int###
    let bu8: u8 = 4; //8-bit-unsigned
    let b8: i8 = 4; //8-bit-singed

    let bu16: u16 = 94; //16-bit-unsinged
    let b16: i16 = 95; //16-bit-signed

    let bu32: u32 = 94; //32-bit-unsinged
    let b32: i32 = 95; //32-bit-signed

    let bu64: u64 = 94; //64-bit-unsinged
    let b64: i64 = 95; //64-bit-signed

    let bu128: u128 = 94; //128-bit-unsinged
    let b128: i128 = 95; //128-bit-signed

    let arch: usize = 94; //arch-unsinged
    let uarch: isize = 95; //arch-signed

    //###float###
    let f = 2.0; // f64
    let f2: f32 = 3.0; // f32

    //###numeric operations###

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    //###boolean type###
    let t = true;

    let f: bool = false; // with explicit type annotation

    //###character type###
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    //###compound types###
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("five_hundres: {five_hundred}");
    println!("six_point_four: {six_point_four}");
    println!("one: {one}");

    //###array type###
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a = [3; 5]; // a = [3, 3, 3, 3, 3];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("first: {first}");
    println!("second: {second}");

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}
