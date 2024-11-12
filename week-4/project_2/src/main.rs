//Develop a program in Rust that takes as input the experience and age of an employee to determine the annual incentive

use std::io;

fn main() {
    //ask the user for his/her experience 
    println!("Are you an experience  employee(y/n): ");
    let mut  employee_experience = String::new();
    io::stdin().read_line(&mut employee_experience).expect("Failed to read input");
    let employee_experience = employee_experience.trim().to_lowercase();

    println!("How old are you: ");
    let mut input2 =String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let age:i32 = input2.trim().parse().expect("Failed to input");

   // println!("this is age {} this is ex{}",age,employee_experience);
   if age >= 40 && employee_experience == "y"{
    let annual_incentive:f64 = 1560000.00 * 12.00;
    println!("Since this employee is {}years old and is experience there for his/her Annual Incentive is {}",age,annual_incentive)
   }else if age >= 30 && age < 40 && employee_experience == "y" {
    let annual_incentive:f64 = 1480000.00 * 12.00;
    println!("Since this employee is {}years old and is experience there for his/her Annual Incentive is {}",age,annual_incentive)
   }else if age < 28 {
    let annual_incentive:f64 = 1300000.00 * 12.00;
    println!("Since this employee is {}years old and is experience there for his/her Annual Incentive is {}",age,annual_incentive)
   }else if employee_experience =="n"{
    let annual_incentive:f64 = 100000.00 * 12.00;
    println!("Since this employee is {}years old and is experience there for his/her Annual Incentive is {}",age,annual_incentive)
   }else {
    println!("Your input is not vaild");
   }
}
