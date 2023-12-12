
pub fn substract_ten(num: u32) -> u32 {
    num - 10
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn substract_ten_test(){
        let x: u32 = 100;
        let y: u32 = substract_ten(x);
        println!("x and y are from test: {}, {}", x, y);
        assert_eq!(y, 90);
    }
}