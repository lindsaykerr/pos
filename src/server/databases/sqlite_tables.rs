use crate::server::databases::data_structs::{DBTableStruct, DbFieldStruct};
use crate::server::databases::data_structs::Value;

pub fn supplier_table() -> DBTableStruct {
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
}

pub fn email_table() -> DBTableStruct {
    let mut suppliers_email: DBTableStruct = DBTableStruct::new();
    suppliers_email.fields.push(
        DbFieldStruct::new(0, "id", Value::Integer(0), true));
    suppliers_email.fields.push(
        DbFieldStruct::new(1, "email", Value::String(String::new()), true));
    suppliers_email
}

pub fn numbers_table() -> DBTableStruct {
    let mut suppliers_numbers: DBTableStruct = DBTableStruct::new();
    suppliers_numbers.fields.push(
        DbFieldStruct::new(0, "id", Value::Integer(0), true));
    suppliers_numbers.fields.push(
        DbFieldStruct::new(1, "number", Value::String(String::new()), true));
    suppliers_numbers
}

pub fn id_table() -> DBTableStruct {
    let mut id: DBTableStruct = DBTableStruct::new();
    id.fields.push(
        DbFieldStruct::new(0, "id", Value::Integer(0), true));
    id
}

pub fn supplier_name_table() -> DBTableStruct {
    let mut supplier_name: DBTableStruct = DBTableStruct::new();
    supplier_name.fields.push(
        DbFieldStruct::new(0, "name", Value::String(String::new()), true));
    supplier_name
}

pub fn address_table() -> DBTableStruct {
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
}

pub fn rep_table() -> DBTableStruct {
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
}

pub fn categories_table() -> DBTableStruct {
    let mut supplier_categories: DBTableStruct = DBTableStruct::new();
    supplier_categories.fields.push(
        DbFieldStruct::new(0, "id", Value::Integer(0), true));
    supplier_categories.fields.push(
        DbFieldStruct::new(1, "category", Value::String(String::new()), true));
    supplier_categories
}