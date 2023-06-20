use crate::errors::DatabaseError;
use crate::server::api::query_types::{
    Query, Content};
use crate::server::databases::data_structs::{DBTableStruct, Value};

use std::collections::HashMap;
use json::JsonValue;
use crate::config::SQLITE_DB_PATH;
use sqlite;
use crate::server::databases::{
    sqlite::{
        util::open_connection,
        sqlite_tables,
        post_sql_queries,
    },
    config::data_keys,
    
};

// use crate::server::databases::sqlite::sqlite_tables;

pub fn process_query(query: Query, body_content: JsonValue) -> Result<JsonValue, DatabaseError> {
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
            if let Ok(value_map) = extract_json_to_table(&body_content, supplier_table) {

                // using those values build the SQL insert statement
                let sql_insert_supplier = post_sql_queries::post_sql(query, &value_map);

                // execute the SQL statement and notify the user of if successful or not

                if let Some(sql) = sql_insert_supplier {
                    if let Err(_) = connection.execute(sql) {
                        return Err(DatabaseError::SubmissionError("Failed to insert supplier".to_string()));
                    }
                    else {
                        json_object["success"] = json::JsonValue::Boolean(false);
                        json_object["message"] = json::JsonValue::String("Insertion failure".to_string());
                        return Ok(json_object);
                    }
                }

                // We will need the the db supplier id for the remainder of the process
                let name = value_map.get(data_keys::NAME).unwrap();
                let name: String = if let Value::String(n) = name  {
                    n.clone()
                }
                else {
                    String::new()
                };
                let mut statement = connection.prepare(r"SELECT id, fk_contact as ContactID FROM supplier");
                if let Err(_) = statement {
                    return Err(DatabaseError::QueryError("Failed to get supplier id".to_string()));
                }

                let supplier_id = &statement.as_ref().unwrap().read::<i64, _>(0).unwrap();
                let contact_id = &statement.as_ref().unwrap().read::<i64, _>(1).unwrap();
       


                if body_content[data_keys::ADDRESS] != JsonValue::Null {
                    insert_address(Query::POSTAddress(Content::None), &body_content[data_keys::ADDRESS], &connection)?;
                    update_supplier_address_id(*supplier_id, &connection, UpdateOnId::None)?;
                }


                if body_content[data_keys::CONTACT] != JsonValue::Null {
                    if body_content[data_keys::CONTACT][data_keys::EMAIL] != JsonValue::Null {
                        let email_map = insert_email_addresses(Query::POSTContactEmails(Content::None), &body_content[data_keys::CONTACT][data_keys::EMAIL], &connection)?;
                        insert_email_contact(email_map, contact_id, &connection)?;
                    }
                    if body_content[data_keys::CONTACT][data_keys::NUMBER] != JsonValue::Null {
                        let numbers_map = insert_phone_numbers(Query::POSTContactPhoneNumbers(Content::None), &body_content[data_keys::CONTACT][data_keys::NUMBER], &connection)?;
                        insert_phone_contact(numbers_map, contact_id, &connection)?;
                    }   
                }

                if body_content[data_keys::REP] != JsonValue::Null {
                    insert_representative(Query::POSTRep(Content::None), &body_content[data_keys::REP], &connection)?;
                    update_supplier_rep_id(*supplier_id, &connection, UpdateOnId::None)?;
                }
            }


        },

        Query::POSTAddress(_) => {
            insert_address(query, &body_content, &connection)?;
            json_object["success"] = json::JsonValue::Boolean(true);
            json_object["message"] = json::JsonValue::String("Insertion success".to_string());
        }
        _ => {
            return Err(DatabaseError::SubmissionError("Invalid query type".to_string()));
        }
    }

    Ok(json_object)
}

