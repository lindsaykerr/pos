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
            Query::ApiDoc => Query::ApiDoc,
        }
    }
}


pub fn api(method: String, path: String) -> Option<Query>  {

    
    

    // API routing goes here
    match method.as_str() {
        "GET" => {
            let mut path_seg_root = Box::new(PathSegment::new(String::from(""), 0));
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

    pub fn move_to_next(&self, seg_value: String) -> Option<&Box<PathSegment>> {
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
}

fn query_from_uri<'a>(uri: String, path_seg_root: &Box<PathSegment>) -> Option<Query> {
    let mut uri_segs: Vec<_> = uri.split("/").into_iter().map(|x| String::from(x)).collect();
    
    if uri_segs.len() < 2 {
        return Some(Query::ApiDoc);
    }

    uri_segs.remove(0);

    let mut variable = String::new();

    
    let mut next: &Box<PathSegment> = path_seg_root;

    for seg in uri_segs {
        
        let seg_copy = seg.clone();

     ;

        match next.move_to_next(seg_copy) {
            Some(seg) => { 
                println!("next_seg: {}", seg.value);
                next = seg
            },
            None => {
                print!("no next seg");
                if !next.query.is_none() {
                    println!("value {}", next.value);
                    let query = match next.query.as_ref().unwrap() {
                        Query::GETStockSupplierNameFromId(_) => Query::GETStockSupplierNameFromId(variable.parse::<u64>().unwrap()),
                        Query::GETStockSupplierFromId(_) => Query::GETStockSupplierFromId(variable.parse::<u64>().unwrap()),
                        Query::GetStockSuppliersEmail => Query::GetStockSuppliersEmail,
                        Query::GetStockSuppliersNumbers => Query::GetStockSuppliersNumbers,
                        Query::GetStockSupplierIdFromName(_) => Query::GetStockSupplierIdFromName(variable.clone()),
                        _ => return None,
                    };
                    
                    return Some(query);
                }
                if next.value.eq(&String::from("{}")) {
                    variable = seg.clone();
                    continue;
                }
                return None;
            }
        }
    

    
    }
    

    None
}

fn get_query_tree(path_seg_root: &mut PathSegment)  {
   
    let mut api = path_seg_root.child_seg_by_value(String::from("api"));
   
        // GET /api/suppliers branch
        let suppliers = api.child_seg_by_value(String::from("suppliers"));
        suppliers.query = Some(Query::GETStockSuppliers);

            let mut suppliers_email = suppliers.child_seg_by_value(String::from("email"));
            suppliers_email.query = Some(Query::GetStockSuppliersEmail);

            let mut suppliers_numbers = suppliers.child_seg_by_value(String::from("numbers"));
            suppliers_numbers.query = Some(Query::GetStockSuppliersNumbers);

        
        // GET /api/supplier branch
        let mut supplier = api.child_seg_by_value(String::from("supplier"));

            // GET /api/supplier/id branch
            let mut supplier_id_seg = supplier.child_seg_by_value(String::from("id"));
                
                // GET /api/supplier/id/{name} query
                let mut supplier_id_name = supplier.child_seg_by_value(String::from("{}"));
                    supplier_id_name.query = Some(Query::GetStockSupplierIdFromName(String::from("{}")));

            // GET /api/supplier/{id} query/branch
            let mut supplier_id = supplier.child_seg_by_value(String::from("{}"));
                supplier_id.query = Some(Query::GETStockSupplierFromId(0));
                
                // GET /api/supplier/{id}/name query
                let mut supplier_id_name = supplier_id.child_seg_by_value(String::from("name"));
                    supplier_id_name.query = Some(Query::GETStockSupplierNameFromId(0));
}