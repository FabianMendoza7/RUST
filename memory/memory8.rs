// Deref coercion on the HEAP
fn main() {
    let mut name: String = String::from("Jhon");
    let name_t: &mut String = &mut name;
    
    // This doesn't work:
    //name_t = String::from("Shaun");
    println!("{}", name_t);  
    
    // To solve, we need to dereference, like this:
    *name_t = String::from("Shaun");
    println!("{}", name_t);  
    println!("{}", name); 
}