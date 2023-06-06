#[derive(Debug)]
pub enum Query {
    GETSuppliers,
    GETSuppliersEmail,
    GETSuppliersNumbers,
    GETSuppliersCategories,

    GETSupplierNameFromId(u64),
    GETSupplierFromId(u64),
    GETSupplierIdFromName(String),
    GETSupplierEmailFromId(u64),
    GETSupplierNumbersFromId(u64),
    GETSupplierAddressFromId(u64),
    GETSupplierCategoriesFromId(u64),
    GETSupplierRepFromId(u64),

    //GETStockSupplierContactInfoFromId,
    GETSupplyRepFromId(u64),
    GETSupplyRepPhoneNumbersFromId(u64),
    GETSupplyRepEmailFromId(u64),

    ApiInvalidUri,
    NoneApi,
    ApiDoc,
}

impl Query {
    pub fn clone(&self) -> Query {
        match self {
            Query::GETSuppliers => Query::GETSuppliers,    
            Query::GETSuppliersEmail => Query::GETSuppliersEmail,  
            Query::GETSuppliersNumbers => Query::GETSuppliersNumbers,
            Query::GETSuppliersCategories => Query::GETSuppliersCategories,

            Query::GETSupplierNameFromId(id) => Query::GETSupplierNameFromId(*id),
            Query::GETSupplierFromId(id) => Query::GETSupplierFromId(*id),
            Query::GETSupplierIdFromName(name) => Query::GETSupplierIdFromName(name.clone()),
            Query::GETSupplierEmailFromId(id) => Query::GETSupplierEmailFromId(*id),
            Query::GETSupplierNumbersFromId(id) => Query::GETSupplierNumbersFromId(*id),
            Query::GETSupplierAddressFromId(id) => Query::GETSupplierAddressFromId(*id),
            Query::GETSupplierCategoriesFromId(id) => Query::GETSupplierCategoriesFromId(*id),
            Query::GETSupplierRepFromId(id) => Query::GETSupplierRepFromId(*id),

            Query::GETSupplyRepFromId(id) => Query::GETSupplyRepFromId(*id),
            Query::GETSupplyRepPhoneNumbersFromId(id) => Query::GETSupplyRepPhoneNumbersFromId(*id),
            Query::GETSupplyRepEmailFromId(id) => Query::GETSupplyRepEmailFromId(*id),
       
            Query::ApiInvalidUri => Query::ApiInvalidUri,
            Query::NoneApi => Query::NoneApi,
            Query::ApiDoc => Query::ApiDoc,
        }
    }
}