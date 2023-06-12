use crate::errors::DatabaseError;
use crate::server::api::query_types::Query;
use crate::server::databases::data_structs::{DBTable, DBTableStruct, DBTableRow, Value};
use crate::server::databases::sqlite::sqlite_tables;
use crate::server::databases::sqlite::get_sql_queries;
use sqlite::{self, Statement, State};

pub fn dbtable_from_query(query_type: &Query, connection: &sqlite::Connection) -> Result<DBTable, DatabaseError> {

    // try connect to and query the db
    let query = get_sql_queries::get_sql(query_type);
    //println!("sql query: {}", query);
    let statement_result = connection.prepare(query.as_str());
    if let Err(_e) = statement_result {
        println!("sqlite_DBTable_from_query connection.prepare failed");
        return Err(DatabaseError::QueryError("Sqlite db query stockItems failed".to_string()));
    }
    let statement = statement_result.unwrap();
 
    // gets the table structure for this query
    
    let dbtable = sqlite_tables::get_tables(query_type.clone()); 
    let response_data = response_data_into_db_table(statement, dbtable);
 
    
    Ok(response_data)
 }
 
 // places the query results into a DBTable
 fn response_data_into_db_table(mut statement: Statement, row_structure: DBTableStruct) -> DBTable {
 
    if statement.column_count() != row_structure.fields.len() {
        println!("statement columns {}, row_structure.fields.len() {}", statement.column_count(), row_structure.fields.len());
        panic!("Number of columns in the statement does not match the number of fields in the db table row");
    }
 
    let mut db_table = DBTable::new(&row_structure);
 
    while let Ok(State::Row) = statement.next() { 
 
        let mut db_row = DBTableRow::new();
        
        // using the row structure as a guide, we can iterate through the required fields in the row
 
        for field in row_structure.fields.iter() {
 
            let name = field.name.as_str();
            let field_type = &field.field_type;
            let not_null_flag = field.not_null;
            
            // read a value from a cell within a row using the index of the cell
            
            let value: sqlite::Value = statement.read(field.index).unwrap();  
            
            // sometimes the value of a db entry may be null, so we need to check for this
            
            if not_null_flag && value.kind().eq(&sqlite::Type::Null) {  
 
                println!("Database field {} is null, but is not allowed to be", name);
                continue;
            }
         
            // knowing that the value should be of a certain type, the next step is to convert it 
            // to that type and add it to a the DBTableRow struct
 
            match field_type {
                Value::Boolean(_) => {
                    if let Ok(value) = TryFrom::try_from(&value) {
                        let value: i64 = value;
                        if value == 0 {
                            db_row.add_cell(Value::Boolean(false));
                        }
                        else {
                            db_row.add_cell(Value::Boolean(true));
                        }
                    } 
                    else {
                        db_row.add_cell(Value::Null);
                    }             
        
                }, 
                Value::Float(_) => {
                    if let Ok(value) = TryFrom::try_from(&value) {
                        let value: f64 = value;
                        db_row.add_cell(Value::Float(value));
                    }
                    else {
                        db_row.add_cell(Value::Null);
                    }
        
                },
                Value::Integer(_) => {
                    if let Ok(value) = TryFrom::try_from(&value) {
                        let value: i64 = value;
                        db_row.add_cell(Value::Integer(value));
                    }
                    else {
                        db_row.add_cell(Value::Null);
                    }
                },
                Value::String(_) => { 
                    if let Ok(value) = TryFrom::try_from(value) {
                        let value: String = value;
                        db_row.add_cell(Value::String(value));
                    }
                    else {                        
                        db_row.add_cell(Value::Null);
                    }
                },
                Value::Binary(_) => {
                    if let Ok(value) = TryFrom::try_from(value) {
                        let value: Vec<u8> = value;
                        db_row.add_cell(Value::Binary(value));
                    }
                    else {
                        db_row.add_cell(Value::Null);
                    }
                },
                Value::Null => {
 
                    db_row.add_cell(Value::Null);
                }
            }
        }
        db_table.add_row(db_row);
    }
    db_table
 }










