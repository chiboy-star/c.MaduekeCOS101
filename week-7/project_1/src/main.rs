/*
You have been invited to join the team of developers to build a Public Service APS level
checker for the Federal Government of Nigeria. You have been provided the following table
below.
Develop a Rust program using vectors to validate Staff level. E.g. If Staff is an Associate
Lawyer, and has 5-8 years of work experience, then the staff holds position APS 5-8.
Public Servant Office Administrator Academic Lawyer Teacher
APS 1-2 Intern – Paralegal Placement
APS 3-5 Administrator Research Assistant Junior Associate Classroom Teacher
APS 5-8
Senior
Administrator PhD Candidate Associate Snr Teacher
EL1 8-10 Office Manager Post-Doc
Researcher
Senior Associate 1-
2
Leading Teacher
EL2 10-13 Director Senior Lecturer Senior Associate 3-
4
Deputy Principal
SES CEO Dean Partner Principal
*/

use std::io;

fn string_input()->String{
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read input");
    let string = input_string.trim().to_string();
    return string;
}

fn main() {
    let mut input1 = String::new();
    println!("How many years of work experience do you have: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let work_exp:u32 = input1.trim().parse().expect("Failed to input");

    if work_exp > 0 && work_exp <= 2{
        aps_1();
    }else if work_exp >= 3 && work_exp <= 5{
        aps_3();
    }else if work_exp >= 5 && work_exp <= 8{
        aps_5();
    }else if work_exp >= 8 && work_exp <= 10{
        el1_8();
    }else if work_exp >= 10 && work_exp <= 13{
        el2_10();
    }else if work_exp >= 14{
        ses();
    }else{
        println!("something is wrong with you input try again");
    }

fn aps_1(){
    println!("Are you an Intern,Paralegal or Placement: ");
    let mut staff_post = string_input();
    staff_post = staff_post.to_lowercase();
    let aps1 = vec!["Intern","Paralegal","Placement"];
    for ref mut i in 0..aps1.len(){
        if staff_post == aps1[*i]{
            println!("This staff holds position APS 1-3, as (a/an){}.",staff_post);
        }else{
            *i = *i + 1;
        }
    return;
    }

}

fn aps_3(){
    println!("Are you an Administrator,Research Assistant,Junior Associate or Classroom Teacher: ");
    let mut staff_post = string_input();
    staff_post = staff_post.to_lowercase();
    let aps3 = vec!["administrator","research assistant","junior associate","classroom teacher"];
    for ref mut i in 0..aps3.len(){
        if staff_post == aps3[*i]{
            println!("This staff holds position APS 3-5, as (a/an){}.",staff_post);
        }else{
            *i = *i + 1;
        }
    return;
    }
}

fn aps_5(){
    println!("Are you a senior administrator,phd candidate,associate,snr teacher: ");
    let mut staff_post = string_input();
    staff_post = staff_post.to_lowercase();
    let aps5 = vec!["senior administrator","phd candidate","associate","snr teacher"];
    for ref mut i in 0..aps5.len(){
        if staff_post == aps5[*i]{
            println!("This staff holds position APS 5-8, as (a/an){}.",staff_post);
        }else{
            *i = *i + 1;
        }
    return;
    }
}

fn el1_8(){
    println!("Are you a office manager,post-doc researcher,senior associate 1-2,leading teacher: ");
    let mut staff_post = string_input();
    staff_post = staff_post.to_lowercase();
    let el18 = vec!["office manager","post-doc researcher","senior associate 1-2","leading teacher"];
    for ref mut i in 0.. el18.len(){
        if staff_post ==  el18[*i]{
            println!("This staff holds position EL1 8-10, as (a/an){}.",staff_post);
        }else{
            *i = *i + 1;
        }
    return;
    }
}

fn el2_10(){
    println!("Are you a director,senior lecturer,senior associate 3-4,deputy principal: ");
    let mut staff_post = string_input();
    staff_post = staff_post.to_lowercase();
    let el210 = vec!["director","senior lecturer","senior associate 3-4","deputy principal"];
    for ref mut i in 0.. el210.len(){
        if staff_post ==  el210[*i]{
            println!("This staff holds position EL2 10-13, as (a/an){}.",staff_post);
        }else{
            *i = *i + 1;
        }
    return;
    }
}

fn ses(){
    println!("Are you a ceo,dean,partner,principal: ");
    let mut staff_post = string_input();
    staff_post = staff_post.to_lowercase();
    let ses = vec!["ceo","dean","partner","principal"];
    for ref mut i in 0..ses.len(){
        if staff_post == ses[*i]{
            println!("This staff holds position SES, as (a/an){}.",staff_post);
        }else{
            *i = *i + 1;
        }
    return;
    }
}
}
