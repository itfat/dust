enum IpAddrKind {
    V4,
    V6
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: four,
        address: String::from("127.0.0.1");
    }

    let loop_back = IpAddr {
        kind: six,
        address: String::from("::1");
    }
}

// Concise way

enum IpAddr {
    // V4(String),
    V4(u8, u8, u8, u8), // Can assign different types
    V6(String)
}

fn main() {

    // let home = IpAddr::V4(String::from("127.0.0.1"));
    let home = IpAddr::V4(127, 0, 0, 1);
    let loop_back = IpAddr::V6(String::from("::1"));
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
} //is same as decalaring different structs but it is more easier cause all types are grouped

impl Message { // enums can also be implemented like structs

}

struct QuitMessage; 
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); 
struct ChangeColorMessage(i32, i32, i32); 

enum Option<T> { //Defined in standard library - to check if a value is present or absent
    None,
    Some(T)
}

fn main() {
    let x: i8 = 3;
    let y: Option<i8> = Some(3);

    let sum = x + y; //no implementation for `i8 + Option<i8>`
}


