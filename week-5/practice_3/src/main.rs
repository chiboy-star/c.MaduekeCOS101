fn main() {
    let name1 = "Ayomide Adesokan";
    println!("My name is {}",name1);

    //find and replace 
    let name2 = name1.replace("Ayomide","Adebare");
    println!("You cna alos call me {}",name2);
    let faculty = "Faculty of Science and Technology";

    //find and replace
    let school = faculty.replace("Faculty","School");
    println!("I ma a student of the {}",school);
}
