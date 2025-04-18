#![allow(unused)] /* '_'  to disable 'unused variable' warnings */

fn main() {
    /* Variables and Mutability */
    let mut x = 5;
    println!("Variables and Mutability:\nThe value of x is: {}", x);
    x = 6;
    println!("Now the value of x is: {}", x);

    /* Constants */
    const SUBSCRIBER_COUNT: u32 = 100_000;
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    /* Shadowing */
    let x = 5;
    println!("Shadowing:\nThe value of x is: {}", x);
    let x = x + 1;
    println!("Now the value of 'let x = x + 1' is: {}", x);
    let x = x * 2;
    println!("Now the value of 'let x = x * 2' is: {x}"); // Outra forma de imprimir o 'x'

    let spaces = "     "; // String
    let spaces = spaces.len(); // Agora é um número
    println!("spaces: {spaces}");
}