fn insert_address(query: Query, body_content: &JsonValue, connection: &sqlite::Connection) -> Result<(), DatabaseError> {
    
    if !body_content.has_key(data_keys::ADDRESS) {
        return Err(DatabaseError::SubmissionError("No address data".to_string()));
    }

    let address_table = sqlite_tables::post_tables(Query::POSTAddress(Content::None));
    let address_values = extract_json_to_table(&body_content[data_keys::ADDRESS], address_table)?;

    let sql_insert_address = post_sql_queries::post_sql(Query::POSTAddress(Content::None), &address_values);

    // insert the address details into the database
    if let Some(sql) = sql_insert_address {
        if let Err(_) = connection.execute(sql) {
            return Err(DatabaseError::SubmissionError("Failed to insert supplier address".to_string()));
        }

        Ok(())   
    } else {
        return Err(DatabaseError::SubmissionError("Invalid SQL statement, could not add address info".to_string()));
    }

}

fn update_supplier_address_id(supplier_id: i64, connection: &sqlite::Connection, address_id: UpdateOnId) -> Result<(), DatabaseError> {
    
    let mut add_id: i64;

    if let UpdateOnId::Id(x) = address_id {
        add_id = x;
    } else {
        let address_id = connection.prepare(format!{"SELECT MAX(id) as id FROM address", }.as_str());
        if let Err(_) = address_id {
            return Err(DatabaseError::QueryError("Failed to get address id".to_string()));
        }
        add_id = address_id.unwrap().read::<i64, _>(0).unwrap();
    }

    // SQL statement to update the supplier table with the address id
    let sql_statement = format!{"UPDATE supplier SET fk_address = {} WHERE id = {}", add_id, supplier_id};

    if let Err(_) = connection.execute(sql_statement.as_str()) {
        return Err(DatabaseError::SubmissionError("Failed to update supplier address".to_string()));
    }

    Ok(())
}



fn insert_email_addresses(query: Query, body_content: &JsonValue, connection: &sqlite::Connection) -> Result<HashMap<i64, String>, DatabaseError> {
    if !body_content.has_key(data_keys::EMAIL) {
        return Err(DatabaseError::SubmissionError("No contact data".to_string()));
    }
    let mut body_content = body_content[data_keys::EMAIL].clone();
    
    // there is a chance that more than one contact email address will be submitted
    let mut emails: HashMap<String, Value> = HashMap::new();
    if body_content.is_array() {
        let mut i = 0;
        loop {

            let email: JsonValue = body_content.pop();
            if email == JsonValue::Null {
                break;
            }
            i += 1;
            let email = email.as_str().unwrap().to_string();
            emails.insert(i.to_string(), Value::String(email));
        }
    }
    else if body_content.is_string() {
        let email = body_content.as_str().unwrap().to_string();
        emails.insert(0.to_string(), Value::String(email));
    }
    else {
        return Err(DatabaseError::SubmissionError("Invalid email address data type".to_string()));
    }


    // submit emails to the database 
    let sql_statement = post_sql_queries::post_sql(query, &emails);

    if let Some(sql) = sql_statement {

        if let Err(_) = connection.execute(sql) {
            return Err(DatabaseError::SubmissionError("Failed to insert email addresses".to_string()));
        }
    } else {
        return Err(DatabaseError::SubmissionError("Invalid SQL statement, could not add email info".to_string()));
    }

    // check if the email address have been added to the database
    // 1. Build the SQL statement
    let mut sql_find = String::from("SELECT id, Email FROM emails WHERE Email IN (\"");
    for value in emails.into_iter() {
        if let Value::String(x) = value.1 {
            sql_find.push_str(x.as_str());
            sql_find.push_str("\",\"");
        }
    }

    // 2. Execute the SQL statement
    let query_result = connection.prepare(sql_find.as_str());
    if let Err(_) = query_result {
        return Err(DatabaseError::QueryError("Failed to get email addresses".to_string()));
    }

    // 3. Get the results of the query and return them
    let mut email_map = HashMap::new();
    let result = query_result.into_iter();
    for row in result {
        let email_id = row.read::<i64, _>(0).unwrap();
        let email = row.read::<String, _>(1).unwrap();
        email_map.insert(email_id, email);
    }
    Ok(email_map)
}

