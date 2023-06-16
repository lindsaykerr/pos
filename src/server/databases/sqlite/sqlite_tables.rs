use crate::server::databases::data_structs::{DBTableStruct, DbFieldStruct};
use crate::server::databases::data_structs::Value;
use crate::server::api::query_types::Query;

use crate::server::databases::config::data_keys;

pub fn post_tables(for_query: Query) -> DBTableStruct {
    match for_query {
        Query::POSTSupplier(_) => {
            let mut supplier = supplier_table();
            supplier.fields.remove(0); // Remove id field
            supplier.fields.remove(3); // Remove address_id field (can be null)
            supplier.fields.remove(4); // Remove contact_id field (auto generated on db)
            supplier.fields.remove(5); // Remove rep_id field (can be null)
            supplier
            
        },
        Query::POSTAddress(_) => {
            let mut address = address_table();
            address.fields.remove(0); // Remove id field
            address
        },
        Query::POSTContactEmails(_) => {
            let mut emails = email_table();
            emails.fields.remove(0); // Remove id field
            emails
        },
        Query::POSTContactPhoneNumbers(_) => {
            let mut numbers = numbers_table();
            numbers.fields.remove(0); // Remove id field
            numbers
        },
        _ => DBTableStruct::new()
    }
}



// Get the correct table structure for GET requests
pub fn get_tables(for_query: Query) -> DBTableStruct {

    match for_query {
        Query::GETSuppliers | Query::GETSupplierFromId(_) => {
            supplier_table()
        },
        Query::GETSuppliersEmail | Query::GETSupplierEmailFromId(_) => {
            email_table()
        },
        Query::GETSuppliersNumbers | Query::GETSupplierNumbersFromId(_) => {
            numbers_table()
        },
        Query::GETSupplierIdFromName(_) => {
            id_table()
        },
        Query::GETSupplierNameFromId(_) => {
            supplier_name_table()
        },
    
        Query::GETSupplierAddressFromId(_) => {
            address_table()
        },

        Query::GETSupplierRepFromId(_) => {
            rep_table()
        },
        Query::GETSuppliersCategories => {
            categories_table()

        },
        Query::GETSupplyRepFromId(_) => {
            let mut rep: DBTableStruct = DBTableStruct::new();
            
            rep.fields.push(
            DbFieldStruct::new(0, data_keys::TITLE, Value::String(String::new()), true));
            rep.fields.push(
            DbFieldStruct::new(1, data_keys::FIRST_NAME, Value::String(String::new()), true));
            rep.fields.push(
            DbFieldStruct::new(2, data_keys::LAST_NAME, Value::String(String::new()), true));
            rep.fields.push(
            DbFieldStruct::new(3, data_keys::CONTACT_ID, Value::Integer(0), true));
            rep
        },
        Query::GETSupplierCategoriesFromId(_) => {
            categories_table()
        },
        Query::GETSupplyRepPhoneNumbersFromId(_) => {
            numbers_table()
        },
        Query::GETSupplyRepEmailFromId(_) => {
            email_table()
        },
    
    _ => DBTableStruct::new()
    }
}

// Tables for the sqlite database, are represented by DBTableStruct's
// held within each table function.
//
// They represent the tables and views that can be called from the database
// Each table has a set of fields that relate to a tables column.
// In doing so the correct table column properties found within the db can be 
// mapped to the correct field in the struct. The field name given is that of
// the expected json object key, and not the column name in the database. 
//
// These tables can be used to validate incoming json objects, and format outgoing
// ones.

