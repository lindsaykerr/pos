use crate::errors::DatabaseError;
use crate::server::api::query_types::{Query, ContentFormat};
use sqlite;
// use crate::server::databases::sqlite::sqlite_tables;

pub fn post_request_to_db(query: &Query, connection: &sqlite::Connection) -> Result<(), DatabaseError> {
    
    /*match query {
        Query::POSTSupplier(content) => {
            if let ContentFormat::Json(content) = content {
                let json_content = content.clone();

                // get table structure
                let supplier_table = sqlite_tables::supplier_table();
                
                // certain fields are not required when creating a new supplier
                // as they will be generated by the database
                supplier_table.fields[0].not_null = false;
                supplier_table.fields[3].not_null = false;
                supplier_table.fields[4].not_null = false;
                supplier_table.fields[5].not_null = false;

                // check that all required fields are present in the json object
                for field in supplier_table.fields {
                    if !(field.not_null && json_content.has_key(field.name.as_str())) {
                        println!("field {} is required from passed json object", field.name.as_str());
                        return Err(DatabaseError::SubmissionError("Required field missing from json object".to_string()));
                    }
                }
                // it should now be safe to unwrap the json values
                let mut active = false;
                
                let name_value = json_content[supplier_table.fields[1].name].as_str().unwrap().parse::<String>();
                let name = match name_value {
                    Ok(name) => name,
                    Err(_e) => {
                        println!("Error parsing name from json object");
                        return Err(DatabaseError::SubmissionError("Error parsing name from json object".to_string()));
                    }
                };

                let active_value = json_content[supplier_table.fields[2].name].as_bool();
                let active = match active_value{ 
                    Some(active) => active,
                    None => {
                        println!("Error parsing active from json object");
                        return Err(DatabaseError::SubmissionError("Error parsing active from json object".to_string()));
                    }
                };

                let sql_statment = format!("INSERT INTO supplier (name, active) VALUES ('{}', {})", name, active);
                let supplier_added = connection.prepare(sql_statment.as_str());
                if let Err(_e) = supplier_added {
                    println!("Error preparing sql statement");
                    return Err(DatabaseError::SubmissionError("Error preparing sql statement".to_string()));
                }

                if json_content.has_key("address") {

                    let address_table = sqlite_tables::address_table();
                    address_table.fields[0].not_null = false;
                    address_table.fields[1].not_null = false;
                    address_table.fields[2].not_null = false;
                    address_table.fields[3].not_null = false;
                    address_table.fields[4].not_null = false;
                    address_table.fields[5].not_null = false;

                    let address = json_content["address"];
                    let line1 = address[address_table.fields[1].name.as_str()].as_str().unwrap().parse::<String>();
                    let line2 = address[address_table.fields[2].name.as_str()].as_str().unwrap().parse::<String>();
                    let town = address[address_table.fields[3].name.as_str()].as_str().unwrap().parse::<String>();
                    let council = address[address_table.fields[4].name.as_str()].as_str().unwrap().parse::<String>();
                    let postcode = address[address_table.fields[5].name.as_str()].as_str().unwrap().parse::<String>();

                    let sql_statment = format!("INSERT INTO address (Line1, Line2, Town, Council, Postcode) VALUES ('{}', '{}', '{}', '{}', '{}')", line1.unwrap(), line2.unwrap(), town.unwrap(), council.unwrap(), postcode.unwrap());
                    let address_added = connection.prepare(sql_statment.as_str());
                    if let Err(_e) = address_added {
                        println!("Error preparing sql statement");
                        return Err(DatabaseError::SubmissionError("Error preparing sql statement".to_string()));
                    }
                }




                
            }


        },
        _ => {}
    }
    */
    Ok(())

 
 }