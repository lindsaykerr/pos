use json::JsonValue;

#[derive(Debug)]
pub enum Content {
    Json(JsonValue),
    Html(String),
    Text(String),
    Binary(Vec<u8>),
    None,
}



impl Clone for Content {
    fn clone(&self) -> Content {
        match self  {
            Content::Json(value) => Content::Json(value.clone()),
            Content::Html(value) => Content::Html(value.clone()),
            Content::Text(value) => Content::Text(value.clone()),
            Content::Binary(value) => Content::Binary(value.clone()),
            Content::None => Content::None,
        }
    }
}

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

    POSTSupplier(Content),
    POSTAddress(Content),
    POSTContactEmails(Content),
    POSTContactPhoneNumbers(Content),

    ApiInvalidUri,
    NoneApi,
    ApiDoc,
}

impl Clone for Query {
    fn clone(&self) -> Query {
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

            Query::POSTSupplier(content) => Query::POSTSupplier(content.clone()),
            Query::POSTAddress(content) => Query::POSTAddress(content.clone()),
            Query::POSTContactEmails(content) => Query::POSTContactEmails(content.clone()),
            Query::POSTContactPhoneNumbers(content) => Query::POSTContactPhoneNumbers(content.clone()),

       
            Query::ApiInvalidUri => Query::ApiInvalidUri,
            Query::NoneApi => Query::NoneApi,
            Query::ApiDoc => Query::ApiDoc,
        }
    }
}