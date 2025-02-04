use std::io::Read;
use std::io;
/* 
fn main(){
    let mut file = std::fs::File::open("globacom_dbase.sql");
    let mut contents = String::new();
    file.expect("REASON").read_to_string(&mut contents).unwrap();
    print!("{}",contents)
}
*/
fn main(){
    println!("Hi, Welcome to the Globacom database");
    println!("What is your postion in the company? 
            A - administrator,
            P - project manager,
            E - employee,
            C - customer,
            V - vendor");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let user_postion = input1.trim().to_lowercase();
    if user_postion == "a"{
        println!("Welcome Administrator!");
        administrator()
    }else if user_postion == "p"{
        println!("Welcome Project Manager!");
        manager()
    }else if user_postion == "e"{
        println!("Welcome Employee");
        employee();
    }else if user_postion == "c"{
        println!("Welcome customer");
        customer();
    }else if user_postion == "v"{
        println!("Welcome vendor");
        vendor();
    }else{
        println!("Looks like there was an error in your input, try again!");
    }
}

fn administrator(){
    let mut file = std::fs::File::open("globacom_dbase.sql");
    let mut contents = String::new();
    file.expect("REASON").read_to_string(&mut contents).unwrap();
    print!("{}",contents)
}

fn  manager(){
    let mut file = std::fs::File::open("project_tb.sql");
    let mut contents = String::new();
    file.expect("REASON").read_to_string(&mut contents).unwrap();
    print!("{}",contents)
}

fn  employee(){
    let mut file = std::fs::File::open("staff_tb.sql");
    let mut contents = String::new();
    file.expect("REASON").read_to_string(&mut contents).unwrap();
    print!("{}",contents)
}

fn  customer(){
    let mut file = std::fs::File::open("customer_tb.sql");
    let mut contents = String::new();
    file.expect("REASON").read_to_string(&mut contents).unwrap();
    print!("{}",contents)
}

fn  vendor(){
    let mut file = std::fs::File::open("dataplan_tb.sql");
    let mut contents = String::new();
    file.expect("REASON").read_to_string(&mut contents).unwrap();
    print!("{}",contents)
}