fn insert_email_contact(email_map: HashMap<i64, String>, contact_id: &i64, connection: &sqlite::Connection) -> Result<(), DatabaseError> {
    let mut sql_statement = String::from("INSERT INTO contact_email (fk_email_addresses, fk_contact) VALUES ");
    for value in email_map.into_iter() {
        sql_statement.push_str(format!("({}, {}),", value.0, contact_id).as_str());
    }
    sql_statement.pop();
    sql_statement.push(';');

    if let Err(_) = connection.execute(sql_statement.as_str()) {
        return Err(DatabaseError::SubmissionError("Failed to insert email contact".to_string()));
    }

    Ok(())
}

fn insert_phone_numbers(query: Query, body_content: &JsonValue, connection: &sqlite::Connection) -> Result<HashMap<i64, String>, DatabaseError> {
    if !body_content.has_key(data_keys::NUMBER) {
        return Err(DatabaseError::SubmissionError("No contact data".to_string()));
    }
    let mut body_content = body_content[data_keys::NUMBER].clone();
    
    // there is a chance that more than one contact email address will be submitted
    let mut numbers: HashMap<String, Value> = HashMap::new();
    if body_content.is_array() {
        let mut i = 0;
        loop {

            let number: JsonValue = body_content.pop();
            if number == JsonValue::Null {
                break;
            }
            i += 1;
            let number = number.as_str().unwrap().to_string();
            numbers.insert(i.to_string(), Value::String(number));
        }
    }
    else if body_content.is_string() {
        let number = body_content.as_str().unwrap().to_string();
        numbers.insert(0.to_string(), Value::String(number));
    }
    else {
        return Err(DatabaseError::SubmissionError("Invalid email address data type".to_string()));
    }

    // submit numbers to the database

    let sql_statement = post_sql_queries::post_sql(query, &numbers);

    if let Some(sql) = sql_statement {

        if let Err(_) = connection.execute(sql) {
            return Err(DatabaseError::SubmissionError("Failed to insert phone numbers".to_string()));
        }
    } else {
        return Err(DatabaseError::SubmissionError("Invalid SQL statement, could not add phone number info".to_string()));
    }

    // check if the email address have been added to the database

    // 1. Build the SQL statement
    let mut sql_find = String::from("SELECT id, Number FROM phone_numbers WHERE Number IN (\"");
    for value in numbers.into_iter() {
        if let Value::String(x) = value.1 {
            sql_find.push_str(x.as_str());
            sql_find.push_str("\",\"");
        }
    }

    // 2. Execute the SQL statement
    let query_result = connection.prepare(sql_find.as_str());
    if let Err(_) = query_result {
        return Err(DatabaseError::QueryError("Failed to get phone numbers".to_string()));
    }

    // 3. Get the results of the query and return them
    let mut number_map = HashMap::new();
    let result = query_result.into_iter();
    for row in result {
        let number_id = row.read::<i64, _>(0).unwrap();
        let number = row.read::<String, _>(1).unwrap();
        number_map.insert(number_id, number);
    }
    Ok(number_map)

}

fn insert_phone_contact(number_map: HashMap<i64, String>, contact_id: &i64, connection: &sqlite::Connection) -> Result<(), DatabaseError> {
    let mut sql_statement = String::from("INSERT INTO contact_phone (fk_phone_number, fk_contact) VALUES ");
    for value in number_map.into_iter() {
        sql_statement.push_str(format!("({}, {}),", value.0, contact_id).as_str());
    }
    sql_statement.pop();
    sql_statement.push(';');

    if let Err(_) = connection.execute(sql_statement.as_str()) {
        return Err(DatabaseError::SubmissionError("Failed to insert phone contact".to_string()));
    }

    Ok(())
}

