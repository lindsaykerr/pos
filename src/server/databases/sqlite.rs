pub mod get_processing;
pub mod get_sql_queries;
pub mod post_sql_queries;
pub mod sqlite_tables;
pub mod post_processing;
pub mod util;

use crate::server::api::query_types::{Content, Query};
use crate::errors::DatabaseError;
use json::{self, JsonValue};
use util::open_connection;


pub fn get_request(query: Query) -> Result<String, DatabaseError> {
    let json_response = get_processing::process_query(query)?;
    Ok(json_response.dump())
}

pub fn post_request(query: Query, body: Content) -> Result<String, DatabaseError> {
    let json_response: JsonValue;
    match body {
        Content::Json(content) => {
            json_response = post_processing::process_query(query, content)?;
        },
        _ => {
            return Err(DatabaseError::SubmissionError("Invalid body content type".to_string()));
        }
    }


    Ok(json_response.dump())
}

