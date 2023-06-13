pub mod get_processing;
pub mod get_sql_queries;
pub mod post_sql_queries;
pub mod sqlite_tables;
pub mod post_processing;
pub mod util;


use crate::server::api::query_types::{Content, Query};
use crate::server::databases::config::data_keys;
use crate::errors::DatabaseError;
use crate::config::SQLITE_DB_PATH;


use json::{self, JsonValue};
use crate::server::databases::data_structs::{
    Value, JsonStructType, set_json_object
};
use get_processing::dbtable_from_query;
//use crate::server::connection::Request;
use util::open_connection;


pub fn get_request(query: Query) -> Result<String, DatabaseError> {
    let json_response = to_json(query)?;
    Ok(json_response.dump())
}

pub fn post_request(query: Query, body: Content) -> Result<String, DatabaseError> {
    let json_response: JsonValue;
    match body {
        Content::Json(content) => {
            json_response = from_json(query, content)?;
        },
        _ => {
            return Err(DatabaseError::SubmissionError("Invalid body content type".to_string()));
        }
    }


    Ok(json_response.dump())
}

fn from_json(query: Query, body_content: JsonValue) -> Result<JsonValue, DatabaseError> {
    let database_path = SQLITE_DB_PATH;
    let connection = open_connection(&database_path)?;
    let mut json_object = json::object!{
        "code": 200,
        "success": false,
    };

    match query {
        Query::POSTSupplier(_) => {

            // get the table structure of the supplier table which matches the sqlite db table
            let supplier_table = sqlite_tables::post_tables(query.clone());
            
            // get the values needed to insert and entry into the supplier table
            if let Ok(value_map) = post_processing::extract_json_to_table(&body_content, supplier_table) {

                // using those values build the SQL insert statement
                let sql_insert_supplier = post_sql_queries::post_sql(query, value_map);

                // execute the SQL statement and notify the user of if successful or not
                json_object["success"] = if let Some(sql) = sql_insert_supplier {
                    if let Err(_) = connection.execute(sql) {
                        return Err(DatabaseError::SubmissionError("Failed to insert supplier".to_string()));
                    }
                    else {
                        json::JsonValue::Boolean(true)
                    }          
                } else {
                    json_object["message"] = json::JsonValue::String("Insertion failure".to_string());
                    json::JsonValue::Boolean(false)
                }           
            }
        },
        _ => {
            return Err(DatabaseError::SubmissionError("Invalid query type".to_string()));
        }
    }

    Ok(json_object)
}


