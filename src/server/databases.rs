pub mod sqlite;
pub mod data_structs;
pub mod sqlite_tables;

use crate::server::databases::sqlite::query_for_sqlite_db;
use crate::server::api::Query;
use data_structs::Type;
use crate::errors::DatabaseError;


pub fn process_query(query: Query, db_type: Type) -> Result<String, DatabaseError> {
    
    match db_type {
        Type::Sqlite => query_for_sqlite_db(query),
        _ => panic!("Invalid database type provided in process_query")
    }
}
