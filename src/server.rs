pub mod socket;
pub mod connection;
pub mod databases;
pub mod api;

use connection::connection;
use std::net::{SocketAddr, TcpListener};

pub fn start(socket_addr: SocketAddr) -> Result<(), std::io::ErrorKind> {
    // setup the listener to listen for incoming connections   
    if let Ok(listener) = TcpListener::bind(socket_addr){
      
        for stream in listener.incoming() {
            let stream = stream.unwrap();
        
            connection(stream);
        
        }
        Ok(())
    }
    else {
        println!("Error: Server had a problem binding to the ip address on the host, check if the ip and port are available.");
        Err(std::io::ErrorKind::AddrNotAvailable)
    }
}