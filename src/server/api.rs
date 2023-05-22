
pub enum Query {
    GETStockSuppliers,
    GETStockSupplierName,
    GETStockSupplierId(u64),
    /*GETStockSupplierGetNameFromId,
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
            Query::GETStockSupplierName => Query::GETStockSupplierName,
            Query::GETStockSupplierId(id) => Query::GETStockSupplierId(*id),
            /*Query::GETStockSupplierGetNameFromId => Query::GETStockSupplierGetNameFromId,
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
            match path.as_str() {
                "/" | "/api" => Some(Query::ApiDoc),
                "/api/suppliers" => Some(Query::GETStockSuppliers), 
                _ => {
                    if path.starts_with("/api/supplier") {
                        let regex_id = regex::Regex::new(r"/api/supplier/(/\d+)(/[A-z\-_]*)?").unwrap();
                        for capture in regex_id.captures_iter(path.as_str()) {
                            if capture.len() == 2 {
                                return Some(Query::GETStockSupplierId(capture[1].parse::<u64>().unwrap()));
                            } 
                            else if capture.len() == 3 {
                                match &capture[2] {
                                    "/name" => return Some(Query::GETStockSupplierName),
                                    _ => return None,
                                }
                            } else {
                                return None;
                            }
                        } 
                    }
                    return None;
                },
            
            }
        },
        _ => None,
    }
}