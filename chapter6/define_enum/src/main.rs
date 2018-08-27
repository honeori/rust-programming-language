enum IpAddrKind {
    V4,
    V6
}

fn main() {
    use_enum();
    use_enum_attached_data();
    use_enum_with_impl();

    use_option();
}

fn use_enum () {
    let _v4 = IpAddrKind::V4;
    let _v6 = IpAddrKind::V6;
}

fn use_enum_attached_data() {
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let _home = IpAddrKind::V4(127, 0, 0, 1);
    let _loopback = IpAddrKind::V6(String::from("::1"));
}

fn use_enum_with_impl() {
    enum Message {
        Quit,
        Move { x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
        }
    }

    let m = Message::Write(String::from("Hello"));
    m.call();
}

fn use_option() {
    let some_number = Some(5);
    let some_string = Some(String::from("hello"));

    let absent_number: Option<i32> = None;
}
