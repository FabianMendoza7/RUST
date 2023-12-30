// Dangling reference.
// This code doesn't work.
/*
fn make_string_dangle() -> &String {
    let s: String = String::from("dangle");
    // Create a pointer, a reference, that means: borrowing the value of S.
    // It is called dangling reference.
    let r: &String = &s;
    r
}
fn main() {
    // Does not work.
   let r: &String = make_string_dangle();
}
*/

// This code does work.
fn make_string_not_dangle() -> String {
    let s: String = String::from("not dangle");
    s
}
fn main() {
    let r: String = make_string_not_dangle();
    println!("{}", r);
 }