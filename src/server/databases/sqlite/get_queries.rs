use crate::server::api::query_types::Query;

pub fn get_sql(query: &Query) -> String {

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
