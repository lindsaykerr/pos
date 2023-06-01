use crate::errors::DatabaseError;
use crate::server::api::query_types::Query;
use crate::server::databases::data_structs::{DBTable, DBTableStruct, DBTableRow, DbFieldStruct, Value};
use sqlite::{self, Statement, State};

pub fn db_table_from_query(query_type: &Query, connection: &sqlite::Connection) -> Result<DBTable, DatabaseError> {

    // try connect to and query the db
    let query = sql_query(query_type);
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

 fn sql_query(query: &Query) -> String {
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
        Query::GETSuppliersEmail | Query::GETSupplierEmailFromId(_) => {
            let mut suppliers_email: DBTableStruct = DBTableStruct::new();
            suppliers_email.fields.push(
                DbFieldStruct::new(0, "supplierId", Value::Integer(0), true));
            suppliers_email.fields.push(
                DbFieldStruct::new(1, "email", Value::String(String::new()), true));
            suppliers_email
        },
        Query::GETSuppliersNumbers | Query::GETSupplierNumbersFromId(_) => {
            let mut suppliers_numbers: DBTableStruct = DBTableStruct::new();
            suppliers_numbers.fields.push(
                DbFieldStruct::new(0, "supplierId", Value::Integer(0), true));
            suppliers_numbers.fields.push(
                DbFieldStruct::new(1, "phone", Value::String(String::new()), true));

            suppliers_numbers
        },
        Query::GETSupplierIdFromName(_) => {
            let mut supplier_row: DBTableStruct = DBTableStruct::new();
            supplier_row.fields.push(
                DbFieldStruct::new(0, "id", Value::Integer(0), true));
            supplier_row
        },
        Query::GETSupplierNameFromId(_) => {
            let mut supplier_name: DBTableStruct = DBTableStruct::new();
            supplier_name.fields.push(
            DbFieldStruct::new(0, "name", Value::String(String::new()), true));
            supplier_name
        },
        Query::GETSupplierAddressFromId(_) => {
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
        Query::GETSupplierRepFromId(_) => {
            let mut rep: DBTableStruct = DBTableStruct::new();
            rep.fields.push(
            DbFieldStruct::new(0, "id", Value::Integer(0), true));
            rep.fields.push(
            DbFieldStruct::new(1, "title", Value::String(String::new()), true));
            rep.fields.push(
            DbFieldStruct::new(2, "firstName", Value::String(String::new()), true));
            rep.fields.push(
            DbFieldStruct::new(3, "lastName", Value::String(String::new()), true));
            rep.fields.push(
            DbFieldStruct::new(4, "contactId", Value::Integer(0), true));
            rep
        },
        Query::GETSuppliersCategories => {
            let mut  categories: DBTableStruct = DBTableStruct::new();
            categories.fields.push(
            DbFieldStruct::new(0, "id", Value::Integer(0), true));
            categories.fields.push(
            DbFieldStruct::new(1, "categoryType", Value::String(String::new()), true));
            categories

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
            let mut supply_categories: DBTableStruct = DBTableStruct::new();
            supply_categories.fields.push(
            DbFieldStruct::new(0, "categoryId", Value::Integer(0), true));
            supply_categories.fields.push(
            DbFieldStruct::new(1, "category", Value::String(String::new()), true));
            supply_categories
        },
        Query::GETSupplyRepPhoneNumbersFromId(_) => {
            let mut rep_numbers: DBTableStruct = DBTableStruct::new();
            rep_numbers.fields.push(
            DbFieldStruct::new(0, "id", Value::Integer(0), true));
            rep_numbers.fields.push(
            DbFieldStruct::new(1, "number", Value::String(String::new()), true));
            rep_numbers
        },
        Query::GETSupplyRepEmailFromId(_) => {
            let mut rep_email: DBTableStruct = DBTableStruct::new();
            rep_email.fields.push(
            DbFieldStruct::new(0, "id", Value::Integer(0), true));
            rep_email.fields.push(
            DbFieldStruct::new(1, "email", Value::String(String::new()), true));
            rep_email
        },
    
    _ => DBTableStruct::new()
}
}