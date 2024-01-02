fn main() {
    // You can only have one OWNER assigned to a value at a time.
    // That's why following line does not work:
    /*
    let s: String::from("does not work");
    let t: String = s;
    println!("{}", s);
    */
    
    // To solve that we have to borrow s:
    // we must to point to the value of s (than means reference it),
    // which was store on the heap:
    /*
    let s: String = String::from("does work");
    let t: &String = &s;
    println!("{}", s);
    println!("{}", t);
    */
    
    // lets do s a mutable variable.
    // Works:
    /*
    let mut s: String = String::from("does work");
    let t: &String = &s;
    s.push('?');
    println!("{}", s);
    */
   
   // but if I try to do this, doesn't work because t is inmutable,
   // snd it cannnot reference a mutable variable:
   //println!("{}", t); 
   
   // To solve that, we have to do this:
   let mut s: String = String::from("does work");
   let t: &mut String = &mut s;
   t.push('?');
   println!("{}", t);   
}