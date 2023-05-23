use std::net::TcpStream;
use std::io::{BufReader, prelude::*};
use crate::server::databases::{self, data_structs::Type as DBType};
use crate::server::api::{api, Query};

pub fn connection(mut stream: TcpStream) {
    

    let http_request = stream_to_request_vec(&mut stream);
    // if for whatever reason the request is empty then simply exit the function
    if http_request.len() == 0 {
        return;
    }



    // the first line of the http request holds the request line
    let request_line: String = http_request[0].clone();

    let query = query_from_api_routing(request_line);
    
    let status_line: String; 
    let content: String; 
    let content_type: String; 
    if let None = query {
        content_type = String::from("text/html");
        content = String::from("<p>404 Not Found</p>");
        status_line = "HTTP/1.1 404 NOT FOUND".to_string();
    } else {
        let query = query.unwrap();
        let query_to_match = query.clone();
        match query_to_match {
            Query::ApiDoc => {
                content_type = String::from("text/html");
       
                content = String::from("Api Docs");
                status_line = "HTTP/1.1 200 OK".to_string();
          
          
            },
            _ =>  {

                match databases::process_query(query, DBType::Sqlite) {
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
            }
        }
    };
    
    // create the response
    let content_length = format!("Content-Length: {}", content.len());
    let headers = format!("Content-Type: {content_type}; charset=UTF-8; {}", content_length);
    let response = format!("{status_line}\r\n{headers}\r\n\r\n{content}");
 
    // send the response back to the client
    stream.write_all(response.as_bytes()).unwrap();

    
}

fn stream_to_request_vec(stream: &mut TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(stream);
    let request_vec: Vec<_> = buf_reader.
        lines().
        map(|result| {
            if let Ok(result) = result {
                result
            } else {
                String::from("")
            }
        }).
        take_while(|line| !line.is_empty()).
        collect();

    request_vec 

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


