pub mod parsing;
pub mod routing;
pub mod query_types;
pub mod util_structs;

use util_structs::{PathSegment};
use query_types::{Query};
use routing::{get_query_tree};
use parsing::{parse_api_request_to_query};



pub fn api(method: String, path: String) -> Option<Query>  {

    // API routing goes here
    match method.as_str() {
        "GET" => {
            let mut path_seg_root = Box::new(PathSegment::new(String::from("api"), 0));
            println!("path: {}", path);
            get_query_tree(&mut path_seg_root);

            parse_api_request_to_query(path, &mut path_seg_root)
            
        },
        _ => None,
    }
}





