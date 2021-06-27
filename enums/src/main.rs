mod matches;
mod if_let;

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}


fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: "127.0.0.1".to_string(),
    };

    let home = IpAddr2::V4("127.0.0.1".to_string());
    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6("::1".to_string());

    matches::main();
    if_let::main();
}
