use std::fmt;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::types::schematype::SchemaType;
use crate::types::changetype::ChangeType;
use crate::types::datatype::DataType;
use crate::units::change::Change;
use crate::units::row::Row;

fn return_hash_v() -> HashMap<DataType, Row> {
    HashMap::new()
}

//View
//name: string name, assumed unique
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct View {
    name: String,
    column_names: Vec<String>,
    schema: Vec<SchemaType>,
    key_index: usize,
    #[serde(default = "return_hash_v")]
    table: HashMap<DataType, Row>,
}

//displays View
impl fmt::Display for View {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name);
        for strings in self.column_names.iter() {
            write!(f, "{}", strings);
        }
        for (key, row) in self.table.iter() {
            write!(f, "{:#?} \n", row);
        }

        Ok(())
    }
}

//View functions, unexposed
impl View {
    /// Changes View's table given a vector of Changes
    pub fn change_table(&mut self, change_vec: Vec<Change>) {
        for change in &change_vec {
            for row in &change.batch {
                match change.typing {
                    ChangeType::Insertion => {
                        let key = row.data[self.key_index].clone();
                        self.table.insert(key, row.clone());
                    },
                    ChangeType::Deletion => {
                        let key = row.data[self.key_index].clone();
                        self.table.remove(&key);
                    },
                }
            }
        }
    }

    /// Returns View as a String
    pub fn render(&self) -> String {
        self.to_string()
    }
}
