mod utils;
extern crate js_sys;

use wasm_bindgen::prelude::*;
use std::fmt;
//use std::collections::HashMap;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//Data
#[derive(Debug)]
pub enum DataType {
    None,
    Int(i32),
    Text(String)
}

//Schema types
#[derive(Debug)]
pub enum SchemaType {
    None,
    Int,
    Text
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

#[derive(Debug)]
pub enum RowError {
}

//Row
#[wasm_bindgen]     
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
    //new accepts arbitrary columns, vector slice
    //in js dictionary, flatten
    pub fn new(inputdata: Vec<DataType>) -> Row {
        let mut data = inputdata; 

        Row{ data }
    }

    pub fn update_index(&mut self, index: usize, update: DataType) {
        self.data[index] = update;
    }
}

//hashmaps, keys are in their own column 
#[wasm_bindgen]
#[derive(Debug)]
pub struct View {
    name: String,
    columns: Vec<String>,
    schema: Vec<SchemaType>,
    table: Vec<Row>
}

//writing WITHOUT SCHEMA
impl fmt::Display for View {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name);
        for strings in self.columns.iter() {
            write!(f, "{}", strings);
        }
        for row in self.table.iter() {
            write!(f, "{} \n", row);
        }

        //write!(f, "{:#?}", self)

        Ok(())
    }
}

impl View {
    pub fn set_col(some_iterable: &JsValue) -> Result<Vec<String>, JsValue> {
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

    // pub fn set_sch(some_iterable: &JsValue) -> Result<Vec<SchemaType>, JsValue> {
    //     let mut new_sch = Vec::new();
    //     let iterator = js_sys::try_iter(some_iterable)?.ok_or_else(|| {
    //         "need to pass iterable JS values!"
    //     })?;

    //     for x in iterator {
    //         let x = x?;

    //         new_sch.push(x);
    //     }

    //     Ok(new_sch)
    // }
}

//pageload view, view creation without a user 
#[wasm_bindgen]
impl View {
    pub fn new(input_name: String, col_arr: &JsValue, 
            sch_arr: &JsValue) -> View {
        let name = input_name;
        let mut table = Vec::new();
        let mut schema = Vec::new();

        let mut columns = match Self::set_col(col_arr) {
            Ok(str_vec) => str_vec,
            Err(err) => Vec::new(),
        };   
        //let mut schema = Self::set_sch(sch_arr)

        View {name, columns, schema, table}
    }

    // pub fn insert(&mut self, row: Row) {
    //     self.table.push(row)
    // }

    pub fn render(&self) -> String {
        self.to_string()
    }

    //range, inequality comparison, trait rust eq, partialeq for datatypes
    //copy! into operator, get outside new, view -> operator -> view
    // pub fn selection(&mut self, col: String, matching: String) {
    //     let mut newtable = Vec::new();

    //     let index = self.columns.iter().position(|r| r.to_string() == col).unwrap();

    //     if index == 0 {
    //         for row in self.table.iter() {
    //             match &row.data[0] {
    //                 DataType::Text(n) => if *n == matching {
    //                     let newrow = Row::new(row.data[0].to_string(), row.data[1].to_string());
    //                     newtable.push(newrow);
    //                 }, 
    //                 _ => println!("Hello World!"),
    //             }
    //         }
    //     } else if index == 1 {
    //         let check = matching.parse::<i32>().unwrap();

    //         for row in self.table.iter() {
    //             match &row.data[0] {
    //                 DataType::Int(n) => if *n == check {
    //                     let newrow = Row::new(row.data[0].to_string(), row.data[1].to_string());
    //                     newtable.push(newrow);
    //                 }, 
    //                 _ => println!("Hello World!"),
    //             }
    //         }
    //     }

    //     self.table = newtable;
    // }
}



// //children and parent aware
// pub struct Selection {
    
// }

//distinct join stateful
//without duplicate removal

//library PetGraph
// pub struct Graph


