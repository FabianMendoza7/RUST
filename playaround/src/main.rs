// const OUR_COURSE: &str = "Rust with AutoGPT";

fn main() {
    // println!("Welcome to this course on {}!", OUR_COURSE);

    // // STACK
    // let x: i32;
    // x = 2;
    // println!("x is {}", x);

    // let y: i32 = 4;
    // println!("y is {}", y);

    // // For Loop
    // for i in 0..=y {
    //     if i != 4 {
    //         print!("{}, ", i);
    //         //print!("{}, \n", i);
    //     } else {
    //         println!("{}", i);
    //     }
    // }

    // // Mutation
    // let mut z: i32 = 5;
    // print!("z was {} ", z);
    // z = 10;
    // println!("but is now {}", z);

    // let freezing_temp: f64 = -2.4;
    // println!("freezing_temp is {}", freezing_temp);

    // let is_zero_reminder: bool = 10 % 4 != 0;
    // println!("is_zero_reminder is {}", is_zero_reminder);

    // let my_char: char = 'z';
    // println!("my_char is {}", my_char);

    // let emoji_char: char = '😎';
    // println!("emoji_char is {}", emoji_char);

    // let my_floats: [f32; 5] = [0.0; 5];
    // println!("my_floats is {:?}", my_floats);

    // let my_floats_new: [f32; 5] = my_floats.map(|n: f32| n + 2.0);
    // println!("my_floats_new is {:?}", my_floats_new);

    // Dynamic sized variables.

    // La siguiente variable es estática, se almacena en el stack
    // let name: &str = "Shaun";
    // println!("name is {:?}", name);

    // // Esta variable se almacena en el heap y es dinámica
    // let dynamic_name: String = String::from("Shaun McDonough");
    // println!("dynamic_name is {:?}", dynamic_name);
    // println!("my dynamic_name is stored in memory {:p}", &dynamic_name);

    // // Variable no estática almacenada en el stack:
    // let str_slice: &str = &dynamic_name[0..5];

    
    // //
    // let mut chars: Vec<char> = Vec::new();
    // chars.insert(0, 'h');
    // chars.insert(1, 'e');
    // chars.insert(2, 'l');
    // chars.push('l');
    // chars.push('o');
    // chars.push('.');
    // println!("chars is {:?}", chars);
    // dbg!(&chars);

    // let removed_char: char = chars.pop().unwrap();
    // println!("removed_char is {:?}", removed_char);
    // println!("chars is {:?}", chars);

    // chars.iter().for_each(|c: &char| println!("char is {}", c));
    // chars.iter().for_each(|c: &char| print!("{}", c));

    // let chars_again: Vec<char> = vec!('h','e','l','l','o');
    // dbg!(&chars_again);

    // let collected: String = chars_again.iter().collect();
    // dbg!(collected);

    // for c in chars_again {
    //     print!("{}", c);
    //     if c == 'o' {
    //         println!(", world!");
    //     }
    // }

    // Closures
    let num: i32 = 5;
    let add_num = |x: i32| x + num;
    let new_num: i32 = add_num(7);
    dbg!(new_num);
}
