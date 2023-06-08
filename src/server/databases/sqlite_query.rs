use crate::errors::DatabaseError;
use crate::server::api::query_types::{Query, ContentFormat};
use crate::server::databases::data_structs::{DBTable, DBTableStruct, DBTableRow, DbFieldStruct, Value};
use crate::server::databases::sqlite_tables;
use sqlite::{self, Statement, State};

pub fn db_table_from_query(query_type: &Query, connection: &sqlite::Connection) -> Result<DBTable, DatabaseError> {

    // try connect to and query the db
    let query = get_sql_query(query_type);
    //println!("sql query: {}", query);
    let statement_result = connection.prepare(query.as_str());
    if let Err(_e) = statement_result {
        println!("sqlite_DBTable_from_query connection.prepare failed");
        return Err(DatabaseError::QueryError("Sqlite db query stockItems failed".to_string()));
    }
    let statement = statement_result.unwrap();
 
    // gets the table structure for this query
    
    let v_suppliers_row_struct = db_tables(query_type.clone()); 
    let table_v_suppliers = response_data_into_db_table(statement, v_suppliers_row_struct);
 
    
    Ok(table_v_suppliers)
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

 pub fn post_request_to_db(query: &Query, connection: &sqlite::Connection) -> Result<(), DatabaseError> {
    match query {
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

                    let address = json_content["address"].as_object().unwrap();
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
    ()

 
 }

 fn get_sql_query(query: &Query) -> String {



        match query {
            // suppliers
            Query::GETSuppliers => {
                "SELECT * FROM supplier".to_string()
            },
            Query::GETSuppliersEmail => {
                "SELECT * FROM view_suppliers_email".to_string()
            },
            Query::GETSuppliersNumbers => {
                "SELECT * FROM view_suppliers_numbers".to_string()
            },
            Query::GETSuppliersCategories => {
                "SELECT * FROM supply_categories".to_string()
            },

            // supplier by id
            Query::GETSupplierNameFromId(id) => {
                format!("SELECT name FROM supplier WHERE id = {}", id)
            },
            Query::GETSupplierFromId(id) => {
                format!("SELECT * FROM view_suppliers WHERE id = {}", id)
            },
            Query::GETSupplierIdFromName(name) => {
                format!("SELECT id FROM supplier WHERE name = '{}'", name)
            },
            Query::GETSupplierEmailFromId(id) => {
                format!("SELECT * FROM view_suppliers_email WHERE supplierId = {}", id)
            },
            Query::GETSupplierNumbersFromId(id) => {
                format!("SELECT * FROM view_suppliers_numbers WHERE supplierId = {}", id)
            },
            Query::GETSupplierAddressFromId(id) => {
                format!(r"SELECT
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
            },

            Query::GETSupplierCategoriesFromId(id) => {
                format!(r"SELECT 
                    s.fk_supply_category as CategoryID,
                    c.Type as Category
                FROM supplier_supplies as s 
                LEFT JOIN supply_categories as c 
                ON s.fk_supply_category = c.id
                WHERE s.fk_supplier = {}", id)
            },
            Query::GETSupplierRepFromId(id) => {
                format!(r"SELECT
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
            },


            // supplier rep
            Query::GETSupplyRepFromId(id) => {
                format!(r"SELECT
                (SELECT 
                    title 
                FROM person_title 
                WHERE sr.fk_person_title = person_title.id
                ) as Title, 
                sr.FirstName,
                sr.LastName,
                sr.fk_contact as ContactID
            FROM supply_rep as sr
            WHERE sr.id = {}", id)
            },
            Query::GETSupplyRepPhoneNumbersFromId(id) => {
                format!(r"SELECT 
                    c.SupplyRepID, 
                    c.Number
                    FROM view_supply_rep_numbers as c
                    WHERE c.supplyRepID = {}", id)
            },
            Query::GETSupplyRepEmailFromId(id) => {
                format!(r"SELECT 
                    c.SupplyRepID, 
                    c.Email
                    FROM view_supply_rep_email as c
                    WHERE c.supplyRepID = {}", id)
            },

            _ => "".to_string()
        }
 }
 
// This is used to call a DBRow struct that represents the expected column names 
// of the tables for a given query request. This is used to help map the sql to another 
// data type such as json 
fn db_tables(for_query: Query) -> DBTableStruct {
    match for_query {
        Query::GETSuppliers | Query::GETSupplierFromId(_) => {
            sqlite_tables::supplier_table()
        },
        Query::GETSuppliersEmail | Query::GETSupplierEmailFromId(_) => {
            sqlite_tables::email_table()
        },
        Query::GETSuppliersNumbers | Query::GETSupplierNumbersFromId(_) => {
            sqlite_tables::numbers_table()
        },
        Query::GETSupplierIdFromName(_) => {
            sqlite_tables::id_table()
        },
        Query::GETSupplierNameFromId(_) => {
            sqlite_tables::supplier_name_table()
        },
    
        Query::GETSupplierAddressFromId(_) => {
            sqlite_tables::address_table()
        },

        Query::GETSupplierRepFromId(_) => {
            sqlite_tables::rep_table()
        },
        Query::GETSuppliersCategories => {
            sqlite_tables::categories_table()

        },
        Query::GETSupplyRepFromId(_) => {
            let mut rep: DBTableStruct = DBTableStruct::new();
            
            rep.fields.push(
            DbFieldStruct::new(0, "title", Value::String(String::new()), true));
            rep.fields.push(
            DbFieldStruct::new(1, "firstName", Value::String(String::new()), true));
            rep.fields.push(
            DbFieldStruct::new(2, "lastName", Value::String(String::new()), true));
            rep.fields.push(
            DbFieldStruct::new(3, "contactId", Value::Integer(0), true));
            rep
        },
        Query::GETSupplierCategoriesFromId(_) => {
            sqlite_tables::categories_table()
        },
        Query::GETSupplyRepPhoneNumbersFromId(_) => {
            sqlite_tables::numbers_table()
        },
        Query::GETSupplyRepEmailFromId(_) => {
            sqlite_tables::email_table()
        },
    
    _ => DBTableStruct::new()
    }
}








