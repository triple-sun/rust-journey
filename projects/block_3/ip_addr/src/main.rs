use core::fmt;

enum IpAddr {
    V4(String), 
    V6(String)
}

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            IpAddr::V4(addr) => write!(f, "{}", addr),
            IpAddr::V6(addr) => write!(f, "{}", addr),
        }
    }
}

fn main() {
    let ipv4 = IpAddr::V4(String::from("127.0.0.1"));
    let ipv6 = IpAddr::V6(String::from("fe80::6262:6BFF:FEE2:5757"));

    println!("IPv4 example address is: {ipv4}");
    println!("IPv6 example address is: {ipv6}")

}
