use std::net::TcpStream;
use std::io::{BufReader, prelude::*};
use std::string;
use json::{self, JsonValue, };
use std::time::Duration;
use std::collections::HashMap;
use crate::server::databases::{self, data_structs::Type as DBType};
use crate::server::api::{api, query_types::Query};

pub fn connection(mut stream: TcpStream) {
    

    let http_request = stream_to_request_vec(&mut stream);
    // if for whatever reason the request is empty then simply exit the function

    let query = http_request.0;
    let mut content = http_request.1;
    let status_line: String; 
    let content_type: String;

    match query {
        Some(Query::ApiDoc) => {
            content_type = String::from("text/html");
            content = String::from("Api Docs");
            status_line = "HTTP/1.1 200 OK".to_string();
        },
        Some(Query::NoneApi) => {
            // TODO: implement some sort of routing for none api requests
            content_type = String::from("text/html");
            content = String::from("This page does belong to the api");
            status_line = "HTTP/1.1 200 OK".to_string();
        },
        Some(Query::ApiInvalidUri) => {
            content_type = String::from("text/html");
            content = String::from("Invalid API uri");
            status_line = "HTTP/1.1 400 OK".to_string();
        },
        Some(_) => {
            let query = query.unwrap();
            match databases::process_query(query, Some(String::new()), DBType::Sqlite) {
                Ok(content_response) => {
                    content = content_response;
                    content_type = String::from("application/json");
                    status_line = "HTTP/1.1 200 OK".to_string();
                },
                Err(e) => {
                    println!("Error: {}", e.message());
                    content_type = String::from("text/html");
                    content = String::from("<p>Internal Server Error</p>");
                    status_line = "HTTP/1.1 500 INTERNAL SERVER ERROR".to_string();
                }
            } 
        },
        None => {
            content_type = String::from("text/html");
            content = String::from("<p>404 Not Found</p>");
            status_line = "HTTP/1.1 404 NOT FOUND".to_string();
        }
        
    }
    
    // create the response
    let content_length = format!("Content-Length: {}", content.len());
    let headers = format!("Content-Type: {content_type}; charset=UTF-8; {}", content_length);
    let response = format!("{status_line}\r\n{headers}\r\n\r\n{content}");
 
    // send the response back to the client
    stream.write_all(response.as_bytes()).unwrap();

    
}

fn stream_to_request_vec(stream: &mut TcpStream) -> (Option<Query>, String) {
    
    // setting a timeout for the stream is required because there is no EOF for the TcpStream
    stream.set_read_timeout(Some(Duration::from_millis(500))).expect("Timeout failed to set");
    let mut buf_reader = BufReader::new(stream);

    // string to hold the contents of the stream
    let mut buffer_string: String = String::from("");
    buf_reader.read_to_string(&mut buffer_string).expect("Error reading from stream");

    // look for the empty line in an http request and split the request into two parts, the header and the body
    let request_vec = buffer_string.split_once("\r\n").unwrap();
    let header_section = request_vec.0.trim().to_string().clone();
    let mut body_section = request_vec.1.trim().to_string().clone();

    if header_section == "" {
        return (None, "".to_string());
    }

    // Create a hashmap which will be used to store the header and body information. The hashmap will be used
    // because the amount of headers is unknown.
    let mut request_map = HashMap::new();

    // First get the start line of the request as it is in a different format to the rest of the header
    let mut header_section: Vec<String> = header_section.split("\r\n").map(|s| s.to_string()).collect();
    request_map.insert("start line", header_section[0].clone());

    // Then get the rest of the headers and add them to the hashmap
    for header in &header_section[1..] {
        let header_parts = header.split_once(":").unwrap();
        
        let key = header_parts.0.trim();
        let value = header_parts.1.trim().to_string();
        
        request_map.insert(key, value);
    }


    // get the request method, path and http version from the start line
    let start_line = request_map.get("start line").unwrap();
    let start_line_parts: Vec<&str> = start_line.split(" ").collect();
    let method = start_line_parts[0].trim();

    // get query type
    let query = query_from_api_routing(request_map.get("start line").unwrap().clone().to_string());

    if method == "PUT" {
        let ctype = request_map.get("Content-Type");
        if ctype == Some(&String::from("application/json")) {
            if let Err(e) = json::parse(&body_section) {
                body_section = "".to_string();
            }
        }
    }

    let content = body_section;

    return (query, content);

  

    // - lines() splits the input into a new line when it comes across a newline byte
    // and returns a collection of lines as an iterator. Each line will be in the form
    // of a Result<String, std::io::Error>
    // - map() is used to unwrap the contents of each item of the iterator
    // - take_while() passes on the line from the iterator until there is an empty line 
    // then the condition is false
    // - collect() collects the lines from the iterator into http_request
}

fn query_from_api_routing(request_line: String) -> Option<Query>  {
    let method_path_regex = regex::Regex::new(r"(GET|PUT|POST|DELETE) +(/.*) +HTTP").unwrap();
    let mut method: String = String::new();
    let mut path: String = String::new();

    for capture in method_path_regex.captures_iter(request_line.as_str()) {
        method = capture[1].to_string();
        path = capture[2].to_string();
        break;
    }

    api(method, path)
}


