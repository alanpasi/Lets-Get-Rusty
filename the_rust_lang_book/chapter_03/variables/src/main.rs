/* '_'  to disable 'unused variable' warnings */
#![allow(unused)]

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const SUBSCRIBER_COUNT: u32 = 100_000;

    /* Integers */
    let a = 98_222; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; //Byte (u8 only)

    let f: u8 = 255;

    /* Floating-point numbers */
    let g: f64 = 2.0;
    let h: f32 = 3.0;

    /* Booleans */
    let i = true;
    let j = false;

    /* Characters */
    let k = 'z';
    let l = 'ℤ';
    let heart_eyed_cat = '😻';

    /* Compound Types */
    let tup: (&str, i32) = ("Let's Get Rusty", 100_000);
    let (channel, sub_count) = tup;
    println!("channel: {}\nsub_count: {}", channel, sub_count);

    let sum = my_function(1, 22);
    println!("The sum is: {sum}");
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    //let sum = x + y;
    //return sum;
    x + y
}
