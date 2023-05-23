use json::{self, JsonValue};

/// Used as switch value to choose which database to use
pub enum Type {
    Sqlite,
    Postgres,
}

/// Used to represent a cell value in an relational database
pub enum Value {
    Boolean(bool),
    Binary(Vec<u8>),
    Float(f64),
    Integer(i64),
    String(String),
    Null,
}

/// Used to represent a row the characteristics of a field/column in an relational database
pub struct DbFieldStruct {
    pub column: usize,
    pub name: String,
    pub a_type: Value,
    pub not_null: bool,
}

impl DbFieldStruct {
    pub fn new(column: usize, name: &str, a_type: Value, not_null: bool) -> DbFieldStruct {
        DbFieldStruct { 
            column, 
            name: name.to_string(), 
            a_type,
            not_null,
        }
    }
}

impl Clone for DbFieldStruct {
    fn clone(&self) -> DbFieldStruct {
        DbFieldStruct {
            column: self.column,
            name: self.name.clone(),
            a_type: match &self.a_type {
                Value::Boolean(b) => Value::Boolean(*b),
                Value::Binary(b) => Value::Binary(b.clone()),
                Value::Float(f) => Value::Float(*f),
                Value::Integer(i) => Value::Integer(*i),
                Value::String(s) => Value::String(s.clone()),
                Value::Null => Value::Null,
            },
            not_null: self.not_null,
        }
    }
}

/// Used to represent the complete structure of a relational database table
pub struct DBTableStruct {
    pub fields: Vec<DbFieldStruct>,
}

impl DBTableStruct {
    pub fn new() -> DBTableStruct {
        DBTableStruct {
            fields: Vec::new(),
        }
    }
}

impl Clone for DBTableStruct {
    fn clone(&self) -> DBTableStruct {
        let mut cloned_fields = Vec::new();
        for field in &self.fields {
            let field = field.clone();
            cloned_fields.push(field);
        }
        
        DBTableStruct {
            fields: cloned_fields
        }
    }
}


/// Holds data for a single row in a relational database table
pub struct DBTableRow {
    cells: Vec<Value>,
}
impl DBTableRow {
    pub fn new() -> DBTableRow {
        DBTableRow {
            cells: Vec::<Value>::new(),
        }
    }
    pub fn add_cell(&mut self, cell: Value) {
        self.cells.push(cell);
    }
}

/// Holds the data from a relational database table or view
pub struct  DBTable {
    pub structure: DBTableStruct,
    pub rows: Vec<DBTableRow>,
}

impl DBTable {
    pub fn new(db_struct: &DBTableStruct) -> DBTable {
        DBTable {
            structure: db_struct.clone(),
            rows: Vec::new(),
        }
    }

    ///
    /// Add a new row to the database
    /// 
    pub fn add_row(&mut self, row: DBTableRow) {
        self.rows.push(row);
    }

    ///
    /// Converts the table to a json array
    /// 
    pub fn to_json(&self) -> JsonValue {

        let mut json_table = json::array![];
        
        for row in &self.rows {
            let mut table_row = json::object!{};
      
    
            for (i, cell) in row.cells.iter().enumerate() {
                let fields = &self.structure.fields.get(i);
                
                if let None = fields {
                    continue;
                }
                
                let fields = fields.unwrap();
                let field_name = fields.name.clone();
          
                table_row[field_name] = match cell {
                    Value::Boolean(value) => JsonValue::from(*value),
                    Value::Binary(value) => JsonValue::from(value.clone()),
                    Value::Float(value) => JsonValue::from(*value),
                    Value::Integer(value) => JsonValue::from(*value),
                    Value::String(value) => JsonValue::from(value.clone()),
                    Value::Null => JsonValue::Null,
                }
            }
            if let Err(result) = json_table.push(table_row) {
                println!("Error: {}", result);
            }
        }
    
        json_table
    }
    
}