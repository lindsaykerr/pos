mod errors;

mod config;
pub mod server;

use server as rest_server;
use server::socket::assign_socket_addr;

use std::io::{ErrorKind};

/*
#[derive(Debug)]
pub struct ApiResponse {
    status: u16,
    success: bool,
    payload: String,
}
impl ApiResponse {
    fn new(status: u16, success: bool, payload: String) -> ApiResponse {
        ApiResponse {
            status,
            success,
            payload,
        }
    }
}
*/


fn main() -> Result<(), ErrorKind>{
    
    // there should be one of two arguments provided, the first is the ip address and the second is the port number
    let args: Vec<_> = std::env::args().collect();

    // assign the ip address and port number to a SocketAddr struct
    let socket_addr = assign_socket_addr(args);
    
    // if there was an error with the ip address and or port number provided then
    // exit the program.
    if let Err(e) = socket_addr {
        print!("Error: {}", e.message());
        return Err(ErrorKind::AddrNotAvailable);
    }
    
    // start the server
    rest_server::start(socket_addr.unwrap())?;
    
    Ok(())
    
}