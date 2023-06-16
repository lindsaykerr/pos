use crate::server::{api::query_types::Query, databases::data_structs::Value};
use std::collections::HashMap;
use crate::server::databases::config::data_keys;


pub fn post_sql(query: Query, values: &HashMap<String, Value>) -> Option<String> {
    match query {
        Query::POSTSupplier(_) => {

            let name = values.get(data_keys::NAME).expect("No name field in post query");
            let name = if let Value::String(n) = name  {
                n
            } else {
                return None;
            };

            let active = values.get(data_keys::ACTIVE).expect("No active field in post query");

            let active = if let Value::Boolean(a) = active {
                if *a == true { 1 } else { 0 }
            } else {
                return None;
            };

            let sql = format!("INSERT INTO supplier (name, active) VALUES ('{}', {})", name, active);
            Some(sql)
        },
        Query::POSTAddress(_) => {

            let mut address_line1: String = String::from("Null");
            let mut address_line2: String = String::from("Null");
            let mut address_town: String = String::from("Null");
            let mut address_county: String = String::from("Null");
            let mut address_postcode: String = String::from("Null");

            if values.get(data_keys::ADDRESS_LINE1).is_some() {
                let line1_value = values.get(data_keys::ADDRESS_LINE1).expect("No address_line1 field in post query");
                address_line1 = if let Value::String(a) = line1_value {
                    a.clone()
                }
                else {
                    String::from("Null")
                };
            }
            
            if values.get(data_keys::ADDRESS_LINE2).is_some() {
                let line2_value = values.get(data_keys::ADDRESS_LINE2).expect("No address_line2 field in post query");
                address_line2 = if let Value::String(a) = line2_value {
                    a.clone()
                }
                else {
                    String::from("Null")
                };
            }

            if values.get(data_keys::ADDRESS_TOWN).is_some() {
                let town_value = values.get(data_keys::ADDRESS_TOWN).expect("No address_town field in post query");
                address_town = if let Value::String(a) = town_value {
                    a.clone()
                }
                else {
                    String::from("Null")
                };
            }

            if values.get(data_keys::ADDRESS_COUNCIL).is_some() {
                let county_value = values.get(data_keys::ADDRESS_COUNCIL).expect("No address_county field in post query");
                address_county = if let Value::String(a) = county_value {
                    a.clone()
                }
                else {
                    String::from("Null")
                };
            }

            if values.get(data_keys::ADDRESS_POSTCODE).is_some() {
                let postcode_value = values.get(data_keys::ADDRESS_POSTCODE).expect("No address_postcode field in post query");
                address_postcode = if let Value::String(a) = postcode_value {
                    a.clone()
                }
                else {
                    String::from("Null")
                };
            }

            // TODO: this check might be placed before this function is called
            if address_line1 == "Null" || address_town == "Null" || address_postcode == "Null" {
                return None;
            }

            let sql = format!("INSERT INTO address (address_line1, address_line2, address_town, address_county, address_postcode) VALUES ('{}', '{}', '{}', '{}', '{}')", 
            address_line1, 
            address_line2, 
            address_town, 
            address_county, 
            address_postcode);
            Some(sql)

        }
        Query::POSTContactEmails(_) => {

            if values.is_empty() {
                return None;
            }

            

            let mut sql = String::from("INSERT INTO contact_email (email) VALUES ");
            for (i, value) in values.iter().enumerate() {
                let (key, value) = value;
                if let Value::String(email) = value {
                    sql.push_str(&format!("('{}')", email));
                    if i+1 < values.len() {
                        sql.push_str(", ");
                    }
                }
            }
            Some(sql)

        },
        Query::POSTContactPhoneNumbers(_) => {
            if values.is_empty() {
                return None;
            }

            

            let mut sql = String::from("INSERT INTO contact_phone_number (phone_number) VALUES ");
            for (i, value) in values.iter().enumerate() {
                let (key, value) = value;
                if let Value::String(phone_number) = value {
                    sql.push_str(&format!("('{}')", phone_number));
                    if i+1 < values.len() {
                        sql.push_str(", ");
                    }
                }
            }
            Some(sql)
        }
        _ => {
            panic!("Invalid query type passed to post_query");
        }
    }

}