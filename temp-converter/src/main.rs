use std::io;

fn main() {
    println!("Input celcius temperature:");
    let mut grades = String::new();

    io::stdin()
        .read_line(&mut grades)
        .expect("Failed to read grades");

    let grades:u32 = grades
        .trim()
        .parse()
        .expect("Grades entered was not a number");

    let result = (grades*9/5)+32;

    println!("Fahrenheit temperature: {result}");
}