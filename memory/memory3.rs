// Ownership and Borrowing
fn main() {
    
        // Works
        // let x: i32 = 50;
        // let y: i32 = x;
        
        // println!("{}", x);
        // println!("{}", y);
        
        let s: String = String::from("hello");
        
        //This sentence:
        let t: String = s.clone();
        
        // ...Is similar to this sentence:}
        // let t: String = String::from("hello");
        
        println!("{}", s);
        
        // Works (borrowing == reference)
        let s: String = String::from("hello2");
        let t: &String = &s;
        
        println!("{}", s);
    }