pub fn supplier_table() -> DBTableStruct {
    let mut suppliers: DBTableStruct = DBTableStruct::new();
    suppliers.fields.push(
        DbFieldStruct::new(0, data_keys::ID, Value::Integer(0), true));
    suppliers.fields.push(
        DbFieldStruct::new(1, data_keys::NAME, Value::String(String::new()), true));
    suppliers.fields.push(
        DbFieldStruct::new(2, data_keys::ACTIVE, Value::Integer(0), true));
    suppliers.fields.push(
        DbFieldStruct::new(3, data_keys::ADDRESS_ID, Value::Integer(0), false));
    suppliers.fields.push(
        DbFieldStruct::new(4, data_keys::CONTACT_ID, Value::Integer(0), true));
    suppliers.fields.push(
        DbFieldStruct::new(5, data_keys::REP_ID, Value::Integer(0), false));
    suppliers
}

pub fn email_table() -> DBTableStruct {
    let mut suppliers_email: DBTableStruct = DBTableStruct::new();
    suppliers_email.fields.push(
        DbFieldStruct::new(0, data_keys::ID , Value::Integer(0), true));
    suppliers_email.fields.push(
        DbFieldStruct::new(1, data_keys::EMAIL, Value::String(String::new()), true));
    suppliers_email
}

pub fn numbers_table() -> DBTableStruct {
    let mut suppliers_numbers: DBTableStruct = DBTableStruct::new();
    suppliers_numbers.fields.push(
        DbFieldStruct::new(0, data_keys::ID, Value::Integer(0), true));
    suppliers_numbers.fields.push(
        DbFieldStruct::new(1, data_keys::NUMBER, Value::String(String::new()), true));
    suppliers_numbers
}

pub fn id_table() -> DBTableStruct {
    let mut id: DBTableStruct = DBTableStruct::new();
    id.fields.push(
        DbFieldStruct::new(0, data_keys::ID, Value::Integer(0), true));
    id
}

pub fn supplier_name_table() -> DBTableStruct {
    let mut supplier_name: DBTableStruct = DBTableStruct::new();
    supplier_name.fields.push(
        DbFieldStruct::new(0, data_keys::NAME, Value::String(String::new()), true));
    supplier_name
}

pub fn address_table() -> DBTableStruct {
    let mut address: DBTableStruct = DBTableStruct::new();
    address.fields.push(
        DbFieldStruct::new(0, data_keys::ID, Value::Integer(0), true));
    address.fields.push(
        DbFieldStruct::new(1, data_keys::ADDRESS_LINE1, Value::String(String::new()), true));
    address.fields.push(
        DbFieldStruct::new(2, data_keys::ADDRESS_LINE2, Value::String(String::new()), false));
    address.fields.push(
        DbFieldStruct::new(3, data_keys::ADDRESS_TOWN, Value::String(String::new()), true));
    address.fields.push(
        DbFieldStruct::new(4, data_keys::ADDRESS_COUNCIL, Value::String(String::new()), false));
    address.fields.push(
        DbFieldStruct::new(5, data_keys::ADDRESS_POSTCODE, Value::String(String::new()), true));
    address
}

pub fn rep_table() -> DBTableStruct {
    let mut rep: DBTableStruct = DBTableStruct::new();
    rep.fields.push(
        DbFieldStruct::new(0, data_keys::ID, Value::Integer(0), true));
    rep.fields.push(
        DbFieldStruct::new(1, data_keys::TITLE, Value::String(String::new()), true));
    rep.fields.push(
        DbFieldStruct::new(2, data_keys::FIRST_NAME, Value::String(String::new()), true));
    rep.fields.push(
        DbFieldStruct::new(3, data_keys::LAST_NAME, Value::String(String::new()), true));
    rep.fields.push(
        DbFieldStruct::new(4, data_keys::CONTACT_ID, Value::Integer(0), true));
    rep
}

pub fn categories_table() -> DBTableStruct {
    let mut supplier_categories: DBTableStruct = DBTableStruct::new();
    supplier_categories.fields.push(
        DbFieldStruct::new(0, data_keys::ID, Value::Integer(0), true));
    supplier_categories.fields.push(
        DbFieldStruct::new(1, data_keys::CATEGORY_TYPE, Value::String(String::new()), true));
    supplier_categories
}