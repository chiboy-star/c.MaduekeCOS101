use std::fs::OpenOptions;
use std::io::Write;


fn main() {
    let name = "Madueke Emmanuel Chinemerem\n";
    let mut old_file = std::fs::File::create("data.txt").expect("create failed");
    old_file.write_all(name.as_bytes()).expect("write failed");

    let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
    file.write_all("\nHello Class".as_bytes()).expect("write failed");
    file.write_all("\nThis is the appendage to the document.".as_bytes()).expect("write failed");
    println!("file append success");
}

