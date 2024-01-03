// Deref coercion on the STACK
fn main() {
    let mut x: i32 = 50;
    x = 70;
    dbg!(x);
    
    // This does not work because (?)
    //let y: &mut i32 = &mut x;
    //y += 1;
    //dbg!(y);
    
    // To solve this, we need deref:
    let y: &mut i32 = &mut x;
    *y += 1;
    dbg!(y);
    dbg!(x);
}