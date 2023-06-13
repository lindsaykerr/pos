use crate::server::{api::query_types::Query, databases::data_structs::Value};
use std::collections::HashMap;
use crate::server::databases::config::data_keys;


pub fn post_sql(query: Query, values: HashMap<String, Value>) -> Option<String> {
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
        _ => {
            panic!("Invalid query type passed to post_query");
        }
    }

}