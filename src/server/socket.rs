use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::str::FromStr;
use crate::errors::ErrorType;
use crate::config::{DEFAULT_IP, DEFAULT_PORT};

pub fn assign_socket_addr(args: Vec<String>) -> Result<SocketAddr, ErrorType> {
    
    
    let port: u16 = if args.len() > 2 {
        parse_port(args[2].clone())?
    } else {
        DEFAULT_PORT.clone()
    };

    let ipv4: Ipv4Addr = if args.len() > 1 { 
        parse_ip(args[1].clone())?
    } else {
        Ipv4Addr::from_str(DEFAULT_IP).unwrap()
    };
    

    Ok(SocketAddr::new(IpAddr::V4(ipv4), port))
 
}

// parses port number from string and creates a valid numeric port number
fn parse_port(port: String) -> Result<u16, ErrorType> {
    if let Ok(value) = port.parse::<u16>() {
        if value > 0 && value < 65535 { 
            Ok(value)
        } else {
            Err(ErrorType::InvalidPort("Port number must be between 1 and 65535".to_string()))
        }
    } else {
        Err(ErrorType::ParseError("Port number was not a number".to_string()))
    }
}

// parses ip4 address from string and creates a Ipv4Addr struct
fn parse_ip(ip: String) -> Result<Ipv4Addr, ErrorType> {
    if let Ok(value) = Ipv4Addr::from_str(&ip) {
        Ok(value)
    } else {
        Err(ErrorType::InvalidIp("Ip address was not valid".to_string()))
    }
}