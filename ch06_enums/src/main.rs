#[derive(Debug)]
enum IPKind {
    V4(String), V6(String)
}

struct IP {
    kind: IPKind,
    addr: String
}

fn main() {
    let four = IPKind::V4(String::from("127.0.0.1"));
    let six = IPKind::V6(String::from("127.0.0.1"));
    println!("{:?}", four);

    let x: Option<u8> = None;
    let y: Option<u8> = Some(8);
    let sum = x.unwrap_or(5) + y.unwrap();
    println!("{}", sum);
}
