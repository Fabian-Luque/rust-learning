enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

pub fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
    let _home = IpAddrKind::V4(127, 0, 0, 1);
    let _loopback = IpAddrKind::V6(String::from("::1"));
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('a');
    let absent_number: Option<i32> = None;

    // route(four);
    // route(six);
}

fn route(ip_kind: IpAddrKind) {}
