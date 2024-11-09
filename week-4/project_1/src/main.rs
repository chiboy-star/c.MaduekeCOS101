//Given the values of a, b and c, find the roots of a quadratic equation using Rust program.
 
use std::io;

fn main() {
    //first get input for user
    //input for a
    println!("Enter the value of a: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let  a:f32 = input1.trim().parse().expect("Faild to input");
    //input for b
    println!("Enter the value of b: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let  b:f32 = input2.trim().parse().expect("Faild to input");
    //input for c
    println!("Enter the value of c: ");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let  c:f32 = input3.trim().parse().expect("Failed to input");
    //Find the discriminant 
    //let powerof2:f32 = 2.0;
    let  d:f32 = b * b  - (4.0 * a * c);
    //println!("{}",d)//check the value of d
    if d > 0.0 {
        println!("since the vaule of the discriminat  = {} is postive, then there are 2 distinct root which are given below as X1 and X2",d);
        let x1 = (-1.0*(b)+(d.sqrt())) / 2.0 * a;
        let x2 = (-1.0*(b)-(d.sqrt())) / 2.0 * a;
        println!("for X1={} and for X2={}",x1,x2);
    }else if d == 0.0 {
        println!("since the vaule of the discriminat  = {} is zero,then there is exactly one real root, which is given below as X1",d);
        let x1 = (-1.0*(b)) / 2.0 * a;
        println!("for X1={}",x1);
    }else if d < 0.0{
        println!("since the vaule of the discriminat  = {} is negative, then there are no real roots, which is given below as X1 and X2",d);
        let x1 = (-1.0*(b) / 2.0 * a) + (d.sqrt() / 2.0 * a);
        let x2 = (-1.0*(b) / 2.0 * a) - (d.sqrt() / 2.0 * a);
    }else {
        println!("Theres a problem with your input try again :)")
    }
}
