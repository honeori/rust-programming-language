
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    {
        let user1 = User {
            email:String::from("aa@aa.bb.cc"),
            username: String::from("my-user-name"),
            active: true,
            sign_in_count: 1,
        };
        println!("{}", user1.email);
        // update user1
        let user2 = User {
            email: String::from("another@user.mail"),
            ..user1
        };
        println!("{}", user2.email);
    }

    {
        let user = build_user(String::from("aa@bb"), String::from("func_name"));
        println!("{}", user.email);
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count:1,
    }
}