fn insert_representative(query: Query, body_content: &JsonValue, connection: &sqlite::Connection) -> Result<(), DatabaseError> {
    let rep_table = sqlite_tables::post_tables(query);
    let rep_values = extract_json_to_table(&body_content[data_keys::REP], rep_table)?;

    let sql_insert_rep = post_sql_queries::post_sql(Query::POSTRep(Content::None), &rep_values);

    // insert the address details into the database
    if let Some(sql) = sql_insert_rep {
        if let Err(_) = connection.execute(sql) {
            return Err(DatabaseError::SubmissionError("Failed to insert representative information".to_string()));
        }

        Ok(())   
    } else {
        return Err(DatabaseError::SubmissionError("Invalid SQL statement, could not add representative info".to_string()));
    }

}

fn update_supplier_rep_id(supplier_id: i64, connection: &sqlite::Connection, rep_id: UpdateOnId) -> Result<(), DatabaseError> {
    
    let mut id: i64;

    if let UpdateOnId::Id(x) = rep_id {
        id = x;
    } else {
        let rep_id = connection.prepare(format!{"SELECT MAX(id) as id FROM representative", }.as_str());
        if let Err(_) = rep_id {
            return Err(DatabaseError::QueryError("Failed to get representative id".to_string()));
        }
        id = rep_id.unwrap().read::<i64, _>(0).unwrap();
    }

    // SQL statement to update the supplier table with the address id
    let sql_statement = format!{"UPDATE supplier SET fk_supply_rep = {} WHERE id = {}", id, supplier_id};

    if let Err(_) = connection.execute(sql_statement.as_str()) {
        return Err(DatabaseError::SubmissionError("Failed to update supplier representative".to_string()));
    }

    Ok(())
}



pub enum UpdateOnId {
    Id(i64),
    None,
}







// uses a json object to populate a hashmap that represents a the value for a entry on a SQL table
pub fn extract_json_to_table(data: &JsonValue, table_ref: DBTableStruct) -> Result<HashMap<String, Value>, DatabaseError> {


    let mut table = HashMap::new();
    
    // loop through the fields in the table structure, they hold the expected type and name of a SQL field
    for field in table_ref.fields {
        let field_name = field.name.clone();
        // check if the json object has a valid field 
        if data.has_key(&field_name) {

            // get the value of the field from the json object
            let value = &data[&field_name];
            match field.field_type {
                Value::Integer(_) => {
                    // parse the value into the expected type
                    let value = value.as_i64();
                    if let Some(value) = value {
                        table.insert(field_name, Value::Integer(value));
                    } else {
                        return Err(DatabaseError::SubmissionError("Error parsing integer from json object".to_string()));
                    }
                },
                Value::String(_) => {
                    let value = value.as_str();
                    if let Some(value) = value {
                        table.insert(field_name,Value::String(value.to_string()));
                    } else {
                        return Err(DatabaseError::SubmissionError("Error parsing string from json object".to_string()));
                    }
                },
                Value::Binary(_) => {
                    let value = value.as_str();
                    if let Some(value) = value {
                        let value: Vec<u8> = value.as_bytes().to_vec();
               
                        table.insert(field_name, Value::Binary(value));
              
                    } else {
                        return Err(DatabaseError::SubmissionError("Error parsing binary from json object".to_string()));
                    }
                },
                Value::Boolean(_) => {
                    let value = value.as_bool();
                    if let Some(value) = value {
                        table.insert(field_name, Value::Boolean(value));
                    } else {
                        return Err(DatabaseError::SubmissionError("Error parsing boolean from json object".to_string()));
                    }
                },
                Value::Float(_) => {
                    let value = value.as_f64();
                    if let Some(value) = value {
                        table.insert(field_name, Value::Float(value));
                    } else {
                        return Err(DatabaseError::SubmissionError("Error parsing float from json object".to_string()));
                    }
                },
                Value::Null => {
                    table.insert(field_name, Value::Null);
                }

            }
        } else {
            if field.not_null {
                return Err(DatabaseError::SubmissionError("Required field missing from json object".to_string()));
            }
        }
    } 
   Ok(table)   
}