mod utils;
extern crate js_sys;

use wasm_bindgen::prelude::*;
use std::fmt;
use std::collections::HashMap;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//Data
#[derive(Debug)]
#[derive(Clone)]
#[derive(Hash, Eq, PartialEq)]
pub enum DataType {
    None,
    Int(i32),
    Text(String)
}

// //from conversion, ST->DT
// impl From<SchemaType> for DataType {
//     fn from(item: SchemaType) -> Self {
//         if item == SchemaType::None {
//             SchemaType::None
//         } else if item == SchemaType::Int {
//             SchemaType::Int
//         } else {
//             SchemaType::Text
//         }
//     }
// }

//Schema types
#[wasm_bindgen]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum SchemaType {
    None = 0,
    Int = 1,
    Text = 2
}

//from conversion, f64->SchemaType
impl From<f64> for SchemaType {
    fn from(item: f64) -> Self {
        if item == 2.0 {
            SchemaType::Text
        } else if item == 1.0 {
            SchemaType::Int
        } else {
            SchemaType::None
        }
    }
}

//displays DataTypes
impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::None => write!(f, "*"),
            DataType::Text(n) => {
                write!(f, "{}", n)
            }
            DataType::Int(n) => write!(f, "{}", n)
        }
    }
}

//Row
#[wasm_bindgen]     
#[derive(Hash, Eq, PartialEq)]
#[derive(Debug)]
pub struct Row {
    data: Vec<DataType>
}

//display Rows
impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // for datum in self.data.iter() {
        //     write!(f, "{} \n", datum);
        // }

        write!(f, "{:#?}", self)
    }
}

//Row functions 
impl Row {
    //constructor
    pub fn new(data: Vec<DataType>) -> Row {
        Row{ data }
    }

    //updates index
    pub fn update_index(&mut self, index: usize, update: DataType) {
        self.data[index] = update;
    }
}

//View
#[wasm_bindgen]
#[derive(Debug)]
pub struct View {
    name: String,
    columns: Vec<String>,
    schema: Vec<SchemaType>,
    table_index: usize,
    table: HashMap<Row, DataType>
}

//writing WITHOUT SCHEMA
impl fmt::Display for View {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name);
        for strings in self.columns.iter() {
            write!(f, "{}", strings);
        }
        for row in self.table.iter() {
            write!(f, "{:#?} \n", row);
        }

        //write!(f, "{:#?}", self)

        Ok(())
    }
}

impl View {
    pub fn set_col(some_iterable: &JsValue) 
                   -> Result<Vec<String>, JsValue> {
        let mut new_col = Vec::new();
        let iterator = js_sys::try_iter(some_iterable)?.ok_or_else(|| {
            "need to pass iterable JS values!"
        })?;

        for x in iterator {
            let x = x?;
            let column = x.as_string();

            if column.is_some() {
                new_col.push(column.unwrap());
            }
        }

        Ok(new_col)
    }

    pub fn set_sch(some_iterable: &JsValue) 
                   -> Result<Vec<SchemaType>, JsValue> {
        let mut new_sch = Vec::new();
        let iterator = js_sys::try_iter(some_iterable)?.ok_or_else(|| {
            "need to pass iterable JS values!"
        })?;

        for x in iterator {
            let x = x?;
            let schema = x.as_f64();

            if schema.is_some() {
                new_sch.push(SchemaType::from(schema.unwrap()));
            }
        }

        Ok(new_sch)
    }

    pub fn create_row_vec(&mut self, some_iterable: &JsValue) 
                   -> Result<Vec<DataType>, JsValue> {
        let mut row_vec = Vec::new();
        let iterator = js_sys::try_iter(some_iterable)?.ok_or_else(|| {
            "need to pass iterable JS values!"
        })?;

        let mut count = 0;

        for x in iterator {
            let mut x = x?;

            let mut ind_row = DataType::None;
            
            if self.schema[count]== SchemaType::Int {
                let insert = x.as_f64();
                if insert.is_some() {
                    let final_insert = insert.unwrap() as i32;
                    ind_row = DataType::Int(final_insert);
                }
            } else if self.schema[count] == SchemaType::Text {
                let insert = x.as_string();
                if insert.is_some() {
                    ind_row = DataType::Text(insert.unwrap());
                }
            }

            row_vec.push(ind_row);
            count = count + 1;
        }

        Ok(row_vec)
    }

}

//pageload view, view creation without a user 
#[wasm_bindgen]
impl View {
    pub fn newJS(name: String, table_index: usize, col_arr: &JsValue, 
            sch_arr: &JsValue) -> View {
        let mut table = HashMap::new();

        let mut columns = match Self::set_col(col_arr) {
            Ok(str_vec) => str_vec,
            Err(err) => Vec::new(),
        };  
        let mut schema = match Self::set_sch(sch_arr) {
            Ok(sch_vec) => sch_vec,
            Err(err) => Vec::new(),
        };   

        View {name, table_index, columns, schema, table}
    }

    pub fn insert(&mut self, js_row: &JsValue) {
        let row_data = match Self::create_row_vec(self, js_row) {
            Ok(row_vec) => row_vec,
            Err(err) => Vec::new(),
        }; 

        let key = row_data[self.table_index].clone();
        self.table.insert(Row::new(row_data.clone()), key);
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

#[wasm_bindgen]
pub struct Selection {
    parent: View,
    child: View
}

impl Selection { 
    pub fn newJS(col_ind: usize, selection: JsValue, parent: View) 
               -> Selection {
        if parent.table_index == col_ind {

        } else {

        }


    }
}



// //children and parent aware
// pub struct Selection {
    
// }

//distinct join stateful
//without duplicate removal

//library PetGraph
// pub struct Graph


