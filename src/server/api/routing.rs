use crate::server::api::util_structs::PathSegment;
use crate::server::api::query_types::Query;

pub struct ApiTree {
    pub tree: Box<PathSegment>,
}

impl ApiTree {
    pub fn new() -> ApiTree {

        // A RESTful API can be thought as a tree like structure
        // With each part of the uri acting as a branch or node.
        // In this case let define the root node as "api"
        let mut root = PathSegment::new(String::from("api"), 0);    
        
        // Now we can use the following function to build the remaining structure of the api tree
        // as it contains all the possible api routes.
        build_api_tree(&mut root);

        ApiTree {
            tree: Box::new(root),
        }

    }
}


fn build_api_tree(path_seg_root: &mut PathSegment)  {
   
   
    // **GET /api/suppliers query/branch
    let suppliers = path_seg_root.child_seg_by_value(String::from("suppliers"));
    suppliers.query = Some(Query::GETSuppliers);

        // **GET /api/suppliers/email query
        let suppliers_email = suppliers.child_seg_by_value(String::from("email"));
        suppliers_email.query = Some(Query::GETSuppliersEmail);

        // **GET /api/suppliers/numbers query
        let suppliers_numbers = suppliers.child_seg_by_value(String::from("numbers"));
        suppliers_numbers.query = Some(Query::GETSuppliersNumbers);

        let suppliers_categories = suppliers.child_seg_by_value(String::from("categories"));
        suppliers_categories.query = Some(Query::GETSuppliersCategories);

    
    // GET /api/supplier branch
    let supplier = path_seg_root.child_seg_by_value(String::from("supplier"));

        // GET /api/supplier/id branch
        let supplier_id_seg = supplier.child_seg_by_value(String::from("id"));
            
            // GET /api/supplier/id/{name} query
            let supplier_id_name = supplier_id_seg.child_seg_by_value(String::from("{}"));
                supplier_id_name.query = Some(Query::GETSupplierIdFromName(String::from("{}")));

        // GET /api/supplier/{id} query/branch
        let supplier_id = supplier.child_seg_by_value(String::from("{}"));
            supplier_id.query = Some(Query::GETSupplierFromId(0));
            
            // GET /api/supplier/{id}/name query
            let supplier_id_name = supplier_id.child_seg_by_value(String::from("name"));
                supplier_id_name.query = Some(Query::GETSupplierNameFromId(0));
                
            // GET /api/supplier/{id}/address query
            let supplier_id_address = supplier_id.child_seg_by_value(String::from("address"));
                supplier_id_address.query = Some(Query::GETSupplierAddressFromId(0));
            
            // GET /api/supplier/{id}/rep query
            let supplier_id_rep = supplier_id.child_seg_by_value(String::from("rep"));
                supplier_id_rep.query = Some(Query::GETSupplierRepFromId(0));

            // GET /api/supplier/{id}/categories query
            let supplier_id_categories = supplier_id.child_seg_by_value(String::from("categories"));
                supplier_id_categories.query = Some(Query::GETSupplierCategoriesFromId(0));


        // GET /api/supplier/rep branch
        let supplier_rep = supplier.child_seg_by_value(String::from("rep"));

            // GET /api/supplier/rep/{id} query/branch
            let rep_id = supplier_rep.child_seg_by_value(String::from("{}"));
                rep_id.query = Some(Query::GETSupplyRepFromId(0));

                // GET /api/supplier/rep/{id}/numbers query
                let rep_id_numbers = rep_id.child_seg_by_value(String::from("numbers"));
                    rep_id_numbers.query = Some(Query::GETSupplyRepPhoneNumbersFromId(0));

                // GET /api/supplier/rep/{id}/email query
                let rep_id_email = rep_id.child_seg_by_value(String::from("email"));
                    rep_id_email.query = Some(Query::GETSupplyRepEmailFromId(0));
             
                

}