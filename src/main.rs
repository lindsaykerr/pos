

use std::io::{BufReader, ErrorKind, prelude::*};
use std::net::{TcpListener, TcpStream, IpAddr, Ipv4Addr, SocketAddr};
use std::println;
use sqlite::{State, Statement, Connection};
use regex;
use json::{self, JsonValue};

use std::str::FromStr;

const DEFAULT_IP: &'static str = "127.0.0.1";
const DEFAULT_PORT: &'static u16 = &7878;
const SQLITE_DB_PATH: &'static str = "./stock_test.db";

#[derive(Debug)]
pub enum ErrorType {
    InvalidPort(String),
    InvalidIp(String),
    ParseError(String),
    NotImplemented(String),
}
impl ErrorType {
    fn message(&self)-> &String {
        match *self {
            ErrorType::InvalidPort(ref s) => s,
            ErrorType::InvalidIp(ref s) => s,
            ErrorType::ParseError(ref s) => s,
            ErrorType::NotImplemented(ref s) => s,
        }
    }
}

pub enum DatabaseError {
    ConnectionError(String),
    QueryError(String),
    NotImplemented(String),
}
impl DatabaseError {
    fn message(&self)-> &String {
        match *self {
            DatabaseError::ConnectionError(ref s) => s,
            DatabaseError::QueryError(ref s) => s,
            DatabaseError::NotImplemented(ref s) => s,
        }
    }
}

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


fn main() {
    
    // there should be one of two arguments provided, the first is the ip address and the second is the port number
    let args: Vec<_> = std::env::args().collect();

    // assign the ip address and port number to a SocketAddr struct
    let socket_addr = assign_socket_addr(args);
    
    // if there was an error with the ip address and or port number provided then
    // exit the program.
    if let Err(e) = socket_addr {
        print!("Error: {}", e.message());
        return;
    }
        
  
    // setup the listener to listen for incoming connections   
    if let Ok(listener) = TcpListener::bind(socket_addr.unwrap()){
      
        for stream in listener.incoming() {
            let stream = stream.unwrap();
        
            handle_connection(stream);
        
        }
    }
    else {
        println!("Error: Server had a problem binding to the ip address on the host, check if the ip and port are available.");
        return;
    }
}

// This function ensures that valid SocketAddr is returned.
// If any command line arguments are found, then the first argument corresponds to the ip address and optionally the 
// second relates to the port number. If there are no arguments then the default ip address and port number are used.

