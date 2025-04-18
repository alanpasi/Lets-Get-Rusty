#![allow(unused)]
use std::io;

fn main() {
    // Aqui é necessário a anotação do tipo 'u32'
    let guess: u32 = "42".parse().expect("Não é um número!");

    // Tipos escalares:
    // - inteiro
    // - ponto flutuante
    // - boleano
    // - caracter

    // Integers Types
    let a = 98_222; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; //Byte (u8 only)

    let f: u8 = 255;

    let num_arch: isize = 255; // 'isize' Depende da arquitetura do computador: 64bits (8 bytes) ou 32 bits (4 bytes)

    // Floating-point numbers
    let g: f64 = 2.0;
    let h: f32 = 3.0;

    // Operações Numéricas
    // adição
    let sum = 5 + 10;
    // subtração
    let difference = 95.5 - 4.3;
    // multiplicação
    let product = 4 * 30;
    // divisão
    let quotient = 56.7 / 32.2;
    // resto
    let remainder = 43 % 5;

    // Booleans
    let i = true;
    let j = false;

    // Characters
    let k = 'z';
    let l = 'ℤ';
    let heart_eyed_cat = '😻';

    // Compound Types
    // Tipos compostos primitivos: tuplas e arrays

    // The Tuple Type
    let tup: (&str, i32) = ("Let's Get Rusty", 100_000);
    let (channel, sub_count) = tup;
    println!("channel: {}\nsub_count: {}", channel, sub_count);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("Tuplas:\nO valor de 'y' = {y}");
    println!("Outra maneira de acessar uma tupla 'tup.1' => {}", tup.1);

    // Arrays
    let a = [1, 2, 3, 4, 5];
    let months = ["janeiro", "fevereiro", "março", "abri"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    println!("let a = [3; 5] => {:?}", a);
}