///
/// Queries the sqlite database and returns a response in the form of a json string.
///
pub fn to_json(query: Query) -> Result<JsonValue, DatabaseError> {
    
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
           
            let table_v_suppliers = dbtable_from_query(&query, &connection)?;

            if table_v_suppliers.rows.len() == 0 {
                return Ok(json_object);
            }

            json_object["payload"] = table_v_suppliers.to_json();

        },
        Query::GETSupplierFromId(id) => {
                
            let supplier = dbtable_from_query(&query, &connection)?;

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
            let supplier_email = dbtable_from_query(
                &Query::GETSupplierEmailFromId(id), 
                &connection
            )?;

            if supplier_email.rows.len() > 0 {
                json_object["payload"][data_keys::CONTACT][data_keys::EMAIL] = set_json_object(
                    &supplier_email, 
                    JsonStructType::TableColumn(1)
                );
            }
            
            // using the supplier id, get the associated contact phone numbers
            let contact_numbers = dbtable_from_query(
                &Query::GETSupplierNumbersFromId(id), 
                &connection, 
            )?;
            
            if contact_numbers.rows.len() > 0 {
                json_object["payload"][data_keys::CONTACT][data_keys::NUMBER] = set_json_object(
                    &contact_numbers, 
                    JsonStructType::TableColumn(1)
                );
            }


            // using the supplier id, get the supplier address information if any
            let address = dbtable_from_query(
                &Query::GETSupplierAddressFromId(id),
                &connection, 
                )?;

            if address.rows.len() > 0 {
                json_object["payload"][data_keys::ADDRESS] = set_json_object(&address, JsonStructType::Object);
                json_object["payload"][data_keys::ADDRESS].remove(data_keys::ID);
            }
            
            // using the supplier id, get the supplier rep information if any
            let rep = dbtable_from_query(
                &Query::GETSupplierRepFromId(id), 
                &connection
            )?;

            if rep.rows.len() > 0 {
                json_object["payload"][data_keys::REP] = set_json_object(&rep, JsonStructType::Object);
                json_object["payload"][data_keys::REP].remove(data_keys::ID);
                json_object["payload"][data_keys::REP].remove(data_keys::CONTACT_ID);
                
                if let Value::Integer(rep_id) = rep.rows[0].cells[0] {
                    let email = dbtable_from_query(
                        &Query::GETSupplyRepEmailFromId(rep_id as u64), 
                        &connection)?;
        
                    if email.rows.len() > 0 {
                        json_object["payload"][data_keys::REP][data_keys::CONTACT][data_keys::EMAIL] = set_json_object(
                            &email, 
                            JsonStructType::TableColumn(1)
                        );
                    }
        
                    let numbers = dbtable_from_query(
                        &Query::GETSupplyRepPhoneNumbersFromId(rep_id as u64), 
                        &connection)?;
        
                    if numbers.rows.len() > 0 {
                        json_object["payload"][data_keys::REP][data_keys::CONTACT][data_keys::NUMBER] = set_json_object(
                            &numbers, 
                            JsonStructType::TableColumn(1)
                        );
                    }
                }
               
                    
            }
        },
        Query::GETSuppliersEmail => {
            
            let v_suppliers_email = dbtable_from_query(
                &query, 
                &connection
            )?;

            if v_suppliers_email.rows.len() == 0 {
                return Ok(json_object);
            }

            json_object["payload"] = set_json_object(&v_suppliers_email, JsonStructType::Table);
        },
        Query::GETSuppliersNumbers => {
            
            let v_suppliers_numbers = dbtable_from_query(
                &query, 
                &connection
            )?;

            if v_suppliers_numbers.rows.len() == 0 {
                return Ok(json_object);
            }

            json_object["payload"] = set_json_object(&v_suppliers_numbers, JsonStructType::Table);
        },
        Query::GETSupplierIdFromName(_) => {


            let id = dbtable_from_query(
                &query, 
                &connection
            )?;

            if id.rows.len() == 0 {
                return Ok(json_object);
            }
            
            json_object["payload"] = set_json_object(&id, JsonStructType::Object);

        },

        Query::GETSupplierNameFromId(_) => {
            
            let name = dbtable_from_query(&query, &connection)?;

            if name.rows.len() == 0 {
                return Ok(json_object);
            }

            json_object["payload"] = set_json_object(&name, JsonStructType::Object);
        },

        Query::GETSupplierAddressFromId(_) => {
            
            let address = dbtable_from_query(&query, &connection)?;

            if address.rows.len() == 0 {
                return Ok(json_object);
            }

            json_object["payload"] = set_json_object(&address, JsonStructType::Object);
        },

        Query::GETSupplierRepFromId(_) => {
            
            let rep = dbtable_from_query(&query, &connection)?;


            if rep.rows.len() > 0 {
                json_object["payload"] = set_json_object(&rep, JsonStructType::Object);
                json_object["payload"].remove(data_keys::CONTACT_ID);

                // using the contact id retrieve the contact email addresses and numbers
               
                if let Value::Integer(x) = rep.rows[0].cells[0] {
                    let rep_contact_id: u64 = x as u64;
        
                    let contact_email = dbtable_from_query(
                        &Query::GETSupplyRepEmailFromId(rep_contact_id), 
                        &connection)?;            
    
                    if contact_email.rows.len() > 0 {
                        json_object["payload"][data_keys::CONTACT][data_keys::EMAIL] = set_json_object(
                            &contact_email, 
                            JsonStructType::TableColumn(1)
                        );

                    }

                    let contact_numbers = dbtable_from_query(
                        &Query::GETSupplyRepPhoneNumbersFromId(rep_contact_id), 
                        &connection)?; 
                    if contact_numbers.rows.len() > 0 {
                        json_object["payload"][data_keys::CONTACT][data_keys::NUMBER] = set_json_object(
                            &contact_email, 
                            JsonStructType::TableColumn(1)
                        );
                    }
                }          
            }
        },
        Query::GETSuppliersCategories => {
            
            let categories = dbtable_from_query(
                &query, 
                &connection)?;

            if categories.rows.len() == 0 {
                return Ok(json_object);
            }

            json_object["payload"] = set_json_object(&categories, JsonStructType::Table);
        },
        Query::GETSupplierCategoriesFromId(_) => {
            
            let supply_categories = dbtable_from_query(
                &query, 
                &connection)?;

            if supply_categories.rows.len() == 0 {
                return Ok(json_object);
            }

            json_object["payload"] = set_json_object(&supply_categories, JsonStructType::Table);
        },
        Query::GETSupplyRepFromId(id) => {
            
            let rep = dbtable_from_query(
                &query, 
                &connection)?;

            if rep.rows.len() == 0 {
                return Ok(json_object);
            }

            json_object["payload"] = set_json_object(&rep, JsonStructType::Object);

            json_object["payload"].remove(data_keys::CONTACT_ID);

            let email = dbtable_from_query(
                &Query::GETSupplyRepEmailFromId(id), 
                &connection)?;

            if email.rows.len() > 0 {
                json_object["payload"][data_keys::CONTACT][data_keys::EMAIL] = set_json_object(
                    &email, 
                    JsonStructType::TableColumn(1)
                );
           
            }

            let numbers = dbtable_from_query(
                &Query::GETSupplyRepPhoneNumbersFromId(id), 
                &connection)?;

            if numbers.rows.len() > 0 {
                json_object["payload"][data_keys::CONTACT][data_keys::NUMBER] = set_json_object(
                    &numbers, 
                    JsonStructType::TableColumn(1)
                );
            }
        },

        Query::GETSupplyRepEmailFromId(_) => {
            
            let rep_email = dbtable_from_query(
                &query, 
                &connection)?;

            if rep_email.rows.len() == 0 {
                return Ok(json_object);
            }

            json_object["payload"] = set_json_object(&rep_email, JsonStructType::TableColumn(1));
        },
        Query::GETSupplyRepPhoneNumbersFromId(_) => {
            
            let rep_numbers = dbtable_from_query(
                &query, 
                &connection)?;

            if rep_numbers.rows.len() == 0 {
                return Ok(json_object);
            }

            json_object["payload"] = set_json_object(&rep_numbers, JsonStructType::TableColumn(1));
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



