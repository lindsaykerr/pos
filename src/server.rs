pub mod socket;
pub mod connection;
pub mod databases;
pub mod process_query;
pub mod api;

use connection::connection;
use std::net::{SocketAddr, TcpListener};
use api::routing::ApiTree;


pub fn start(socket_addr: SocketAddr) -> Result<(), std::io::ErrorKind> {
   
    // create the api tree, this is used to route the incoming requests
    let mut api_tree = Box::new(ApiTree::new());

    // setup the listener to listen for incoming connections   
    if let Ok(listener) = TcpListener::bind(socket_addr){
      
        for stream in listener.incoming() {
            let stream = stream.unwrap();
        
            connection(stream, &mut api_tree);
        
        }
        Ok(())
    }
    else {
        println!("Error: Server had a problem binding to the ip address on the host, check if the ip and port are available.");
        Err(std::io::ErrorKind::AddrNotAvailable)
    }
}