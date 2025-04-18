fn main() {
    // Functions
    let sum = my_function(1, 22);
    println!("The sum is: {sum}");

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    // Funções com Valores de Retorno
    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    //let sum = x + y;
    //return sum;
    x + y
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
