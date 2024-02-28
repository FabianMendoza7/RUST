#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_structs(){
        let user_1 = User {
            username: String::from("someusername1"),
            email: String::from("somene@example.com"),
            sign_in_count: 1,
            active: true
        };

        dbg!(user_1);
    }
}