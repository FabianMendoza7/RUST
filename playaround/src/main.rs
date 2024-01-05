const OUR_COURSE: &str = "Rust with AutoGPT";

fn main() {
    println!("Welcome to this course on {}!", OUR_COURSE);

    // STACK
    let x: i32;
    x = 2;
    println!("x is {}", x);

    let y: i32 = 4;
    println!("y is {}", y);

    // For Loop
    for i in 0..=y {
        if i != 4 {
            print!("{}, ", i);
            //print!("{}, \n", i);
        } else {
            println!("{}", i);
        }
    }

    // Mutation
    let mut z: i32 = 5;
    print!("z was {} ", z);
    z = 10;
    println!("but is now {}", z);

    let freezing_temp: f64 = -2.4;
    println!("freezing_temp is {}", freezing_temp);

    let is_zero_reminder: bool = 10 % 4 != 0;
    println!("is_zero_reminder is {}", is_zero_reminder);

    let my_char: char = 'z';
    println!("my_char is {}", my_char);

    let emoji_char: char = '😎';
    println!("emoji_char is {}", emoji_char);

    let my_floats: [f32; 5] = [0.0; 5];
    println!("my_floats is {:?}", my_floats);

    let my_floats_new: [f32; 5] = my_floats.map(|n: f32| n + 2.0);
    println!("my_floats_new is {:?}", my_floats_new);
}
