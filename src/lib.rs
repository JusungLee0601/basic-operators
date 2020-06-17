mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

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
#[wasm_bindgen]  
impl Row {
    //new accepts arbitrary columns, vector slice
    //in js dictionary, flatten
    pub fn new(inputdata: Vec<DataType>) -> Row {
        let mut data = inputdata; 

        Row{ data }
    }

    pub fn update_index<T>(&mut self, index: u32, update: T) {
        self.data[index] = update;
    }
}

//hashmaps
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
    }
}

#[wasm_bindgen]
impl View {
    pub fn new(inputname: String) -> View {
        let mut table = Vec::new();
        // let articleD = "Dummy";
        // let countD = "0";
        // let row = Row::new(articleD.to_owned(), countD.to_owned()); 
        // table.push(row);

        let mut columns = Vec::new();
        // let article = "Article";
        // let count = "Count";
        // columns.push(article.to_owned());
        // columns.push(count.to_owned());
        
        let mut schema = Vec::new();

        let name = inputname;

        View {name, columns, schema, table}
    }

    pub fn insert(&mut self, row: Row) {
        self.table.push(row)
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    //range, inequality comparison, trait rust eq, partialeq for datatypes
    //copy! into operator, get outside new, view -> operator -> view
    pub fn selection(&mut self, col: String, matching: String) {
        let mut newtable = Vec::new();

        let index = self.columns.iter().position(|r| r.to_string() == col).unwrap();

        if index == 0 {
            for row in self.table.iter() {
                match &row.data[0] {
                    DataType::Text(n) => if *n == matching {
                        let newrow = Row::new(row.data[0].to_string(), row.data[1].to_string());
                        newtable.push(newrow);
                    }, 
                    _ => println!("Hello World!"),
                }
            }
        } else if index == 1 {
            let check = matching.parse::<i32>().unwrap();

            for row in self.table.iter() {
                match &row.data[0] {
                    DataType::Int(n) => if *n == check {
                        let newrow = Row::new(row.data[0].to_string(), row.data[1].to_string());
                        newtable.push(newrow);
                    }, 
                    _ => println!("Hello World!"),
                }
            }
        }

        self.table = newtable;
    }
}



// //children and parent aware
// pub struct Selection {
    
// }

//distinct join stateful
//without duplicate removal

//library PetGraph
// pub struct Graph


