
// I think that is more appropriate call it Algebraic Data Type, since it is treated as such.

#[derive(Debug)]
enum IpVersion {
    V4,
    V6
}

#[derive(Debug)]
struct IpAddress {
    version: IpVersion,
    address: String
}

// Only with enums:
enum IpAddrLight {
    IpV4(u8, u8, u8, u8),
    IpV6(String)
}

pub fn test() {
    let four = IpVersion::V4;
    let six = IpVersion::V6;
    let localhost = IpAddress { version: IpVersion::V4, address: String::from("127.0.0.1") };

    let ip4 = IpAddrLight::IpV4(192, 0, 0, 1);
    let ip6 = IpAddrLight::IpV6(String::from("::1"));

    println!("My localhost address is: {:?}", localhost);
}
