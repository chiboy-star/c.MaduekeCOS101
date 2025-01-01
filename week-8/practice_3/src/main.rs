use std::fs;
use std::io::Write;
fn main() {
    let name = "Madueke Emmanuel Chinemerem\n";
    let mut old_file = std::fs::File::create("data.txt").expect("create failed");
    old_file.write_all(name.as_bytes()).expect("write failed");

    fs::remove_file("data.txt").expect("could not remove file");
    println!("filed is removed")
}
