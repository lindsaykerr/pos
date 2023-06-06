use crate::server::api::query_types::Query;
use crate::errors::DatabaseError;
use crate::config::SQLITE_DB_PATH;
use sqlite::{Connection};
use json::{self, JsonValue};
use crate::server::databases::data_structs::{
    Value, JsonStructType, set_json_object
};
use crate::server::databases::sqlite_tables::db_table_from_query;


///
/// Queries the sqlite database and returns a response in the form of a json string.
///
pub fn to_json(query: Query) -> Result<String, DatabaseError> {
    
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
           
            let table_v_suppliers = db_table_from_query(&query, &connection)?;

            if table_v_suppliers.rows.len() == 0 {
                return Ok(json_object.dump());
            }

            json_object["payload"] = table_v_suppliers.to_json();

        },
        Query::GETSupplierFromId(id) => {
                
            let supplier = db_table_from_query(&query, &connection)?;

            if supplier.rows.len() == 0 {
                return Ok(json_object.dump());
            }

      


            // setup reply json object
            json_object["payload"] = json::object! {
                "id" => id,
                "name" => supplier.rows[0].cells[1].to_json(),
                "active" => supplier.rows[0].cells[2].to_json(),
                "contact" => JsonValue::Null,
                "rep" => JsonValue::Null,
                "address" => JsonValue::Null,
            };
            


            // using the supplier id, get the associated contact email addresses
            let supplier_email = db_table_from_query(
                &Query::GETSupplierEmailFromId(id), 
                &connection
            )?;

            if supplier_email.rows.len() > 0 {
                json_object["payload"]["contact"]["email"] = set_json_object(
                    &supplier_email, 
                    JsonStructType::TableColumn(1)
                );
            }
            
            // using the supplier id, get the associated contact phone numbers
            let contact_numbers = db_table_from_query(
                &Query::GETSupplierNumbersFromId(id), 
                &connection, 
            )?;
            
            if contact_numbers.rows.len() > 0 {
                json_object["payload"]["contact"]["numbers"] = set_json_object(
                    &contact_numbers, 
                    JsonStructType::TableColumn(1)
                );
            }


            // using the supplier id, get the supplier address information if any
            let address = db_table_from_query(
                &Query::GETSupplierAddressFromId(id),
                &connection, 
                )?;

            if address.rows.len() > 0 {
                json_object["payload"]["address"] = set_json_object(&address, JsonStructType::Object);
                json_object["payload"]["address"].remove("id");
            }
            
            // using the supplier id, get the supplier rep information if any
            let rep = db_table_from_query(
                &Query::GETSupplierRepFromId(id), 
                &connection
            )?;

            if rep.rows.len() > 0 {
                json_object["payload"]["rep"] = set_json_object(&rep, JsonStructType::Object);
                json_object["payload"]["rep"].remove("id");
                json_object["payload"]["rep"].remove("contactId");
                
                if let Value::Integer(rep_id) = rep.rows[0].cells[0] {
                    let email = db_table_from_query(
                        &Query::GETSupplyRepEmailFromId(rep_id as u64), 
                        &connection)?;
        
                    if email.rows.len() > 0 {
                        json_object["payload"]["rep"]["contact"]["email"] = set_json_object(
                            &email, 
                            JsonStructType::TableColumn(1)
                        );
                    }
        
                    let numbers = db_table_from_query(
                        &Query::GETSupplyRepPhoneNumbersFromId(rep_id as u64), 
                        &connection)?;
        
                    if numbers.rows.len() > 0 {
                        json_object["payload"]["rep"]["contact"]["numbers"] = set_json_object(
                            &numbers, 
                            JsonStructType::TableColumn(1)
                        );
                    }
                }
               
                    
            }
        },
        Query::GETSuppliersEmail => {
            
            let v_suppliers_email = db_table_from_query(
                &query, 
                &connection
            )?;

            if v_suppliers_email.rows.len() == 0 {
                return Ok(json_object.dump());
            }

            json_object["payload"] = set_json_object(&v_suppliers_email, JsonStructType::Table);
        },
        Query::GETSuppliersNumbers => {
            
            let v_suppliers_numbers = db_table_from_query(
                &query, 
                &connection
            )?;

            if v_suppliers_numbers.rows.len() == 0 {
                return Ok(json_object.dump());
            }

            json_object["payload"] = set_json_object(&v_suppliers_numbers, JsonStructType::Table);
        },
        Query::GETSupplierIdFromName(_) => {


            let id = db_table_from_query(
                &query, 
                &connection
            )?;

            if id.rows.len() == 0 {
                return Ok(json_object.dump());
            }
            
            json_object["payload"] = set_json_object(&id, JsonStructType::Object);

        },

        Query::GETSupplierNameFromId(_) => {
            
            let name = db_table_from_query(&query, &connection)?;

            if name.rows.len() == 0 {
                return Ok(json_object.dump());
            }

            json_object["payload"] = set_json_object(&name, JsonStructType::Object);
        },

        Query::GETSupplierAddressFromId(_) => {
            
            let address = db_table_from_query(&query, &connection)?;

            if address.rows.len() == 0 {
                return Ok(json_object.dump());
            }

            json_object["payload"] = set_json_object(&address, JsonStructType::Object);
        },

        Query::GETSupplierRepFromId(_) => {
            
            let rep = db_table_from_query(&query, &connection)?;


            if rep.rows.len() > 0 {
                json_object["payload"] = set_json_object(&rep, JsonStructType::Object);
                json_object["payload"].remove("contactId");

                // using the contact id retrieve the contact email addresses and numbers
               
                if let Value::Integer(x) = rep.rows[0].cells[0] {
                    let rep_contact_id: u64 = x as u64;
        
                    let contact_email = db_table_from_query(
                        &Query::GETSupplyRepEmailFromId(rep_contact_id), 
                        &connection)?;            
    
                    if contact_email.rows.len() > 0 {
                        json_object["payload"]["contact"]["email"] = set_json_object(
                            &contact_email, 
                            JsonStructType::TableColumn(1)
                        );

                    }

                    let contact_numbers = db_table_from_query(
                        &Query::GETSupplyRepPhoneNumbersFromId(rep_contact_id), 
                        &connection)?; 
                    if contact_numbers.rows.len() > 0 {
                        json_object["payload"]["contact"]["numbers"] = set_json_object(
                            &contact_email, 
                            JsonStructType::TableColumn(1)
                        );
                    }
                }          
            }
        },
        Query::GETSuppliersCategories => {
            
            let categories = db_table_from_query(
                &query, 
                &connection)?;

            if categories.rows.len() == 0 {
                return Ok(json_object.dump());
            }

            json_object["payload"] = set_json_object(&categories, JsonStructType::Table);
        },
        Query::GETSupplierCategoriesFromId(_) => {
            
            let supply_categories = db_table_from_query(
                &query, 
                &connection)?;

            if supply_categories.rows.len() == 0 {
                return Ok(json_object.dump());
            }

            json_object["payload"] = set_json_object(&supply_categories, JsonStructType::Table);
        },
        Query::GETSupplyRepFromId(id) => {
            
            let rep = db_table_from_query(
                &query, 
                &connection)?;

            if rep.rows.len() == 0 {
                return Ok(json_object.dump());
            }

            json_object["payload"] = set_json_object(&rep, JsonStructType::Object);

            json_object["payload"].remove("contactId");

            let email = db_table_from_query(
                &Query::GETSupplyRepEmailFromId(id), 
                &connection)?;

            if email.rows.len() > 0 {
                json_object["payload"]["contact"]["email"] = set_json_object(
                    &email, 
                    JsonStructType::TableColumn(1)
                );
           
            }

            let numbers = db_table_from_query(
                &Query::GETSupplyRepPhoneNumbersFromId(id), 
                &connection)?;

            if numbers.rows.len() > 0 {
                json_object["payload"]["contact"]["numbers"] = set_json_object(
                    &numbers, 
                    JsonStructType::TableColumn(1)
                );
            }
        },

        Query::GETSupplyRepEmailFromId(_) => {
            
            let rep_email = db_table_from_query(
                &query, 
                &connection)?;

            if rep_email.rows.len() == 0 {
                return Ok(json_object.dump());
            }

            json_object["payload"] = set_json_object(&rep_email, JsonStructType::TableColumn(1));
        },
        Query::GETSupplyRepPhoneNumbersFromId(_) => {
            
            let rep_numbers = db_table_from_query(
                &query, 
                &connection)?;

            if rep_numbers.rows.len() == 0 {
                return Ok(json_object.dump());
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

