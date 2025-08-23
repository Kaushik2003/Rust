#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}
struct IpAddress {
    address: String,
    kind: IpAddrKind,
}
impl IpAddress{
    fn new(address: &str)->Self {
        Self{
            address: address.to_string(),
            kind:IpAddrKind::V4,
        }
    }
}

fn route(ip: IpAddress) {
    println!(
        "Routing reuquest or to IP{} of kind {:?}",
        ip.address,
        ip.kind,
    );
}

fn main() {
    let ip = IpAddress::new("1.2.3.4");
    route(ip);
}
