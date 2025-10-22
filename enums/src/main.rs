#![allow(unused)]
fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    //
    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);
    //
    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }

    // enum IpAddr {
    //     V4(String),
    //     V6(String),
    // }
    //
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    //
    // let loopback = IpAddr::V6(String::from("::1"));

    // Better enum with multiple values
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    enum Option<T> {
        None,
        Some(T),
    }
    let some_number = Option::Some(5);
    let some_char = Option::Some('e');

    let absent_number: Option<i32> = Option::None;
}



// fn route(ip_kind: IpAddrKind) {}
