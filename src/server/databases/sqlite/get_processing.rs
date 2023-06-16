use crate::errors::DatabaseError;
use crate::server::api::query_types::Query;
use crate::server::databases::{
    data_structs::{DBTable, DBTableStruct, DBTableRow, Value, table_to_json_struct, JsonStructType},
    sqlite::{ sqlite_tables, get_sql_queries, open_connection},
    config::data_keys,
};
use crate::config::SQLITE_DB_PATH;
use json::JsonValue;
use sqlite::{self, Statement, State};


///
/// Processes query, submits query to database, and returns a json response
///
pub fn process_query(query: Query) -> Result<JsonValue, DatabaseError> {
    
    let database_path = SQLITE_DB_PATH;
    let connection = open_connection(&database_path)?;
 
    let mut json_object = json::object!{
        "code": 200,
        "success": false,
    };

    // Aids in debugging specific queries
    /* 
    let cloned_query = query.clone();
    match cloned_query {
        Query::GETStockSuppliers => println!("GETStockSuppliers"),
        Query::GETStockSupplierId(id) => println!("GETStockSupplierId({})", id),
        Query::GETStockSupplierName => println!("GETStockSupplierName"),
        _ => println!("Invalid query")
    }
    */

    match query {

        Query::GETSuppliers => {
           
            let table_v_suppliers = data_table_from_query(&query, &connection)?;

            if table_v_suppliers.rows.len() == 0 {
                return Ok(json_object);
            }

            json_object["payload"] = table_v_suppliers.to_json();

        },
        Query::GETSupplierFromId(id) => {
                
            let supplier = data_table_from_query(&query, &connection)?;

            if supplier.rows.len() == 0 {
                return Ok(json_object);
            }

      
            // setup reply json object
            json_object["payload"] = json::object! {
                data_keys::ID => id,
                data_keys::NAME => supplier.rows[0].cells[1].to_json(),
                data_keys::ACTIVE => supplier.rows[0].cells[2].to_json(),
                data_keys::CONTACT => JsonValue::Null,
                data_keys::REP => JsonValue::Null,
                data_keys::ADDRESS => JsonValue::Null,
            };

      
            // using the supplier id, get the associated contact email addresses
            let supplier_email = data_table_from_query(
                &Query::GETSupplierEmailFromId(id), 
                &connection
            )?;

            if supplier_email.rows.len() > 0 {
                json_object["payload"][data_keys::CONTACT][data_keys::EMAIL] = table_to_json_struct(
                    &supplier_email, 
                    JsonStructType::TableColumn(1)
                );
            }
            
            // using the supplier id, get the associated contact phone numbers
            let contact_numbers = data_table_from_query(
                &Query::GETSupplierNumbersFromId(id), 
                &connection, 
            )?;
            
            if contact_numbers.rows.len() > 0 {
                json_object["payload"][data_keys::CONTACT][data_keys::NUMBER] = table_to_json_struct(
                    &contact_numbers, 
                    JsonStructType::TableColumn(1)
                );
            }


            // using the supplier id, get the supplier address information if any
            let address = data_table_from_query(
                &Query::GETSupplierAddressFromId(id),
                &connection, 
                )?;

            if address.rows.len() > 0 {
                json_object["payload"][data_keys::ADDRESS] = table_to_json_struct(&address, JsonStructType::Object);
                json_object["payload"][data_keys::ADDRESS].remove(data_keys::ID);
            }
            
            // using the supplier id, get the supplier rep information if any
            let rep = data_table_from_query(
                &Query::GETSupplierRepFromId(id), 
                &connection
            )?;

            if rep.rows.len() > 0 {
                json_object["payload"][data_keys::REP] = table_to_json_struct(&rep, JsonStructType::Object);
                json_object["payload"][data_keys::REP].remove(data_keys::ID);
                json_object["payload"][data_keys::REP].remove(data_keys::CONTACT_ID);
                
                if let Value::Integer(rep_id) = rep.rows[0].cells[0] {
                    let email = data_table_from_query(
                        &Query::GETSupplyRepEmailFromId(rep_id as u64), 
                        &connection)?;
        
                    if email.rows.len() > 0 {
                        json_object["payload"][data_keys::REP][data_keys::CONTACT][data_keys::EMAIL] = table_to_json_struct(
                            &email, 
                            JsonStructType::TableColumn(1)
                        );
                    }
        
                    let numbers = data_table_from_query(
                        &Query::GETSupplyRepPhoneNumbersFromId(rep_id as u64), 
                        &connection)?;
        
                    if numbers.rows.len() > 0 {
                        json_object["payload"][data_keys::REP][data_keys::CONTACT][data_keys::NUMBER] = table_to_json_struct(
                            &numbers, 
                            JsonStructType::TableColumn(1)
                        );
                    }
                }
               
                    
            }
        },
        Query::GETSuppliersEmail => {
            
            let v_suppliers_email = data_table_from_query(
                &query, 
                &connection
            )?;

            if v_suppliers_email.rows.len() == 0 {
                return Ok(json_object);
            }

            json_object["payload"] = table_to_json_struct(&v_suppliers_email, JsonStructType::Table);
        },
        Query::GETSuppliersNumbers => {
            
            let v_suppliers_numbers = data_table_from_query(
                &query, 
                &connection
            )?;

            if v_suppliers_numbers.rows.len() == 0 {
                return Ok(json_object);
            }

            json_object["payload"] = table_to_json_struct(&v_suppliers_numbers, JsonStructType::Table);
        },
        Query::GETSupplierIdFromName(_) => {


            let id = data_table_from_query(
                &query, 
                &connection
            )?;

            if id.rows.len() == 0 {
                return Ok(json_object);
            }
            
            json_object["payload"] = table_to_json_struct(&id, JsonStructType::Object);

        },

        Query::GETSupplierNameFromId(_) => {
            
            let name = data_table_from_query(&query, &connection)?;

            if name.rows.len() == 0 {
                return Ok(json_object);
            }

            json_object["payload"] = table_to_json_struct(&name, JsonStructType::Object);
        },

        Query::GETSupplierAddressFromId(_) => {
            
            let address = data_table_from_query(&query, &connection)?;

            if address.rows.len() == 0 {
                return Ok(json_object);
            }

            json_object["payload"] = table_to_json_struct(&address, JsonStructType::Object);
        },

        Query::GETSupplierRepFromId(_) => {
            
            let rep = data_table_from_query(&query, &connection)?;


            if rep.rows.len() > 0 {
                json_object["payload"] = table_to_json_struct(&rep, JsonStructType::Object);
                json_object["payload"].remove(data_keys::CONTACT_ID);

                // using the contact id retrieve the contact email addresses and numbers
               
                if let Value::Integer(x) = rep.rows[0].cells[0] {
                    let rep_contact_id: u64 = x as u64;
        
                    let contact_email = data_table_from_query(
                        &Query::GETSupplyRepEmailFromId(rep_contact_id), 
                        &connection)?;            
    
                    if contact_email.rows.len() > 0 {
                        json_object["payload"][data_keys::CONTACT][data_keys::EMAIL] = table_to_json_struct(
                            &contact_email, 
                            JsonStructType::TableColumn(1)
                        );

                    }

                    let contact_numbers = data_table_from_query(
                        &Query::GETSupplyRepPhoneNumbersFromId(rep_contact_id), 
                        &connection)?; 
                    if contact_numbers.rows.len() > 0 {
                        json_object["payload"][data_keys::CONTACT][data_keys::NUMBER] = table_to_json_struct(
                            &contact_email, 
                            JsonStructType::TableColumn(1)
                        );
                    }
                }          
            }
        },
        Query::GETSuppliersCategories => {
            
            let categories = data_table_from_query(
                &query, 
                &connection)?;

            if categories.rows.len() == 0 {
                return Ok(json_object);
            }

            json_object["payload"] = table_to_json_struct(&categories, JsonStructType::Table);
        },
        Query::GETSupplierCategoriesFromId(_) => {
            
            let supply_categories = data_table_from_query(
                &query, 
                &connection)?;

            if supply_categories.rows.len() == 0 {
                return Ok(json_object);
            }

            json_object["payload"] = table_to_json_struct(&supply_categories, JsonStructType::Table);
        },
        Query::GETSupplyRepFromId(id) => {
            
            let rep = data_table_from_query(
                &query, 
                &connection)?;

            if rep.rows.len() == 0 {
                return Ok(json_object);
            }

            json_object["payload"] = table_to_json_struct(&rep, JsonStructType::Object);

            json_object["payload"].remove(data_keys::CONTACT_ID);

            let email = data_table_from_query(
                &Query::GETSupplyRepEmailFromId(id), 
                &connection)?;

            if email.rows.len() > 0 {
                json_object["payload"][data_keys::CONTACT][data_keys::EMAIL] = table_to_json_struct(
                    &email, 
                    JsonStructType::TableColumn(1)
                );
           
            }

            let numbers = data_table_from_query(
                &Query::GETSupplyRepPhoneNumbersFromId(id), 
                &connection)?;

            if numbers.rows.len() > 0 {
                json_object["payload"][data_keys::CONTACT][data_keys::NUMBER] = table_to_json_struct(
                    &numbers, 
                    JsonStructType::TableColumn(1)
                );
            }
        },

        Query::GETSupplyRepEmailFromId(_) => {
            
            let rep_email = data_table_from_query(
                &query, 
                &connection)?;

            if rep_email.rows.len() == 0 {
                return Ok(json_object);
            }

            json_object["payload"] = table_to_json_struct(&rep_email, JsonStructType::TableColumn(1));
        },
        Query::GETSupplyRepPhoneNumbersFromId(_) => {
            
            let rep_numbers = data_table_from_query(
                &query, 
                &connection)?;

            if rep_numbers.rows.len() == 0 {
                return Ok(json_object);
            }

            json_object["payload"] = table_to_json_struct(&rep_numbers, JsonStructType::TableColumn(1));
        },
        _ => {
            let error_message = format!("Query has not been implemented provided: {:?}", query);
            return Err(DatabaseError::QueryError(error_message));
        }

    }
    if !json_object["payload"].is_null() {
        json_object["success"] = json::JsonValue::Boolean(true);
    }

    Ok(json_object)
}

pub fn data_table_from_query(query_type: &Query, connection: &sqlite::Connection) -> Result<DBTable, DatabaseError> {

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
    let response_data = db_data_into_table(statement, dbtable);
 
    
    Ok(response_data)
 }
 
 // places values returned by db query into a DBTable
 fn db_data_into_table(mut statement: Statement, row_structure: DBTableStruct) -> DBTable {
 
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










