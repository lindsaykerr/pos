use crate::server::api::Query;
use crate::errors::DatabaseError;
use crate::config::SQLITE_DB_PATH;
use sqlite::{State, Statement, Connection};
use json::{self};
use crate::server::databases::data_structs::{DBTable, DBTableRow, DBTableStruct,  DbFieldStruct, Value};


pub fn query_for_sqlite_db(query: Query) -> Result<String, DatabaseError> {
    
    let database_path = SQLITE_DB_PATH;
    let connection = open_connection(&database_path)?;
 

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
           
            let table_v_suppliers = db_table_from_query(
                &query, 
                &connection, 
                "SELECT * FROM view_suppliers"
            )?;

            if table_v_suppliers.rows.len() == 0 {
                return Ok(json_object.dump());
            }

            json_object["payload"] = table_v_suppliers.to_json();
            // TODO: add the other tables to the json object
            /*
            let table_v_suppliers_email = DBTable_from_query(
                &Query::GetStockSuppliersEmail, 
                &connection, 
                "SELECT * FROM view_suppliers_email"
            )?;

            let table_v_suppliers_numbers = DBTable_from_query(
                &Query::GetStockSuppliersNumbers, 
                &connection, 
                "SELECT * FROM view_suppliers_numbers"
            )?;
            */

   
        },
        _ => {
            return Err(DatabaseError::QueryError("Invalid query provided".to_string()));
        }

    }
    Ok(json_object.dump())
}

// database connection
fn open_connection(database_path: &str) -> Result<Connection, DatabaseError> {

    if let Ok(connection) = sqlite::open(std::path::Path::new(&database_path))  {
        Ok(connection)
    } else {
        return Err(DatabaseError::ConnectionError("Failed to connect to db".to_string()));
        
    }
}


fn db_table_from_query(query_type: &Query, connection: &sqlite::Connection, sql_query: &str ) -> Result<DBTable, DatabaseError> {
   // try connect to and query the db
   let statement_result = connection.prepare(sql_query.clone());
   if let Err(_e) = statement_result {
       println!("sqlite_DBTable_from_query connection.prepare failed");
       return Err(DatabaseError::QueryError("Sqlite db query stockItems failed".to_string()));
   }
   let statement = statement_result.unwrap();
   // get the expected table structure for this query
   let v_suppliers_row_struct = db_tables(&query_type.clone());

   // build a DBTable from the query results and the expected table structure 
   let table_v_suppliers = response_data_into_db_table(statement, v_suppliers_row_struct);

   
   Ok(table_v_suppliers)
}

fn response_data_into_db_table(mut statement: Statement, row_structure: DBTableStruct) -> DBTable {
   if statement.column_count() != row_structure.fields.len() {
       println!("statement columns {}, row_structure.fields.len() {}", statement.column_count(), row_structure.fields.len());
       panic!("Number of columns in the statement does not match the number of fields in the db table row");
   }

   let mut db_table = DBTable::new(&row_structure);

   while let Ok(State::Row) = statement.next() { 


       let mut db_row = DBTableRow::new();

       for field in row_structure.fields.iter() {
           let id = field.column;
           let name = field.name.as_str();
           let a_type = &field.a_type;
           let not_null_flag = field.not_null;
           
           let value: sqlite::Value = statement.read(id).unwrap();
           

           // sometimes the value of a db entry may be null, so we need to check for this
           if not_null_flag && value.kind().eq(&sqlite::Type::Null) {
               println!("Database field {} is null, but is not allowed to be", name);
               continue;
           }
      
           match a_type {
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


// This is used to call a DBRow struct that represents the expected column names 
// of the tables for a given query request. This is used to help map the sql to another 
// data type such as json 
fn db_tables(for_query: &Query) -> DBTableStruct {
   match for_query {
       Query::GETStockSuppliers => {
           let mut supplier_row: DBTableStruct = DBTableStruct::new();
           supplier_row.fields.push(
               DbFieldStruct::new(0, "id", Value::Integer(-1), true));
           supplier_row.fields.push(
               DbFieldStruct::new(1, "name", Value::String(String::new()), true));
           supplier_row.fields.push(
     
               DbFieldStruct::new(2, "contactId", Value::Integer(-1), true));
           supplier_row.fields.push(
               DbFieldStruct::new(3, "active", Value::Boolean(true), true));
           supplier_row.fields.push(
               DbFieldStruct::new(4, "addressLine1", Value::String(String::new()), false));
           supplier_row.fields.push(
               DbFieldStruct::new(5, "addressLine2", Value::String(String::new()), false));
           supplier_row.fields.push(
               DbFieldStruct::new(6, "addressTown", Value::String(String::new()), false));
           supplier_row.fields.push(
               DbFieldStruct::new(7, "addressCouncil", Value::String(String::new()), false));
           supplier_row.fields.push(
               DbFieldStruct::new(8, "addressPostCode", Value::String(String::new()), false));
           supplier_row.fields.push(
               DbFieldStruct::new(9, "repFirstName", Value::String(String::new()), false));
           supplier_row.fields.push(
               DbFieldStruct::new(10, "repLastName", Value::String(String::new()), false));
           supplier_row.fields.push(
               DbFieldStruct::new(11, "repContactId", Value::Integer(-1), false));

           supplier_row
       },
       Query::GetStockSuppliersEmail => {
           let mut supplier_row: DBTableStruct = DBTableStruct::new();
           supplier_row.fields.push(
               DbFieldStruct::new(0, "supplierId", Value::Integer(0), true));
           supplier_row.fields.push(
               DbFieldStruct::new(1, "email", Value::String(String::new()), true));
           supplier_row
       },
       Query::GetStockSuppliersNumbers => {
           let mut supplier_row: DBTableStruct = DBTableStruct::new();
           supplier_row.fields.push(
               DbFieldStruct::new(0, "supplierId", Value::Integer(0), true));
           supplier_row.fields.push(
               DbFieldStruct::new(1, "phone", Value::String(String::new()), true));
           supplier_row
       },
       _ => DBTableStruct::new()
   }
}