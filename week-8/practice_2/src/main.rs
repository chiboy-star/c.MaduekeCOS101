use std::io::Read;
use std::io::Write;

fn main() {

    let name = "Madueke Emmanuel Chinemerem\n";
    let age  = "He is 18 years of age\n";
    let school  = "He attends PAU\n";

    let mut old_file = std::fs::File::create("welcome_text.txt").expect("create failed");
    old_file.write_all(name.as_bytes()).expect("write failed");
    old_file.write_all(age.as_bytes()).expect("write failed");
    old_file.write_all(school.as_bytes()).expect("write failed");



    let mut file = std::fs::File::open("welcome_text.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}
