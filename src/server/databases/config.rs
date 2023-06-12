
struct SqliteColumnNames {
    pub id: String,
    pub name: String,
    pub active: String,
    pub contact: String,
    pub contact_id: String,
    pub address: String,
    pub address_id: String,
    pub category_id: String,
    pub rep: String,    
    pub rep_id: String,
    pub email: String,
    pub number: String,
    pub address_line1: String,
    pub address_line2: String,
    pub address_town: String,
    pub address_council: String,
    pub address_postcode: String,
    pub title: String,
    pub first_name: String,
    pub last_name: String,
    pub category_type: String,
}

impl SqliteColumnNames {
    pub fn new() -> SqliteColumnNames {
        SqliteColumnNames {
            id: String::from("id"),
            name: String::from("name"),
            active: String::from("active"),
            contact: String::from("contact"),
            address: String::from("address"),
            rep: String::from("rep"),
            contact_id: String::from("contactId"),
            address_id: String::from("addressId"),
            category_id: String::from("categoryId"),
            rep_id: String::from("repId"),
            email: String::from("email"),
            number: String::from("number"),
            address_line1: String::from("line1"),
            address_line2: String::from("line2"),
            address_town: String::from("town"),
            address_council: String::from("council"),
            address_postcode: String::from("postcode"),
            title: String::from("title"),
            first_name: String::from("firstName"),
            last_name: String::from("lastName"),
            category_type: String::from("category"),
        }
    }
}

pub const DATA_FIELD: SqliteColumnNames = SqliteColumnNames::new();




