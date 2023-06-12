use crate::server::api::query_types::Query;
use std::collections::HashMap;


pub fn post_query(query: Query, values: HashMap<String, String>) -> String {
    match query {
        Query::POSTSupplier(_) => {
            let name = values.get("name");
            let active = values.get("active");
            let address = values.get("address");
            let email = values.get("email");
            let phone = values.get("phone");
            let category = values.get("category");
            let sql = format!("INSERT INTO supplier (name, active) VALUES ('{}', {})", name, active);
            sql
        },
        _ => {
            panic!("Invalid query type passed to post_query");
        }
    }

}