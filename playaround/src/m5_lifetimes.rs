#[allow(dead_code, unused_variables)]
fn example_0(){
    let r: &i32;

    //solution: remove the brackets.
    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}

#[allow(dead_code, unused_variables)]
fn example_1() {
    // Allocate space in memory.
    let highest_age: i32;

    // Initialize vars.
    let alice_age: i32 = 20;
    let bob_age: i32 = 21;

    // Call function.
    highest_age = largest(&alice_age, &bob_age);

    // Print output.
    println!("Highest age is {}", highest_age);

    fn largest(compare_1: &i32, compare_2: &i32) -> i32 {
        if compare_1 > compare_2 {
            *compare_1
        } else {
            *compare_2
        }
    }
}

#[allow(dead_code, unused_variables)]
fn example_2() {
    // Allocate space in memory.
    let highest_age: i32;

    // Initialize vars.
    let alice_age: i32 = 20;
    let bob_age: i32 = 21;

    // Call function.
    highest_age = largest(&alice_age, &bob_age);

    // Print output.
    println!("Highest age is {}", highest_age);

    fn largest(compare_1: &i32, compare_2: &i32) -> i32 {
        if compare_1 > compare_2 {
            *compare_1
        } else {
            *compare_2
        }
    }
}
