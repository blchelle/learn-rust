// Defining an enum type
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// Uses an enum inside of a struct
// OPTION 1
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

impl IpAddr {
    fn log_address(&self) {
        println!("IP Address: {:?}", self);
    }
}

// An enum with associated string values
// OPTION 2
#[derive(Debug)]
enum IpAddrEnum {
    V4(String),
    V6(String),
}

// An excellent reason to sometimes use OPTION 2
#[derive(Debug)]
enum IpAddrEnum2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Enums can also have methods
impl IpAddrEnum2 {
    fn log_ip(&self) {
        println!("IP Address: {:?}", self);
    }
}

fn main() {
    // Using the enum above
    let version_four = IpAddrKind::V4;
    let version_six = IpAddrKind::V6;

    // Logs the versions
    log_ip(&version_four);
    log_ip(&version_six);

    // Uses the IpAddr struct
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    home.log_address();

    // Using an enum with associated data
    let random_addr = IpAddrEnum::V4(String::from("101.101.101.101"));
    let loop_back = IpAddrEnum::V6(String::from("::1"));
    println!("IP Address: {:?}", random_addr);
    println!("IP Address: {:?}", loop_back);

    // Using the enum where each value has different structures
    let brocks_ip = IpAddrEnum2::V4(101, 101, 202, 111);
    let joes_ip = IpAddrEnum2::V6(String::from("::6"));
    brocks_ip.log_ip();
    joes_ip.log_ip();
}

fn log_ip(ip_kind: &IpAddrKind) {
    println!("IP Version: {:?}", ip_kind);
}
