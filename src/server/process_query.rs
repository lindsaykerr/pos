

use crate::errors::DatabaseError;
use crate::server::api::query_types::Query;
use crate::server::connection::Request;
use crate::server::databases::sqlite;

pub fn get_request(query: Query, request: Request) -> (String, String, String) {

    // specify which process function to use based on the query
    // these will unusually tied to some sort of database query
    let result: Result<String,DatabaseError>;

    match query {
        Query::GETSuppliers | 
        Query::GETSuppliersCategories | 
        Query::GETSuppliersEmail | 
        Query::GETSuppliersNumbers | 
        Query::GETSupplierFromId(_) |
        Query::GETSupplierIdFromName(_) |
        Query::GETSupplierNameFromId(_) |
        Query::GETSupplierEmailFromId(_) |
        Query::GETSupplierNumbersFromId(_) |
        Query::GETSupplierAddressFromId(_) |
        Query::GETSupplierCategoriesFromId(_) |
        Query::GETSupplierRepFromId(_) |
        Query::GETSupplyRepFromId(_) |
        Query::GETSupplyRepPhoneNumbersFromId(_) |
        Query::GETSupplyRepEmailFromId(_) => {
            result = sqlite::get_request(query);
        },
        _ => {
            panic!("Invalid GET query: {:?}", query);
        }



    }
    match result {
        Ok(content) => {
            return (content, String::from("application/json"), String::from("HTTP/1.1 200 OK"));
        },
        Err(error) => {
            println!("Error: {:?}", error);
            return ("Api Error".to_string(), String::from("text/html"), String::from("HTTP/1.1 500 Internal Server Error"));
        }
    }


}



pub fn post_request(query: Query, request: Request) -> (String, String, String) {
    let mut result: Result<String,DatabaseError>;

    match &query {
        Query::POSTSupplier(_) => {
            result = sqlite::post_request(query, request.body);
        },
        _ => {
            panic!("Invalid POST query: {:?}", query);
        }
    }
    (String::from("content"), String::from("content_type"), String::from("status_line"))

}

pub fn put_request(query: Query, request: Request) -> (String, String, String) {
    (String::from("content"), String::from("content_type"), String::from("status_line"))

}

pub fn delete_request(query: Query, request: Request) -> (String, String, String) {
    (String::from("content"), String::from("content_type"), String::from("status_line"))

}


