use crate::value_type::ValueType;

pub struct Schema {
    pub tables: Vec<(String, Table)>,
}

pub struct Table {
    pub columns: Vec<(String, ValueType)>,
}
