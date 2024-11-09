// Rust program to read the height of a prerson
// and the print if person is tall.,dwarf,
// or average height person 

use std::io;

fn main() {
    let mut input = String::new();

    println!("\nEner you Height (in centimeteres):");
    io::stdin().read_line(&mut input).expect("Not a vaild string");
    let height:f32 = input.trim().parse().expect("Not a vaild number");

    if height >= 150.0 && height <= 170.0{
        println!("You are of average height person");
    }else if height > 170.0 && height <= 195.0{
        println!("You are tall");
    }else if height < 150.0 && height > 100.0{
        println!("You are dwarf");
    }else{
        println!("Abnormal height");
    }
}
