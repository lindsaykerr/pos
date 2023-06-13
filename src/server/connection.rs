
use std::net::TcpStream;
use std::io::{BufReader, prelude::*};
use json;
use std::time::Duration;
use std::collections::HashMap;
use crate::server::api::{uri_to_api_query, query_types::{Query, Content}};
use crate::server::api::routing::ApiTree;
use crate::server::process_query;

use super::api::query_types;

pub struct Request {
    pub method: String,
    pub path: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: query_types::Content,
}


pub fn connection(mut stream: TcpStream, api_tree: &mut Box<ApiTree>) {
    

    let some_request = stream_to_request(&mut stream);
    if some_request.is_none() {
        return;
    }
    let the_request = some_request.unwrap();


    let some_query = uri_to_api_query(&the_request.path, api_tree);


    // if for whatever reason the request is empty then simply exit the function

    let response_content: String;
    let response_status_line: String; 
    let response_content_type: String;


    // Valid and Invalid API URIs will respond in a json content type  
    match some_query {
        Some(Query::ApiDoc) => {
            response_content_type = String::from("text/html");
            response_content = String::from("Api Docs");
            response_status_line = "HTTP/1.1 200 OK".to_string();
        },
        Some(Query::NoneApi) => {
            // TODO: implement some sort of routing for none api requests
            response_content_type = String::from("text/html");
            response_content = String::from("This page does belong to the api");
            response_status_line = "HTTP/1.1 200 OK".to_string();
        },
        Some(Query::ApiInvalidUri) => {
            response_content_type = String::from("application/json");

            let response = json::object!{
                "error" => "Invalid API uri"
            };
            response_content = String::from("Invalid API uri");
            response_status_line = "HTTP/1.1 400 OK".to_string();
        },
        Some(_) => {
            (response_content, response_content_type, response_status_line) = 
            match the_request.method.as_str() {
                "GET" => {
                    process_query::get_request(some_query.unwrap(), the_request)
                },
                "POST" => {
                    process_query::post_request(some_query.unwrap(), the_request)
                },
                "PUT" => {
                    process_query::get_request(some_query.unwrap(), the_request)
                },
                "DELETE" => {
                    process_query::delete_request(some_query.unwrap(), the_request)
                },
                _ => {
                    (String::from("text/html"),
                    String::from("Invalid request method"),
                    "HTTP/1.1 400 OK".to_string())
                }
            }

        },
        None => {
            response_content_type = String::from("text/html");
            response_content = String::from("<p>404 Not Found</p>");
            response_status_line = "HTTP/1.1 404 NOT FOUND".to_string();
        }
        
    }
    
    // create the response
    let content_length = format!("Content-Length: {}", response_content.len());
    let headers = format!("Content-Type: {response_content_type}; charset=UTF-8; {}", content_length);
    let response = format!("{response_status_line}\r\n{headers}\r\n\r\n{response_content}");
 
    // send the response back to the client
    stream.write_all(response.as_bytes()).unwrap();

    
}

fn stream_to_request(stream: &mut TcpStream) -> Option<Request> {

    // create a struct to hold the request information
    let mut request_struct = Request {
        method: String::new(),
        path: String::new(),
        http_version: String::new(),
        headers: HashMap::new(),
        body: Content::None,
    };


    // setting a timeout for the stream is required because there is no EOF for the TcpStream
    stream.set_read_timeout(Some(Duration::from_millis(500))).expect("Timeout failed to set");
    let mut buf_reader = BufReader::new(stream);

    // string to hold the contents of the stream
    let mut buffer_string: String = String::from("");
    if let Err(_) = buf_reader.read_to_string(&mut buffer_string) {
        buffer_string = buffer_string.trim_end().to_string();
    }
    //println!("buffer_string: {}", buffer_string);

    // look for the empty line in an http request and split the request into two parts, the header and the body
    let request_vec = buffer_string.split_once("\n\n");

    // sometimes request will have a body, so split the request into two parts, the header and the body
    // if needed.
    let header_section: String;
    let mut body_section: String = String::new();
    if let None = request_vec {

        header_section = buffer_string.trim().to_string().clone();
    }
    else {
        let request_vec = request_vec.unwrap();
        header_section = request_vec.0.trim().to_string().clone();
        body_section = request_vec.1.trim().to_string().clone();
    }

    if header_section == "" {
        return None;
    }

    // First get the start line of the request as it is in a different format to the rest of the header
    let header_section: Vec<String> = header_section.split("\r\n").map(|s| s.to_string()).collect();


    // get the request method, path and http version from the start line
    let start_line = header_section[0].clone();
  
    let start_line_parts: Vec<&str> = start_line.split(" ").collect();
  
    request_struct.method = start_line_parts[0].trim().to_string();
    request_struct.path = start_line_parts[1].trim().to_string();
    request_struct.http_version = start_line_parts[2].trim().to_string();



    // Create a hashmap which will be used to store the header and body information. The hashmap will be used
    // because the amount of headers is unknown.
    let mut headers = HashMap::new();


    // Then get the rest of the headers and add them to the hashmap
    for header in &header_section[1..] {
        if header == "" {
            break;
        }
        let header_parts = header.split_once(":").unwrap();
        let key = header_parts.0.trim().to_string();
        let value = header_parts.1.trim().to_string();
        
        headers.insert(key, value);
    }
    request_struct.headers = headers;
 


    if let Some(content_type) = request_struct.headers.get("Content-Type") {
        match content_type.as_str() {
            "application/json" => {
                let parsed = json::parse(&body_section);
                if let Ok(parsed) = parsed {
                    request_struct.body = Content::Json(parsed);
                }
                else {
                    request_struct.body = Content::None;
                    println!("Error: Failed to parse json body");

                }
            },
            _ => {}
        }
    }

    Some(request_struct)

}



