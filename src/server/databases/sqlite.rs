use crate::server::api::Query;
use crate::errors::DatabaseError;
use crate::config::SQLITE_DB_PATH;
use sqlite::{State, Statement, Connection, RowIndex};
use json::{self, JsonValue};
use crate::server::databases::data_structs::{DBTable, DBTableRow, DBTableStruct,  DbFieldStruct, Value};

///
/// Queries the sqlite database and returns a response in the form of a json string.
///
pub fn query_for_sqlite_db(query: Query) -> Result<String, DatabaseError> {
    
    let database_path = SQLITE_DB_PATH;
    let connection = open_connection(&database_path)?;
 
    let mut json_object = json::object!{
        "code": 200,
        "success": true,
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

            let mut id: i64 = -1;
            let mut contact_id: i64 = -1;
            let mut address_id: i64 = -1;
            let mut rep_id: i64 = -1;

            if let Value::Integer(supplier_id) = supplier.rows[0].cells[0] {
                id = supplier_id;
            };

            if let Value::Integer(address) = supplier.rows[0].cells[3] {
                contact_id = address;
            };

            if let Value::Integer(contact) = supplier.rows[0].cells[4] {
                address_id = contact;
            }

            if let Value::Integer(rep) = supplier.rows[0].cells[5] {
                rep_id = rep;
            }
            
            json_object["payload"] = set_json_object(&supplier, JsonStructType::Object);


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
                &format!(r"SELECT
                    supply_rep.id, 
                    supply_rep.fk_person_title, 
                    supply_rep.FirstName,
                    supply_rep.LastName,
                FROM supply_rep, 
                
                
                (
                    SELECT 
                        supplier.fk_rep as RepID 
                        FROM supplier 
                        WHERE supplier.id = {}
                ) as sr 
                WHERE sr.RepID = rep.id; ", id)
            )?;

            if rep.rows.len() == 0 {
                return Ok(json_object.dump());
            }

            json_object["payload"] = set_json_object(&rep, JsonStructType::Object);
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
    Ok(json_object.dump())
}

pub enum JsonStructType {
    Table,
    Object,
    TableColumn(usize)
}

fn set_json_object(table: &DBTable, json_type: JsonStructType) -> JsonValue {
    let mut temp_json = table.to_json();

    match json_type {
        JsonStructType::Table => {
            return temp_json;
        },
        JsonStructType::Object => {
            if !temp_json.is_empty() {
                return temp_json[0].clone();
            }
            else {
                return JsonValue::Null;
            }
        }
        JsonStructType::TableColumn(column_index) => {
            let mut temp_json = JsonValue::new_array();
            for row in table.rows.iter() {
                let temp = row.cells[column_index].to_json();
                temp_json.push(temp);
            }
            return temp_json;
        }
    }
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

// This is used to call a DBRow struct that represents the expected column names 
// of the tables for a given query request. This is used to help map the sql to another 
// data type such as json 
fn db_tables(for_query: Query) -> DBTableStruct {
    match for_query {
        Query::GETStockSuppliers | Query::GETStockSupplierFromId(_) => {
            let mut suppliers: DBTableStruct = DBTableStruct::new();
            suppliers.fields.push(
                DbFieldStruct::new(0, "id", Value::Integer(0), true));
            suppliers.fields.push(
                DbFieldStruct::new(1, "name", Value::String(String::new()), true));
            suppliers.fields.push(
                DbFieldStruct::new(2, "active", Value::Integer(0), true));
            suppliers.fields.push(
                DbFieldStruct::new(3, "addressId", Value::Integer(0), false));
            suppliers.fields.push(
                DbFieldStruct::new(4, "contactId", Value::Integer(0), true));
            suppliers.fields.push(
                DbFieldStruct::new(5, "repId", Value::Integer(0), false));
            suppliers
        },
        Query::GetStockSuppliersEmail => {
            let mut suppliers_email: DBTableStruct = DBTableStruct::new();
            suppliers_email.fields.push(
                DbFieldStruct::new(0, "supplierId", Value::Integer(0), true));
            suppliers_email.fields.push(
                DbFieldStruct::new(1, "email", Value::String(String::new()), true));
            suppliers_email
        },
        Query::GetStockSuppliersNumbers => {
            let mut suppliers_numbers: DBTableStruct = DBTableStruct::new();
            suppliers_numbers.fields.push(
                DbFieldStruct::new(0, "supplierId", Value::Integer(0), true));
            suppliers_numbers.fields.push(
                DbFieldStruct::new(1, "phone", Value::String(String::new()), true));

            suppliers_numbers
        },
        Query::GetStockSupplierIdFromName(_) => {
            let mut supplier_row: DBTableStruct = DBTableStruct::new();
            supplier_row.fields.push(
                DbFieldStruct::new(0, "id", Value::Integer(0), true));
            supplier_row
        },
        Query::GETStockSupplierAddressFromId(_) => {
            let mut address: DBTableStruct = DBTableStruct::new();
            address.fields.push(
              DbFieldStruct::new(0, "id", Value::Integer(0), true));
            address.fields.push(
              DbFieldStruct::new(1, "line1", Value::String(String::new()), true));
            address.fields.push(
              DbFieldStruct::new(2, "line2", Value::String(String::new()), false));
            address.fields.push(
              DbFieldStruct::new(3, "town", Value::String(String::new()), true));
            address.fields.push(
              DbFieldStruct::new(4, "council", Value::String(String::new()), false));
            address.fields.push(
              DbFieldStruct::new(5, "postCode", Value::String(String::new()), true));
            address
        },
        Query::GETStockSupplierRepFromId(_) => {
            let mut rep: DBTableStruct = DBTableStruct::new();
            rep.fields.push(
              DbFieldStruct::new(0, "id", Value::Integer(0), true));
            rep.fields.push(
              DbFieldStruct::new(1, "title", Value::String(String::new()), true));
            rep.fields.push(
              DbFieldStruct::new(1, "firstName", Value::String(String::new()), true));
            rep.fields.push(
              DbFieldStruct::new(2, "lastName", Value::String(String::new()), true));
            rep.fields.push(
              DbFieldStruct::new(3, "contactId", Value::Integer(0), true));
            rep
        },
        Query::GETStockSuppliersCategories => {
            let mut  categories: DBTableStruct = DBTableStruct::new();
            categories.fields.push(
              DbFieldStruct::new(0, "id", Value::Integer(0), true));
            categories.fields.push(
              DbFieldStruct::new(1, "categoryType", Value::String(String::new()), true));
            categories

        },
        Query::GETStockSupplierSupplyCategories(_) => {
            let mut supply_categories: DBTableStruct = DBTableStruct::new();
            supply_categories.fields.push(
              DbFieldStruct::new(0, "categoryId", Value::Integer(0), true));
            supply_categories.fields.push(
              DbFieldStruct::new(1, "category", Value::String(String::new()), true));
            supply_categories
        },
       
       _ => DBTableStruct::new()
   }
}