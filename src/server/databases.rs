use crate::server::api::Query;
use crate::errors::DatabaseError;
use crate::config::SQLITE_DB_PATH;
use json::{self, JsonValue};
use sqlite::{State, Statement, Connection};

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
            let suppliers_dump: JsonValue = sqlite_to_json_payload(statement, supplier_row);
            

            if suppliers_dump.is_null() || !suppliers_dump.is_array() {
                return Ok(json_object.dump());
            }

            let statement_result = connection.prepare("SELECT * FROM view_suppliers");
            if let Err(e) = statement_result {
                return Err(DatabaseError::QueryError("Sqlite db query stockItems failed".to_string()));
            }
            let statement = statement_result.unwrap();
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