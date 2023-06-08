use std::hash::Hash;
use std::net::TcpStream;
use std::io::{BufReader, prelude::*};
use regex::Regex;
use json::{self, JsonValue, };
use std::time::Duration;
use std::collections::HashMap;
use crate::server::databases::{self, data_structs::Type as DBType};
use crate::server::api::{process_api_query, parsing::query_with_path_variables, query_types::{Query, ContentFormat}};
use crate::server::api::routing::ApiTree;

use super::api::query_types;

pub struct Request {
    method: String,
    path: String,
    http_version: String,
    headers: HashMap<String, String>,
    body: query_types::ContentFormat,
}


pub fn connection(mut stream: TcpStream, api_tree: &mut Box<ApiTree>) {
    

    let some_request = stream_to_request(&mut stream);
    if some_request.is_none() {
        return;
    }
    let the_request = some_request.unwrap();


    let some_query = request_to_api_query(&the_request.path, api_tree);


    // if for whatever reason the request is empty then simply exit the function

    let mut response_content: String = String::new();
    let response_status_line: String; 
    let response_content_type: String;

    match query {
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
            response_content_type = String::from("text/html");
            response_content = String::from("Invalid API uri");
            response_status_line = "HTTP/1.1 400 OK".to_string();
        },
        Some(_) => {
            (response_content, response_content_type, response_status_line) = 
            match the_request.method.as_str() {
                "GET" => {
                    process_get_query(query.unwrap(), the_request);
                },
                "POST" => {
                    process_post_query(query.unwrap(), the_request);
                },
                "PUT" => {
                    process_put_query(query.unwrap(), the_request);
                },
                "DELETE" => {
                    process_delete_query(query.unwrap(), the_request);
                },
                _ => {
                    response_content_type = String::from("text/html");
                    response_content = String::from("Invalid request method");
                    response_status_line = "HTTP/1.1 400 OK".to_string();
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
        body: ContentFormat::None,
    };


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
        return None;
    }

    // First get the start line of the request as it is in a different format to the rest of the header
    let mut header_section: Vec<String> = header_section.split("\r\n").map(|s| s.to_string()).collect();


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
        let header_parts = header.split_once(":").unwrap();
        
        let key = header_parts.0.trim().to_string();
        let value = header_parts.1.trim().to_string();
        
        headers.insert(key, value);
    }
    request_struct.headers = headers;


    if let Some(content_type) = headers.get("Content-Type") {
        match content_type.as_str() {
            "application/json" => {
                let parsed = json::parse(&body_section);
                if let Ok(parsed) = parsed {
                    request_struct.body = ContentFormat::Json(parsed);
                }
                else {
                    request_struct.body = ContentFormat::None;
                    println!("Error: Failed to parse json body");

                }
            },
            _ => {}
        }
    }

    Some(request_struct)

}

pub fn request_to_api_query(uri: &String, api_tree: &mut Box<ApiTree>) -> Option<Query> {

    // get query type


    // split the uri into segments
    let uri_segs: Vec<_> = uri.split("/").into_iter().map(|x| String::from(x)).collect();
    let mut index = 0;
    let mut variables: Vec<String> = Vec::new();

    // if the uri is empty the api is not being accessed, return None
    if uri_segs.len() == 0 {
        return None;
    }

    // the expected first item should be "", so if there is anything else there return None
    if uri_segs.len() == 1 && uri_segs[0].is_empty() {
        return None;
    }

    // increment index to ignore first segment in the rui segment
    index += 1;


    // if the segment is not "api" then the user is not accessing the api, return None
    if !uri_segs[1].eq(&"api".to_string()) {
        return Some(Query::NoneApi)
    }
    // if there is only the "api" segment then return the ApiDoc query
    else if uri_segs.len() == 2 {
        return Some(Query::ApiDoc)
    }

    // increment index to ignore "api" segment
    index += 1;

    let mut tree_seg = &api_tree.tree;

    while tree_seg.has_children() {
    
        // if there is a child segment with the same value as the current uri segment
        // then move to that segment
        if let Some(next) = tree_seg.get_next(uri_segs[index].clone()) {
            //println!("registered valid path segment {} ", uri_segs[index]);
            
            
            if index +1 < uri_segs.len() {  
                index += 1;
                tree_seg = next;
                continue;
            }
            else if let Some(query) = &next.query {
                // request body content will always be the last value to be pushed to the variables vector
                return Some(query_with_path_variables(query, &variables));
            }

            return None;
                
        }
        // if there is a child segment with the value "{}" that indicates a uri parameter
        // then save that value 
        if let Some(next) = tree_seg.get_next("{}".to_string()) {
            
            variables.push(uri_segs[index].clone());
            if index + 1 < uri_segs.len() {
                tree_seg = next;    
                index += 1;
                continue;
            }
            else if let Some(query) = &next.query {
                // request body content will always be the last value to be pushed to the variables vector
                return Some(query_with_path_variables(query, &variables));
            }
            
            return None;
            
        }
        // Knowing there is no child segment, it should also be the case that there
        // is no longer any uri segments left, if there are no more uri segments then 
        if index == uri_segs.len() {
            
            // see if the current segment has a query associated with it
            if let Some(query) = &tree_seg.query {
                // request body content will always be the last value to be pushed to the variables vector
                return Some(query_with_path_variables(query, &variables));
            }
            
            return None;
            
        }  
        // Fall through case, the uri segment is not valid
        return Some(Query::ApiInvalidUri);
                
        
    }


    return None;

    fn uri_seg_decode(uri: &str) -> String {
        let mut uri = uri.to_string();
        let special_regex = Regex::new(r"(%[0-9A-Fa-f]{2})").unwrap();
        for cap in special_regex.captures_iter(&uri.clone()) {
            let special = cap.get(1).unwrap().as_str();
            let special = special.replace("%", "");
            let special = u8::from_str_radix(&special, 16).unwrap();
            let special = special as char;
            uri = uri.replace(cap.get(1).unwrap().as_str(), &special.to_string());
        }
        uri
    }
    
   
}

