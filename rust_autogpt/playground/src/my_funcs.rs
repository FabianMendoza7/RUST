pub fn add_five(num: u32) -> u32 {
    num + 5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adds_five_test(){
        let x: u32 = 100;
        let y: u32 = add_five(x);
        println!("x and y are from test: {}, {}", x, y);
        assert_eq!(y, 105);
    }
}