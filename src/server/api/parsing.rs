use crate::server::api::query_types::Query;
use crate::server::api::util_structs::PathSegment;
use crate::server::api::query_types::ContentFormat;
use regex::Regex;


// This function uses a tree to validate and parse the api uri resource path
// It will return a query enum if the uri is valid, otherwise it will return None
pub fn parse_api_request_to_query(
    uri: String, content: String, 
    tree_seg_root: &Box<PathSegment>
) -> Option<Query> {

    let uri_segs: Vec<_> = uri.split("/").into_iter().map(|x| String::from(x)).collect();
    let mut index = 0;
    let mut variables: Vec<String> = Vec::new();

    // if the uri is empty the api is not being accessed, return None
    if uri_segs.len() == 0 {
        return None;
    }

    // the expected first item should be "", so if there is anything else there return None
    if uri_segs.len() == 1 && uri_segs[0].is_empty() {
        return None;
    }

    // increment index to ignore first segment in the rui segment
    index += 1;


    // if the segment is not "api" then the user is not accessing the api, return None
    if !uri_segs[1].eq(&"api".to_string()) {
        return Some(Query::NoneApi)
    }
    // if there is only the "api" segment then return the ApiDoc query
    else if uri_segs.len() == 2 {
        return Some(Query::ApiDoc)
    }

    // increment index to ignore "api" segment
    index += 1;

    let mut tree_seg = tree_seg_root;

    while tree_seg.has_children() {
   
        // if there is a child segment with the same value as the current uri segment
        // then move to that segment
        if let Some(next) = tree_seg.get_next(uri_segs[index].clone()) {
            //println!("registered valid path segment {} ", uri_segs[index]);
            
            
            if index +1 < uri_segs.len() {  
                index += 1;
                tree_seg = next;
                continue;
            }
            else if let Some(query) = &next.query {
                // request body content will always be the last value to be pushed to the variables vector
                if !content.is_empty() {
                    variables.push(content);
                }
                return Some(query_from_parsed_variables(query, &variables));
            }

            return None;
               
        }
        // if there is a child segment with the value "{}" that indicates a uri parameter
        // then save that value 
        if let Some(next) = tree_seg.get_next("{}".to_string()) {
          
            variables.push(uri_segs[index].clone());
            if index + 1 < uri_segs.len() {
                tree_seg = next;    
                index += 1;
                continue;
            }
            else if let Some(query) = &next.query {
                // request body content will always be the last value to be pushed to the variables vector
                if !content.is_empty() {
                    variables.push(content);
                }
                return Some(query_from_parsed_variables(query, &variables));
            }
            
            return None;
            
        }
        // Knowing there is no child segment, it should also be the case that there
        // is no longer any uri segments left, if there are no more uri segments then 
        if index == uri_segs.len() {
            
            // see if the current segment has a query associated with it
            if let Some(query) = &tree_seg.query {
                // request body content will always be the last value to be pushed to the variables vector
                if !content.is_empty() {
                    variables.push(content);
                }
                return Some(query_from_parsed_variables(query, &variables));
            }
            
            return None;
            
        }  
        // Fall through case, the uri segment is not valid
        return Some(Query::ApiInvalidUri);
             
        
    }
    None
}


// The main purpose of this function is to take a query and a set of variables 
// and assign the variables to the query. To do so it must parse the variables, 
// if the variables are not valid then it will returns ApiInvalidUri.
fn query_from_parsed_variables(query: &Query, variables: &Vec<String>) -> Query {

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
                response_query = Query::POSTSupplier(ContentFormat::Json(json_body.unwrap()));
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
