use crate::server::api::query_types::Query;
use crate::server::api::query_types::Content;
use regex::Regex;


// This function uses a tree to validate and parse the api uri resource path
// It will return a query enum if the uri is valid, otherwise it will return None


// The main purpose of this function is to take a query and a set of variables 
// and assign the variables to the query. To do so it must parse the variables, 
// if the variables are not valid then it will returns ApiInvalidUri.
pub fn query_with_path_variables(query: &Query, variables: &Vec<String>) -> Query {

    let mut response_query = Query::ApiInvalidUri;

    match query {

        // suppliers
        Query::GETSuppliers => {
            response_query = Query::GETSuppliers;
        },
        Query::GETSuppliersEmail => {
            response_query = Query::GETSuppliersEmail;
                
        },
        Query::GETSuppliersNumbers => {
            response_query = Query::GETSuppliersNumbers;
        },
        Query::GETSuppliersCategories => {
            response_query = Query::GETSuppliersCategories;
        },

        // supplier from id
        Query::GETSupplierNameFromId(_) => {
            if let Ok(id) = variables[0].parse::<u64>() {
                response_query = Query::GETSupplierNameFromId(id);
            }
        },
        Query::GETSupplierFromId(_) => {
            if let Ok(id) = variables[0].parse::<u64>() {
                response_query = Query::GETSupplierFromId(id);
            }    
        },
        Query::GETSupplierIdFromName(_) => {
            let name = &variables[0];
            let name = uri_seg_decode(name);
            response_query = Query::GETSupplierIdFromName(name.clone());
        },
        Query::GETSupplierEmailFromId(_) => {
            if let Ok(id) = variables[0].parse::<u64>() {
                response_query = Query::GETSupplierEmailFromId(id);
            }
        },
        Query::GETSupplierNumbersFromId(_) => {
            if let Ok(id) = variables[0].parse::<u64>() {
                response_query = Query::GETSupplierNumbersFromId(id);
            }
        },
        Query::GETSupplierAddressFromId(_) => {
            if let Ok(id) = variables[0].parse::<u64>() {
                response_query = Query::GETSupplierAddressFromId(id);
            }
        },
        Query::GETSupplierCategoriesFromId(_) => {
            if let Ok(id) = variables[0].parse::<u64>() {
                response_query = Query::GETSupplierCategoriesFromId(id);
            }
        },
        Query::GETSupplierRepFromId(_) => {
            if let Ok(id) = variables[0].parse::<u64>() {
                response_query = Query::GETSupplierRepFromId(id);
            }
        },

        // supplier rep
        Query::GETSupplyRepFromId(_) => {
            if let Ok(id) = variables[0].parse::<u64>() {
                response_query = Query::GETSupplyRepFromId(id);
            }
        },
        Query::GETSupplyRepPhoneNumbersFromId(_) => {
            if let Ok(id) = variables[0].parse::<u64>() {
                response_query = Query::GETSupplyRepPhoneNumbersFromId(id);
            }
        },
        Query::GETSupplyRepEmailFromId(_) => {
            if let Ok(id) = variables[0].parse::<u64>() {
                response_query = Query::GETSupplyRepEmailFromId(id);
            }
        },

        // POST supplier
        Query::POSTSupplier(_) => {
            let json_body = json::parse(&variables[0].clone());
            if json_body.is_ok() {
                response_query = Query::POSTSupplier(Content::Json(json_body.unwrap()));
            }     
        },

        _ => panic!("Query has not been implemented at query_add_variables in parsing.rs {:?}", query),
    }
    response_query
}


fn uri_seg_decode(uri: &str) -> String {
    let mut uri = uri.to_string();
    let special_regex = Regex::new(r"(%[0-9A-Fa-f]{2})").unwrap();
    for cap in special_regex.captures_iter(&uri.clone()) {
        let special = cap.get(1).unwrap().as_str();
        let special = special.replace("%", "");
        let special = u8::from_str_radix(&special, 16).unwrap();
        let special = special as char;
        uri = uri.replace(cap.get(1).unwrap().as_str(), &special.to_string());
    }
    uri
}




#[cfg(test)]
mod test {
    #[test]
    fn test_uri_seg_decode() {
        let uri = "/api/supplier/1/Mr%20Smith%20%26%20Co%20Ltd/name";
        let decoded = super::uri_seg_decode(uri);
        assert_eq!(decoded, "/api/supplier/1/Mr Smith & Co Ltd/name");
    }
}
