use crate::server::api::Query;
use crate::errors::DatabaseError;
use crate::config::SQLITE_DB_PATH;

use json::{self, JsonValue};
use sqlite::{State, Statement, Connection, ReadableWithIndex, Type as SqliteType};

pub enum Type {
    Sqlite,
    Postgres,
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
    not_null: bool,
}

impl DbField {
    pub fn new(column: usize, name: &str, a_type: Value, not_null: bool) -> DbField {
        DbField { 
            column, 
            name: name.to_string(), 
            a_type,
            not_null,
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








pub fn process_query(query: Query, db_type: Type) -> Result<String, DatabaseError> {
    
    match db_type {
        Type::Sqlite => query_to_sqlite(query),
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
    let cloned_query = query.clone();

    // what query is being made?
    match cloned_query {
        Query::GETStockSuppliers => println!("GETStockSuppliers"),
        Query::GETStockSupplierId(id) => println!("GETStockSupplierId({})", id),
        Query::GETStockSupplierName => println!("GETStockSupplierName"),
        _ => println!("Invalid query")
    }

    match query {

        Query::GETStockSuppliers => {

            // First sql statement retrieves all fields from a "view_suppliers" view
            let statement_result = connection.prepare("SELECT * FROM view_suppliers");
            if let Err(e) = statement_result {
                return Err(DatabaseError::QueryError("Sqlite db query stockItems failed".to_string()));
            }
            let statement = statement_result.unwrap();

            // It has the following table structure:
            let db_row_struct = sqlite_db_tables(&query.clone()); 

            let suppliers_dump: JsonValue = sqlite_to_json_payload(statement, db_row_struct);
            

            if suppliers_dump.is_null() || !suppliers_dump.is_array() {
                return Ok(json_object.dump());
            }

            json_object["payload"] = suppliers_dump;
        },
        _ => {
            return Err(DatabaseError::QueryError("Invalid query provided".to_string()));
        }

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
            let not_null_flag = field.not_null;
            
            let value: sqlite::Value = statement.read(id).unwrap();

            // sometimes the value of a db entry may be null, so we need to check for this
            if not_null_flag && value.kind().eq(&sqlite::Type::Null) {
                entry_object[name] = json::JsonValue::Null;
                continue;
            }
       
            match a_type {
                Value::Boolean => {
                
       
                    if let Ok(value) = TryFrom::try_from(&value) {
                        let value: i64 = value;
                        if value == 0 {
                            entry_object[name] = JsonValue::from(false);
                        }
                        else {
                            entry_object[name] = JsonValue::from(true);
                        }
                    } 
                    else {
                        print!("Invalid value type provided in sqlite_to_json, should have been been 1 or 2");
                        entry_object[name] = json::JsonValue::Null;
                    }             
        
                },
             
                Value::Float => {
                    if let Ok(value) = TryFrom::try_from(&value) {
                        let value: f64 = value;
                        entry_object[name] = JsonValue::from(value);
                    }
                    else {
                        print!("Invalid value type provided in sqlite_to_json, should have been a float");
                        entry_object[name] = json::JsonValue::Null;
                    }
        
                },
                Value::Integer => {
                    if let Ok(value) = TryFrom::try_from(&value) {
                        let value: i64 = value;
                        entry_object[name] = JsonValue::from(value);
                    }
                    else {
                        print!("Invalid value type provided in sqlite_to_json, should have been an integer");
                        entry_object[name] = json::JsonValue::Null;
                    }
                },
                Value::String => { 
                    if let Ok(value) = TryFrom::try_from(value) {
                        let value: String = value;
                        entry_object[name] = JsonValue::from(value);
                    }
                    else {
                        print!("Invalid value type provided in sqlite_to_json, should have been a string");
                        entry_object[name] = json::JsonValue::Null;
                    }
                },
                Value::Binary => {
                    if let Ok(value) = TryFrom::try_from(value) {
                        let value: Vec<u8> = value;
                        entry_object[name] = JsonValue::from(value);
                    }
                    else {
                        print!("Invalid value type provided in sqlite_to_json, should have been a string");
                        entry_object[name] = json::JsonValue::Null;
                    }
        
                },
                Value::Null => {
                    entry_object[name] = json::JsonValue::Null;
                }
            }
        }

        json_array.push(entry_object).unwrap();   
    }
    json_array
}

// This function is used to call a DBRow struct that represents the expected column names 
// of the tables found in the working db. This is used to help map the sql to another 
// data type such as json 
fn sqlite_db_tables(for_query: &Query) -> DBRow {
    match for_query {
        Query::GETStockSuppliers => {
            let mut supplier_row: DBRow = DBRow::new();
            supplier_row.fields.push(DbField::new(0, "id", Value::Integer, true));
            supplier_row.fields.push(DbField::new(1, "name", Value::String, true));
            supplier_row.fields.push(DbField::new(2, "address", Value::String, true));
            supplier_row.fields.push(DbField::new(3, "phone", Value::String, true));
            supplier_row.fields.push(DbField::new(4, "email", Value::String, true));
            supplier_row.fields.push(DbField::new(5, "website", Value::String, true));
            supplier_row.fields.push(DbField::new(6, "contact", Value::String, true));
            supplier_row.fields.push(DbField::new(7, "notes", Value::String, true));
            
            supplier_row
        },
        _ => DBRow::new()
    }
}