#[derive(Debug)]
enum CarColour {
    Red,
    Green,
    Blue,
    Silver
}

#[derive(Debug)]
enum GivenResult<T, E>{
    Ok(T),
    Err(E)
}

#[derive(Debug)]
enum GivenOption<f32>{
    None,
    Some(f32)
}

fn create_car_colour_blue() -> CarColour {
    let my_car_colour: CarColour = CarColour::Blue;
    my_car_colour
}

//Antes:
//fn check_under_five(num_check: u8) -> bool {
//     if num_check < 5 {
//         true
//     } else {
//         false
//     }
// }

//Ahora con enums:
fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
    if num_check < 5 {
        GivenResult::Ok(num_check)
    } else {
        GivenResult::Err("Not under 5!".to_string())
    }
}

fn remainder_zero(num_check: f32) -> GivenOption<f32> {
    let remainder: f32 = num_check % 10.0;

    if remainder != 0.0 {
        GivenOption::Some(remainder)
    } else {
        GivenOption::None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_enums(){
        //dbg!("HELLO!");
        let car_colour: CarColour = create_car_colour_blue();
        dbg!(car_colour);

        let under_five_res: GivenResult<u8, String> = check_under_five(2);
        dbg!(under_five_res);

        let under_five_res: GivenResult<u8, String> = check_under_five(7);
        dbg!(under_five_res);

        let remainder = remainder_zero(10.0);
        dbg!(remainder);
    }
}