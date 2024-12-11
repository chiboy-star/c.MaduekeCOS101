/*
Ernst & Young (EY) Global Limited, is a
multinational professional services network with
headquarters in London and branch offices all
around the world. EY is one of the largest
professional services networks in the world. EY
Nigeria recently launched a project and are
scouting for developers with the highest years of
experience. You have been contacted by the CEO
of EY Nigeria to develop a Rust program to find
the person with the highest years of programming
experience during the job interview using any rust
compound data type.
*/
use std::io;

fn string_input()->String{
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read input");
    let string = input_string.trim().to_string();
    return string;
}
fn number_input()->i32{
    let mut input_number = String::new();
    io::stdin().read_line(&mut input_number).expect("Failed to read input");
    let number = input_number.trim().parse().expect("Invailed input");
    return number;
}
fn main(){
    let mut interview_info : Vec<(String,i32)> = Vec::new();
    println!("How many people came for the interview: ");
    let interview_num = number_input();

    for i in 0..interview_num{
        println!("What is the name of person {}",i);
        let name = string_input();

        println!("How many years work experience does this person have: ");
        let work_exp = number_input();

        interview_info.push((name,work_exp));
    }
    let mut highest_work_exp = 0;
    let mut top_developer = " ";

    for (name,work_exp) in &interview_info{
        if *work_exp > highest_work_exp {
            highest_work_exp = *work_exp;
            top_developer = &name
        }
    }

    println!("\nYou entered:");
    for (name,work_exp) in &interview_info{
        println!("Name: {}, Work experience: {}", name, work_exp);
    }
    println!("The top developer among all these people is\n  Name: {} ---- Work Experience: {}",top_developer,highest_work_exp);
}
