/*
Develop a Rust program that displays the following menu for
the food items available to take order from the customer.
The program inputs the type of food and quantity. It finally
displays the total charges for the order according to price
criteria. If the total order is greater than N10,000, give a
discount of 5%.
*/

use std::io;

fn main() {
    println!("Menu                             Price
P = Poundo Yam/Edinkaiko Soup - N3,200
F = Fried Rice & Chicken      - N3,000
A = Amala & Ewedu Soup        - N2,500
E = Eba & Egusi Soup          - N2,000
W = White Rice & Stew         - N2,500");

println!("Pls input a letter corresponding to the food item on the menu that you would like:  ");
        let mut meal = String::new();
        io::stdin().read_line(&mut meal).expect("Faild to read input");
        meal = meal.trim().to_lowercase();

println!("Great choice, how many portions of this meal woudl you like: ");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read input");
       let quantity:f64 = input1.trim().parse().expect("Faild to input ");

        println!("{}  {}",meal,quantity);
        let mut total:f64 = 0.0;
if meal == "p"{
         total = quantity * 3200.0;
        println!("so you want {} portion(s) Poundo Yam/Edinkaiko Soup your total will come out to N{}",quantity,total);
}else if meal == "f" {
       total = quantity * 3000.0;
        println!("so you want {} portion(s) Fried Rice & Chicken your total will come out to N{}",quantity,total);
}else if meal == "a"{
        total = quantity * 2500.0;
        println!("so you want {} portion(s) Amala & Ewedu Soup your total will come out to N{}",quantity,total);
}else if meal == "e"{
         total = quantity * 2000.0;
        println!("so you want {} portion(s) Eba & Egusi Soup your total will come out to N{}",quantity,total);
}else if meal == "w"{
         total = quantity * 2500.0;
        println!("so you want {} portion(s) of White Rice & Stew your total will come out to N{}",quantity,total);
}else{
        println!("Error!,try again");
}

if total >= 10000.0{
        let discount = total * 0.05;
        total = total - discount;
        println!("since you total is over 10,000 you have been offered a 5% discount = {} bringing you new total to {}",discount,total);
}else{
        println!("no discount give");
}
}
