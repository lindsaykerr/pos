pub mod sqlite;
pub mod data_structs;
pub mod sqlite_tables;

use crate::server::databases::sqlite::to_json;
use crate::server::api::query_types::Query;
use data_structs::Type;
use crate::errors::DatabaseError;


pub fn process_query(query: Query, db_type: Type) -> Result<String, DatabaseError> {
    
    match db_type {
        Type::Sqlite => to_json(query),
        _ => panic!("Invalid database type provided in process_query")
    }
}
