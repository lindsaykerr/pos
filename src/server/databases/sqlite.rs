use crate::server::api::Query;
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
pub fn query_for_sqlite_db(query: Query) -> Result<String, DatabaseError> {
    
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

        },
        Query::GETStockSupplierFromId(id) => {
                
            let supplier = db_table_from_query(
                &query, 
                &connection, 
                &format!("SELECT * FROM view_suppliers WHERE id = {}", id)
            )?;

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
                &Query::GetStockSuppliersEmail, 
                &connection, 
                &format!("SELECT * FROM view_suppliers_email WHERE supplierId = {}", id)
            )?;

            if supplier_email.rows.len() > 0 {
                json_object["payload"]["contact"]["email"] = set_json_object(
                    &supplier_email, 
                    JsonStructType::TableColumn(1)
                );
            }
            
            // using the supplier id, get the associated contact phone numbers
            let contact_numbers = db_table_from_query(
                &Query::GetStockSuppliersNumbers, 
                &connection, 
                &format!("SELECT * FROM view_suppliers_numbers WHERE supplierId = {}", id)
            )?;
            
            if contact_numbers.rows.len() > 0 {
                json_object["payload"]["contact"]["numbers"] = set_json_object(
                    &contact_numbers, 
                    JsonStructType::TableColumn(1)
                );
            }


            // using the supplier id, get the supplier address information if any
            let address = db_table_from_query(
         
                &Query::GETStockSupplierAddressFromId(id),
                &connection, 
                &format!(r"SELECT
                    address.id, 
                    address.Line1, 
                    address.Line2,
                    address.Town,
                    address.Council,
                    address.Postcode
                FROM address, (
                    SELECT 
                        supplier.fk_address as AddressID 
                        FROM supplier 
                        WHERE supplier.id = {}
                ) as sa 
                WHERE sa.AddressID = address.id; ", id)
            )?;

            if address.rows.len() > 0 {
                json_object["payload"]["address"] = set_json_object(&address, JsonStructType::Object);
                json_object["payload"]["address"].remove("id");
            }
            
            // using the supplier id, get the supplier rep information if any
            let rep = db_table_from_query(
                &Query::GETStockSupplierRepFromId(id), 
                &connection, 
                &format!(r"SELECT
                    sr.id,
                    (SELECT 
                        title 
                    FROM person_title 
                    WHERE sr.fk_person_title = person_title.id
                    ) as Title, 
                    sr.FirstName,
                    sr.LastName,
                    sr.fk_contact as ContactID
                FROM supply_rep as sr,(
                    SELECT 
                        supplier.fk_supply_rep as RepID 
                    FROM supplier 
                    WHERE supplier.id = {}
            ) as s 
            WHERE s.RepID = sr.id", id)
            )?;

            if rep.rows.len() > 0 {
                json_object["payload"]["rep"] = set_json_object(&rep, JsonStructType::Object);
                json_object["payload"]["rep"].remove("id");
                json_object["payload"]["rep"].remove("contactId");
                let rep_contact_id: i64; 
                if let Value::Integer(x) = rep.rows[0].cells[4] {
                    rep_contact_id = x;
                } else {
                    return Err(DatabaseError::QueryError("Failed to get rep contact id".to_string()));
                }
                
                let contact_email = db_table_from_query(
                    &Query::GetStockSuppliersEmail, 
                    &connection, 
                    &format!("SELECT * FROM view_contact_email WHERE ContactId = {}", rep_contact_id)
                )?;            
   
                if contact_email.rows.len() > 0 {
                    json_object["payload"]["rep"]["contact"]["email"] = set_json_object(
                        &contact_email, 
                        JsonStructType::TableColumn(1)
                    );

                }

                let contact_numbers = db_table_from_query(
                    &Query::GetStockSuppliersNumbers, 
                    &connection, 
                    &format!("SELECT * FROM view_contact_numbers WHERE ContactId = {}", rep_contact_id)
                )?; 
                if contact_numbers.rows.len() > 0 {
                    json_object["payload"]["rep"]["contact"]["numbers"] = set_json_object(
                        &contact_email, 
                        JsonStructType::TableColumn(1)
                    );
                }           

            
            }

            
      
        },
        Query::GetStockSuppliersEmail => {
            
            let v_suppliers_email = db_table_from_query(
                &query, 
                &connection, 
                "SELECT * FROM view_suppliers_email"
            )?;

            if v_suppliers_email.rows.len() == 0 {
                return Ok(json_object.dump());
            }

            json_object["payload"] = set_json_object(&v_suppliers_email, JsonStructType::Table);
        },
        Query::GetStockSuppliersNumbers => {
            
            let v_suppliers_numbers = db_table_from_query(
                &query, 
                &connection, 
                "SELECT * FROM view_suppliers_numbers"
            )?;

            if v_suppliers_numbers.rows.len() == 0 {
                return Ok(json_object.dump());
            }

            json_object["payload"] = set_json_object(&v_suppliers_numbers, JsonStructType::Table);
        },
        Query::GetStockSupplierIdFromName(ref name) => {


            let id = db_table_from_query(
                &query, 
                &connection, 
                &format!("SELECT id FROM view_suppliers WHERE name = '{}'", name)
            )?;

            if id.rows.len() == 0 {
                return Ok(json_object.dump());
            }
            
            json_object["payload"] = set_json_object(&id, JsonStructType::Object);

        },

        Query::GETStockSupplierAddressFromId(id) => {
            
            let address = db_table_from_query(
                &query, 
                &connection, 
                &format!(r"SELECT
                    address.id, 
                    address.Line1, 
                    address.Line2,
                    address.Town,
                    address.Council,
                    address.Postcode
                FROM address, (
                    SELECT 
                        supplier.fk_address as AddressID 
                        FROM supplier 
                        WHERE supplier.id = {}
                ) as sa 
                WHERE sa.AddressID = address.id; ", id)
            )?;

            if address.rows.len() == 0 {
                return Ok(json_object.dump());
            }

            json_object["payload"] = set_json_object(&address, JsonStructType::Object);
        },

        Query::GETStockSupplierRepFromId(id) => {
            
            let rep = db_table_from_query(
                &query, 
                &connection, 
                &format!("SELECT
                    sr.id,
                    (SELECT 
                        title 
                    FROM person_title 
                    WHERE sr.fk_person_title = person_title.id
                    ) as Title, 
                    sr.FirstName,
                    sr.LastName,
                    sr.fk_contact as ContactID
                FROM supply_rep as sr,(
                    SELECT 
                        supplier.fk_supply_rep as RepID 
                    FROM supplier 
                    WHERE supplier.id = {}
            ) as s 
            WHERE s.RepID = sr.id", id))?;


            if rep.rows.len() > 0 {
                json_object["payload"] = set_json_object(&rep, JsonStructType::Object);
                json_object["payload"].remove("contactId");

                // using the contact id retrieve the contact email addresses and numbers
                let rep_contact_id: i64; 
                if let Value::Integer(x) = rep.rows[0].cells[4] {
                    rep_contact_id = x;
                } else {
                    return Err(DatabaseError::QueryError("Failed to get rep contact id".to_string()));
                }
                
                let contact_email = db_table_from_query(
                    &Query::GetStockSuppliersEmail, 
                    &connection, 
                    &format!("SELECT * FROM view_contact_email WHERE ContactId = {}", rep_contact_id)
                )?;            
   
                if contact_email.rows.len() > 0 {
                    json_object["payload"]["contact"]["email"] = set_json_object(
                        &contact_email, 
                        JsonStructType::TableColumn(1)
                    );

                }

                let contact_numbers = db_table_from_query(
                    &Query::GetStockSuppliersNumbers, 
                    &connection, 
                    &format!("SELECT * FROM view_contact_numbers WHERE ContactId = {}", rep_contact_id)
                )?; 
                if contact_numbers.rows.len() > 0 {
                    json_object["payload"]["contact"]["numbers"] = set_json_object(
                        &contact_email, 
                        JsonStructType::TableColumn(1)
                    );
                }           

            
            }
        },
        Query::GETStockSuppliersCategories => {
            
            let categories = db_table_from_query(
                &query, 
                &connection, 
                "SELECT * FROM supply_categories"
            )?;

            if categories.rows.len() == 0 {
                return Ok(json_object.dump());
            }

            json_object["payload"] = set_json_object(&categories, JsonStructType::Table);
        },
        Query::GETStockSupplierSupplyCategories(id) => {
            
            let supply_categories = db_table_from_query(
                &query, 
                &connection, 
                &format!(r"SELECT 
                  s.fk_supply_category as CategoryID,
                  c.Type as Category
                FROM supplier_supplies as s 
                LEFT JOIN supply_categories as c 
                on s.fk_supply_category = c.id
                where s.fk_supplier = {}", id)
            )?;

            if supply_categories.rows.len() == 0 {
                return Ok(json_object.dump());
            }

            json_object["payload"] = set_json_object(&supply_categories, JsonStructType::Table);
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

