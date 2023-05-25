use std::path;

use crate::errors;

#[derive(Debug)]
pub enum Query {
    GETStockSuppliers,
    GETStockSupplierNameFromId(u64),
    GETStockSupplierFromId(u64),
    GetStockSuppliersEmail,
    GetStockSuppliersNumbers,
    GetStockSupplierIdFromName(String),

    /*
    GETStockSupplierAddressFromId,
    GETStockSupplierContactInfoFromId,
    GETStockSupplierRepFromId,
    GETStockSupplierSupplyCategories,
    GETStockRepIdFromName,
    GETStockRepInfoFromId,
    GETStockRepContactInfoFromId,
    StockItems,
    StockItemById(u32),
    */
    ApiInvalidUri,
    NoneApi,
    ApiDoc,
}

impl Query {
    pub fn clone(&self) -> Query {
        match self {
            Query::GETStockSuppliers => Query::GETStockSuppliers,
            Query::GETStockSupplierNameFromId(id) => Query::GETStockSupplierNameFromId(*id),
            Query::GETStockSupplierFromId(id) => Query::GETStockSupplierFromId(*id),
            Query::GetStockSuppliersEmail => Query::GetStockSuppliersEmail,
            Query::GetStockSuppliersNumbers => Query::GetStockSuppliersNumbers,
            Query::GetStockSupplierIdFromName(name) => Query::GetStockSupplierIdFromName(name.clone()),

            /*
            Query::GETStockSupplierAddressFromId => Query::GETStockSupplierAddressFromId,
            Query::GETStockSupplierContactInfoFromId => Query::GETStockSupplierContactInfoFromId,
            Query::GETStockSupplierRepFromId => Query::GETStockSupplierRepFromId,
            Query::GETStockSupplierSupplyCategories => Query::GETStockSupplierSupplyCategories,
            Query::GETStockRepIdFromName => Query::GETStockRepIdFromName,
            Query::GETStockRepInfoFromId => Query::GETStockRepInfoFromId,
            Query::GETStockRepContactInfoFromId => Query::GETStockRepContactInfoFromId,
            Query::StockItems => Query::StockItems,
            Query::StockItemById(id) => Query::StockItemById(*id),*/
            Query::ApiInvalidUri => Query::ApiInvalidUri,
            Query::NoneApi => Query::NoneApi,
            Query::ApiDoc => Query::ApiDoc,
        }
    }
}




pub fn api(method: String, path: String) -> Option<Query>  {

    
    

    // API routing goes here
    match method.as_str() {
        "GET" => {
            let mut path_seg_root = Box::new(PathSegment::new(String::from("api"), 0));
            println!("path: {}", path);
            get_query_tree(&mut path_seg_root);

            query_from_uri(path, &mut path_seg_root)

            
        },
        _ => None,
    }
}



pub struct PathSegment {
    pub value: String,
    pub seg_number: u16,
    pub children_segments: Vec<Box<PathSegment>>,
    pub query: Option<Query>,
}
impl PathSegment {
    pub fn new(value: String, seg_number: u16) -> PathSegment {
        PathSegment {
            value,
            seg_number,
            children_segments: Vec::new(),
            query: None,
        }
    }

    pub fn child_seg(&mut self, child: PathSegment) {

        self.children_segments.push(Box::new(child));
    }

    pub fn child_seg_by_value(&mut self, value: String) -> &mut Box<PathSegment> {
     
        
        let child = PathSegment::new(value.clone(), self.seg_number + 1);
        self.children_segments.push(Box::new(child));
        self.children_segments.last_mut().unwrap()
    }

    pub fn get_next(&self, seg_value: String) -> Option<&Box<PathSegment>> {
        if self.children_segments.len() == 0 {
            return None;
        }

        for child in &self.children_segments {
       
            if child.value.eq(&seg_value) {
                return Some(child);
            }
        }
        None
    }

    pub fn has_children(&self) -> bool {
        self.children_segments.len() > 0
    }
}



