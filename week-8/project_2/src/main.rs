/*
PAU uses a “Student Management Information System” (PAU-SMIS) to manage
student-related data. This system provides facilities for recording and maintaining
personal details of students, maintaining marks scored in assessments and
computing results of students, keeping track of student attendance, managing
many other student-related data. With your high-level programming skills in Rust,
develop a program that reads the personal details of the students from an array or
vector, then displays the details and save into a file in the following format.
*/

use std::fs::File;
use std::io::Write;
use xlsxwriter::*;

fn main() {
    // Define a vector of student details as tuples (Name, Matric Number, Department, Level)
    let students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", "300"),
        ("Adams Aliyu", "ECO10110101", "Economics", "100"),
        ("Shania Bolade", "CSC10328828", "Computer", "200"),
        ("Adekunle Gold", "EEE110202202", "Electrical", "200"),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", "100"),
    ];

    // Create a new Excel workbook
    let workbook = Workbook::new("student_details.xlsx");
    let mut sheet = workbook.add_worksheet(None).expect("Failed to create sheet");

    // Write headers to the Excel file
    sheet.write_string(0, 0, "Student Name", None).unwrap();
    sheet.write_string(0, 1, "Matric Number", None).unwrap();
    sheet.write_string(0, 2, "Department", None).unwrap();
    sheet.write_string(0, 3, "Level", None).unwrap();

    // Write student details to the Excel file
    for (row, student) in students.iter().enumerate() {
        sheet.write_string((row + 1) as u32, 0, student.0, None).unwrap();
        sheet.write_string((row + 1) as u32, 1, student.1, None).unwrap();
        sheet.write_string((row + 1) as u32, 2, student.2, None).unwrap();
        sheet.write_string((row + 1) as u32, 3, student.3, None).unwrap();
    }

    // Save the Excel file
    workbook.close().expect("Failed to save file");

    println!("Student details saved to 'student_details.xlsx'.");
}

