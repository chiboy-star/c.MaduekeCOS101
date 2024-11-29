/*
Your MTH 101 Professor has asked you to develop a Rust program
that performs the following calculations:
Area of Trapezium formula = height/2*(base1+base2)
Area of the rhombus formula = ½ × diagonal1 × diagonal2
Area of Parallelogram formula = base x altitude
Area of Cube formula = 6 x (length of the side)2
Volume of Cylinder formula = π*radius2*height
Using your knowledge of Rust Functions, develop the program that
prompts a user to select an equation, reads inputs and then performs
the corresponding calculations.
*/

use std::io;

fn main() {

    let mut input1 = String::new();
    println!("What kind of calculation would you like to perform \npick a formula based on the number infornt it \nArea of Trapezium formula - 1 \nArea of the rhombus formula - 2 \nParallelogram formula - 3 \nArea of Cube formula - 4 \nVolume of Cylinder formula - 5");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let formula = input1.trim();
    //println!("{}",formula);
    if formula == "1"{
        println!("Area of Trapezium = {}",trapezium_area());
    }else if formula == "2"{
        println!("Area of the rhombus = {}",rhombus_area());

    }else if formula == "3"{
        println!("Area of Parallelogram = {}",parallelogram_area());
    }else if formula == "4"{
        println!("Area of Cube = {}",cube_area());
    }else if formula == "5"{
        println!("Volume of Cylinder = {}",volume_of_cylinder());
    }
    else{

    }

}

fn trapezium_area()-> f64{
    let mut input1a = String::new();
    println!("What is the height of the trapezium: ");
    io::stdin().read_line(&mut input1a).expect("Failed to read input");
    let height:f64 = input1a.trim().parse().expect("Failed to input");

    let mut input1b = String::new();
    println!("What is the lenght of the base1: ");
    io::stdin().read_line(&mut input1b).expect("Failed to read input");
    let base1:f64 = input1b.trim().parse().expect("Failed to input");

    let mut input1c = String::new();
    println!("What is the lenght of the base2: ");
    io::stdin().read_line(&mut input1c).expect("Failed to read input");
    let base2:f64 = input1c.trim().parse().expect("Failed to input");

    let area_of_trapezium = (height/2.0)*(base1+base2);

    return area_of_trapezium;
}

fn rhombus_area()-> f64{
    let mut input2a = String::new();
    println!("What is the lenght of the first diagonal: ");
    io::stdin().read_line(&mut input2a).expect("Failed to read input");
    let diagonal1:f64 = input2a.parse().expect("Failed to input");

    let mut input2b = String::new();
    println!("What is the lenght of the seconed diagonal: ");
    io::stdin().read_line(&mut input2b).expect("Failed to read input");
    let diagonal2:f64 = input2b.trim().parse().expect("Failed to input");

    let area_of_the_rhombus = (0.5) * (diagonal1*diagonal2);

    return area_of_the_rhombus;
}

fn parallelogram_area()-> f64{
    //Area of Parallelogram formula = base x altitude
    let mut input3a = String::new();
    println!("What is the lenght of the base of the parallelogram: ");
    io::stdin().read_line(&mut input3a).expect("Failed to read input");
    let base:f64 = input3a.trim().parse().expect("Failed to input");

    let mut input3b = String::new();
    println!("What is the lenght of the altitude of the parallelogram: ");
    io::stdin().read_line(&mut input3b).expect("Failed to read input");
    let altitude:f64 = input3b.trim().parse().expect("Failed to input");

    let area_of_parallelogram:f64 = base * altitude;

    return area_of_parallelogram;
}

fn cube_area()-> f64{
    //Area of Cube formula = 6 x (length of the side)2
    let mut input4 = String::new();
    println!("What is the lenght of the side of the cube: ");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let length_side:f64 = input4.trim().parse().expect("Failed to input");

    let area_of_cube:f64 = 6.0 *(length_side*length_side);
    return area_of_cube;
}

fn volume_of_cylinder()-> f64{
    //Volume of Cylinder formula = π*radius2*height
    let mut input5a = String::new();
    println!("What is the radius of the cylinder: ");
    io::stdin().read_line(&mut input5a).expect("Failed to read input");
    let radius:f64 = input5a.trim().parse().expect("Failed to input");

    let mut input5b = String::new();
    println!("What is the height of the cylinder: ");
    io::stdin().read_line(&mut input5b).expect("Failed to read input");
    let height:f64 = input5b.trim().parse().expect("Failed to input");

    let volume_of_cylinder:f64 = 3.12*(radius*radius)*height;

    return volume_of_cylinder;
}
