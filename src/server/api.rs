pub mod parsing;
pub mod routing;
pub mod query_types;
pub mod util_structs;

use util_structs::{PathSegment};
use query_types::{Query};
use parsing::{parse_api_request_to_query};

use self::routing::ApiTree;



pub fn process_api_query(path: String, body_content: String, api_tree: &mut Box<ApiTree>) -> Option<Query>  {

    // Now that we know the possible routes we can parse the request uri against it to see if 
    // contains a valid route
    parse_api_request_to_query(path, body_content, &api_tree.tree)
   
}





