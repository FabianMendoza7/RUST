#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

impl User {
    fn increment_signin_count(&mut self) {
        self.sign_in_count += 1;
    }

    fn change_email(&mut self, new_email: &str) {
        self.email = String::from(new_email);
    }
}

fn change_username(user: &mut User, new_username: &str){
    user.username = String::from(new_username);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_structs(){
        let mut user_1 = User {
            username: String::from("someusername1"),
            email: String::from("somene@example.com"),
            sign_in_count: 1,
            active: true
        };

        change_username(&mut user_1, "somenewusername");

        dbg!(user_1);

        let mut user_2 = User {
            username: String::from("someusername2"),
            email: String::from("somene2@example2.com"),
            sign_in_count: 0,
            active: false
        };

        user_2.increment_signin_count();

        user_2.change_email("another@email.com");

        dbg!(user_2);
    }
}