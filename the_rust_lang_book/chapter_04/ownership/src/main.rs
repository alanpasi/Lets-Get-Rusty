fn main() {
    {
        let s: &str = "hello";
        println!("{}", s);
    }

    let x = 5;
    let y = x;
    println!("x e y = {}", y);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, world", s1);
    println!("s2 = {}", s2);
}
