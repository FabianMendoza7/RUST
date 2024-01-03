fn main() {
    let s: String = String::from("hello");
    let t: &String = &s;
    
    // Print de value of t.
    println!("{}", t);
    println!("{}", *t); // this called deref cohercion.
    
    // print de address of t.
    println!("{:p}", t);   
}