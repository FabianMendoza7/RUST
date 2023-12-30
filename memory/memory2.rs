fn main() {
    let s: String = String::from("Hello String");
    let s_2: &str = &s[0..5];
    println!("{}", s_2);

    // Literal strings are stored in stack.
    let msg: &str = "hello2";
    println!("{}", msg);

    let msg_string: String = "hello3".to_string();
    println!("{}", msg_string);   
}