fn query_from_uri(uri: String, tree_seg_root: &Box<PathSegment>) -> Option<Query> {

    let mut uri_segs: Vec<_> = uri.split("/").into_iter().map(|x| String::from(x)).collect();
    let mut index = 0;
    let mut variables: Vec<String> = Vec::new();

    // if the uri is empty the api is not being accessed, return None
    if uri_segs.len() == 0 {
        return None;
    }

    // the expected first item should be "", so if that item is all there is then return None
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
            // println!("registered valid path segment {} ", uri_segs[index]);
            
            
            if index +1 < uri_segs.len() {  
                index += 1;
                tree_seg = next;
                continue;
            }
            else if let Some(query) = &next.query {
                return Some(query_from_parsed_variables(query, &variables));
            }
            else {
                return None;
            }   
        }
        // if there is a child segment with the value "{}" that indicates a uri parameter
        // then save that value 
        else if let Some(next) = tree_seg.get_next("{}".to_string()) {
            // println!("registered variable");
            variables.push(uri_segs[index].clone());
            if index + 1 < uri_segs.len() {
                tree_seg = next;    
                index += 1;
                continue;
            }
            else if let Some(query) = &next.query {
                return Some(query_from_parsed_variables(query, &variables));
            }
            else {
                return None;
            }
        }
        else if let Some(query) = &tree_seg.query {
            return Some(query_from_parsed_variables(query, &variables));
        }
        else {
            return None;
        }
    }
    None
}


// The main purpose of this function is to take a query and a set of variables 
// and assign the variables to the query. To do so it must parse the variables, 
// if the variables are not valid then it will returns ApiInvalidUri.
fn query_from_parsed_variables(query: &Query, variables: &Vec<String>) -> Query {

    let mut response_query = Query::ApiInvalidUri;


    match query {
        Query::GetStockSupplierIdFromName(_) => {
            let name = &variables[0];
            response_query = Query::GetStockSupplierIdFromName(name.clone());
        },
        Query::GETStockSupplierFromId(_) => {
                if let Ok(id) = variables[0].parse::<u64>() {
                    response_query = Query::GETStockSupplierFromId(id);
                }    
        },
        Query::GETStockSuppliers => {
            response_query = Query::GETStockSuppliers;
        },
        _ => panic!("query_add_variables: query not found: {:?}", query),
    }
    response_query
}


fn get_query_tree(path_seg_root: &mut PathSegment)  {
   
   
        // GET /api/suppliers branch
        let suppliers = path_seg_root.child_seg_by_value(String::from("suppliers"));
        suppliers.query = Some(Query::GETStockSuppliers);

            let mut suppliers_email = suppliers.child_seg_by_value(String::from("email"));
            suppliers_email.query = Some(Query::GetStockSuppliersEmail);

            let mut suppliers_numbers = suppliers.child_seg_by_value(String::from("numbers"));
            suppliers_numbers.query = Some(Query::GetStockSuppliersNumbers);

        
        // GET /api/supplier branch
        let mut supplier = path_seg_root.child_seg_by_value(String::from("supplier"));

            // GET /api/supplier/id branch
            let mut supplier_id_seg = supplier.child_seg_by_value(String::from("id"));
                
                // GET /api/supplier/id/{name} query
                let mut supplier_id_name = supplier_id_seg.child_seg_by_value(String::from("{}"));
                    supplier_id_name.query = Some(Query::GetStockSupplierIdFromName(String::from("{}")));

            // GET /api/supplier/{id} query/branch
            let mut supplier_id = supplier.child_seg_by_value(String::from("{}"));
                supplier_id.query = Some(Query::GETStockSupplierFromId(0));
                
                // GET /api/supplier/{id}/name query
                let mut supplier_id_name = supplier_id.child_seg_by_value(String::from("name"));
                    supplier_id_name.query = Some(Query::GETStockSupplierNameFromId(0));
}

