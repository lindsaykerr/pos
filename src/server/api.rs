pub mod parsing;
pub mod routing;
pub mod query_types;
pub mod util_structs;
pub mod config;


use query_types::{Query};
use parsing::{query_with_path_variables};
use self::routing::ApiTree;



pub fn uri_to_api_query(uri: &String, api_tree: &mut Box<ApiTree>) -> Option<Query> {

    // get query type


    // split the uri into segments
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

    let mut tree_seg = &api_tree.tree;

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
                return Some(query_with_path_variables(query, &variables));
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
                return Some(query_with_path_variables(query, &variables));
            }
            
            return None;
            
        }
        // Knowing there is no child segment, it should also be the case that there
        // is no longer any uri segments left, if there are no more uri segments then 
        if index == uri_segs.len() {
            
            // see if the current segment has a query associated with it
            if let Some(query) = &tree_seg.query {
                // request body content will always be the last value to be pushed to the variables vector
                return Some(query_with_path_variables(query, &variables));
            }
            
            return None;
            
        }  
        // Fall through case, the uri segment is not valid
        return Some(Query::ApiInvalidUri);
                
        
    }


    return None;


   
}