fn assign_socket_addr(args: Vec<String>) -> Result<SocketAddr, ErrorType> {
    
    
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


fn handle_connection(mut stream: TcpStream) {
    

    let http_request = stream_to_request_vec(&mut stream);
    // if for whatever reason the request is empty then simply exit the function
    if http_request.len() == 0 {
        return;
    }



    // the first line of the http request holds the request line
    let request_line: String = http_request[0].clone();

    let query = query_from_api_routing(request_line);
    
    let mut status_line = String::new();
    let mut content = String::from("");
    let mut content_type = String::from("application/json");
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

                match process_query(query, DatabaseType::Sqlite) {
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
        map(|result| result.unwrap()).
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

    // API routing goes here
    match method.as_str() {
        "GET" => {
            match path.as_str() {
                "/" | "/api" => Some(Query::ApiDoc),
                "/api/suppliers" => Some(Query::GETStockSuppliers), 
                _ => {
                    if path.starts_with("/api/supplier") {
                        let regex_id = regex::Regex::new(r"/api/supplier/(/\d+)(/[A-z\-_]*)?").unwrap();
                        for capture in regex_id.captures_iter(path.as_str()) {
                            if capture.len() == 2 {
                                return Some(Query::GETStockSupplierId(capture[1].parse::<u64>().unwrap()));
                            } 
                            else if capture.len() == 3 {
                                match &capture[2] {
                                    "/name" => return Some(Query::GETStockSupplierName),
                                    _ => return None,
                                }
                            } else {
                                return None;
                            }
                        } 
                    }
                    return None;
                },
         
            }
        },
        _ => None,
    }
    
}


pub enum Query {
    GETStockSuppliers,
    GETStockSupplierName,
    GETStockSupplierId(u64),
    /*GETStockSupplierGetNameFromId,
    GETStockSupplierAddressFromId,
    GETStockSupplierContactInfoFromId,
    GETStockSupplierRepFromId,
    GETStockSupplierSupplyCategories,
    GETStockRepIdFromName,
    GETStockRepInfoFromId,
    GETStockRepContactInfoFromId,
    StockItems,
    StockItemById(u32),
    */
    ApiDoc,
}

impl Query {
    fn clone(&self) -> Query {
        match self {
            Query::GETStockSuppliers => Query::GETStockSuppliers,
            Query::GETStockSupplierName => Query::GETStockSupplierName,
            Query::GETStockSupplierId => Query::GETStockSupplierId,
            /*Query::GETStockSupplierGetNameFromId => Query::GETStockSupplierGetNameFromId,
            Query::GETStockSupplierAddressFromId => Query::GETStockSupplierAddressFromId,
            Query::GETStockSupplierContactInfoFromId => Query::GETStockSupplierContactInfoFromId,
            Query::GETStockSupplierRepFromId => Query::GETStockSupplierRepFromId,
            Query::GETStockSupplierSupplyCategories => Query::GETStockSupplierSupplyCategories,
            Query::GETStockRepIdFromName => Query::GETStockRepIdFromName,
            Query::GETStockRepInfoFromId => Query::GETStockRepInfoFromId,
            Query::GETStockRepContactInfoFromId => Query::GETStockRepContactInfoFromId,
            Query::StockItems => Query::StockItems,
            Query::StockItemById(id) => Query::StockItemById(*id),*/
            Query::ApiDoc => Query::ApiDoc,
        }
    }
}

pub enum DatabaseType {
    Sqlite,
    Postgres,
}


fn process_query(query: Query, db_type: DatabaseType) -> Result<String, DatabaseError> {
    
    match db_type {
        DatabaseType::Sqlite => query_to_sqlite(query),
        _ => panic!("Invalid database type provided in process_query")
    }
}

fn query_to_sqlite(query: Query) -> Result<String, DatabaseError> {
    
    let database_path = SQLITE_DB_PATH;
    let connection = make_sqlite_db_connection(&database_path)?;
 

    let mut json_object = json::object!{
        "code": 200,
        "success": true,
    };

    match query {

        Query::GETStockSuppliers => {

            // First sql statement retrieves all fields from a "view_suppliers" view
            let statement_result = connection.prepare("SELECT * FROM view_suppliers");
            if let Err(e) = statement_result {
                return Err(DatabaseError::QueryError("Sqlite db query stockItems failed".to_string()));
            }
            let statement = statement_result.unwrap();

            // It has the following table structure:
            let mut supplier_row: DBRow = DBRow::new();
            supplier_row.fields.push(DbField::new(0, "id", Value::Integer));
            supplier_row.fields.push(DbField::new(1, "Name", Value::String));
            supplier_row.fields.push(DbField::new(2, "ContactID", Value::Boolean));
            supplier_row.fields.push(DbField::new(3, "Active", Value::Boolean));
            supplier_row.fields.push(DbField::new(4, "Address_Line1", Value::String));
            supplier_row.fields.push(DbField::new(5, "Address_Line2", Value::String));
            supplier_row.fields.push(DbField::new(6, "Address_Town", Value::String));
            supplier_row.fields.push(DbField::new(7, "Address_Council", Value::String));
            supplier_row.fields.push(DbField::new(8, "Address_Postcode", Value::String));
            supplier_row.fields.push(DbField::new(9, "Rep_FirstName", Value::String));
            supplier_row.fields.push(DbField::new(10, "Rep_LastName", Value::String));
            supplier_row.fields.push(DbField::new(11, "Rep_ContactID", Value::Integer));    
            let suppliers_dump: JsonValue = sqlite_to_json_payload(statement, db_row);
            

            if suppliers_dump.is_null() || !suppliers_dump.is_array() {
                return Ok(json_object.dump());
            }

            let statement_result = connection.prepare("SELECT * FROM view_suppliers");
            if let Err(e) = statement_result {
                return Err(DatabaseError::QueryError("Sqlite db query stockItems failed".to_string()));
            }
            let statement = statement_result.unwrap();
        },
        Query::StockItems => {
            
           
            let statement_result = connection.prepare("SELECT * FROM stock_items");
            if let Err(e) = statement_result {
                return Err(DatabaseError::QueryError("Sqlite db query stockItems failed".to_string()));
            }
            let statement = statement_result.unwrap(); 

            let mut db_row: DBRow = DBRow::new();
            db_row.fields.push(DbField::new(0, "id", Value::Float));
            db_row.fields.push(DbField::new(1, "name", Value::String));
            db_row.fields.push(DbField::new(2, "price", Value::Float));
            db_row.fields.push(DbField::new(3, "quantity", Value::Integer));
            db_row.fields.push(DbField::new(4, "supplier", Value::String));
            json_object["payload"] = sqlite_to_json_payload(statement, db_row)

        },
        Query::StockItemById(id) => {

            return Err(DatabaseError::NotImplemented("Sqlite db query stockItemById has not been implemented yet".to_string()));
        },
        _ => panic!("Invalid query provided in query_to_sqlite"),
    }
    Ok(json_object.dump())
}




fn make_sqlite_db_connection(database_path: &str) -> Result<Connection, DatabaseError> {

    if let Ok(connection) = sqlite::open(std::path::Path::new(&database_path))  {
        Ok(connection)
    } else {
        return Err(DatabaseError::ConnectionError("Failed to connect to db".to_string()));
        
    }
}


fn sqlite_to_json_payload(mut statement: Statement, db_table_row: DBRow) -> json::JsonValue {

    if statement.column_count() != db_table_row.fields.len() {
        panic!("Number of columns in the statement does not match the number of fields in the db table row");
    }



    let mut json_array = json::JsonValue::new_array();


    // iterate through each table row entry of the retrieved sql data    
    while let Ok(State::Row) = statement.next() { 

        // create a json object to store the data for each entry
        let mut entry_object = json::object!{};
        
        // using the db table row as a mechanism to assign the each row sql field entry to its json equivalent
        for field in db_table_row.fields.iter() {
            let id = field.column;
            let name = field.name.as_str();
            let a_type = &field.a_type;
            match a_type {
                Value::Boolean =>{
                    let value = statement.read::<i64, _>(id).unwrap();
                    if value == 0 {
                        entry_object[name] = false.into();
                    }
                    else {
                        entry_object[name] = true.into();
                    }
                },
                Value::Binary => {
                    entry_object[name] = statement.read::<Vec<u8>, _>(id).unwrap().into();
                },
                Value::Float => {
                    entry_object[name] = statement.read::<f64, _>(id).unwrap().into();
                },
                Value::Integer => {
                    entry_object[name] = statement.read::<i64, _>(id).unwrap().into();
                },
                Value::String => { 
                    entry_object[name] = statement.read::<String, _>(id).unwrap().into();
                },
                Value::Null => {

                    // spit out value which will be used to determine if the field value is null
                    let a_value: sqlite::Value = statement.read(id).unwrap();

                    // if the value is null then set the json value to null
                    if a_value.kind().eq(&sqlite::Type::Null) {
                        entry_object[name] = json::JsonValue::Null;
                    }
                    else {
                        panic!("Invalid value type provided in sqlite_to_json, should have been null");
                    }
                },
            }
        }

        json_array.push(entry_object).unwrap();   
    }
    json_array
}

pub enum Value {
    Boolean,
    Binary,
    Float,
    Integer,
    String,
    Null,
}

pub struct DbField {
    column: usize,
    name: String,
    a_type: Value,
}

impl DbField {
    pub fn new(column: usize, name: &str, a_type: Value) -> DbField {
        DbField { 
            column, 
            name: name.to_string(), 
            a_type
        }
    }
}
pub struct DBRow {
    fields: Vec<DbField>,
}
impl DBRow {
    pub fn new() -> DBRow {
        DBRow {
            fields: Vec::new(),
        }
    }
}
    
