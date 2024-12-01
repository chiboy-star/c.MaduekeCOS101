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
        println!("There was an error when reading your input.");
    }
}

fn number_inputs()->f64{
    let mut number_input =  String::new();
    io::stdin().read_line(&mut number_input).expect("Failed to read input");
    let number:f64 = number_input.trim().parse().expect("Failed to read input");
    
    return number
}

fn trapezium_area()-> f64{
    println!("What is the height of the trapezium: ");
    let height:f64 = number_inputs();
    println!("What is the lenght of the base1: ");
    let base1:f64 = number_inputs();
    println!("What is the lenght of the base2: ");
    let base2:f64 = number_inputs();
    let area_of_trapezium = (height/2.0)*(base1+base2);

    return area_of_trapezium;
}

fn rhombus_area()-> f64{
    println!("What is the lenght of the first diagonal: ");
    let diagonal1:f64 = number_inputs();
    println!("What is the lenght of the seconed diagonal: ");
    let diagonal2:f64 = number_inputs();
    let area_of_the_rhombus = (0.5) * (diagonal1*diagonal2);

    return area_of_the_rhombus;
}

fn parallelogram_area()-> f64{
    //Area of Parallelogram formula = base x altitude
    println!("What is the lenght of the base of the parallelogram: ");
    let base:f64 = number_inputs();
    println!("What is the lenght of the altitude of the parallelogram: ");
    let altitude:f64 = number_inputs();
    let area_of_parallelogram:f64 = base * altitude;

    return area_of_parallelogram;
}

fn cube_area()-> f64{
    //Area of Cube formula = 6 x (length of the side)2
    println!("What is the lenght of the side of the cube: ");
    let length_side:f64 = number_inputs();
    let area_of_cube:f64 = 6.0 *(length_side*length_side);
    return area_of_cube;
}

fn volume_of_cylinder()-> f64{
    println!("What is the radius of the cylinder: ");
    let radius:f64 = number_inputs();
    println!("What is the height of the cylinder: ");
    let height:f64 = number_inputs();
    let volume_of_cylinder:f64 = 3.12*(radius*radius)*height;

    return volume_of_cylinder;
}