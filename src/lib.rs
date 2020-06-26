mod utils;
extern crate js_sys;

use wasm_bindgen::prelude::*;
use std::fmt;
use std::collections::HashMap;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;

// SOME IMPORTANT ASSUMPTIONS

// + I don't differentiate between Tables and Views. 
// + Operators do not generate views. Instead, views are made first and 
//   then connected with an operator. This assumes that the entire graph is
//   built, and then filled in with relevant columns.
// + Honestly feels so foreign. Strange to have to "build" the structure, but
//   makes sense in what actually gets sent to a client (code), and I guess the 
//   the amount itself is relatively small compared to what's actually held
//   server side.

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//Data
#[derive(Debug)]
#[derive(Clone, Hash, Eq, PartialEq)]
pub enum DataType {
    None,
    Int(i32),
    Text(String)
}

//from conversion, f64->SchemaType
impl From<&JsValue> for DataType {
    fn from(item: &JsValue) -> Self {
        if (*item).as_f64().is_some()  {
            DataType::Int(item.as_f64().unwrap() as i32)
        } else if (*item).as_string().is_some()  {
            DataType::Text(item.as_string().unwrap())
        } else {
            DataType::None
        }
    }
}

//Schema types
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub enum SchemaType {
    None = 0,
    Int = 1,
    Text = 2
}

//from conversion, f64->SchemaType
impl From<JsValue> for SchemaType {
    fn from(item: JsValue) -> Self {
        if item == 2 {
            SchemaType::Text
        } else if item == 1 {
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
#[derive(Debug)]
#[derive(Hash, Eq, PartialEq, Clone)]
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

//Schema types
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeType {
    Insertion,
    Deletion
}

//start with change
//pass in graph pointer
//use non mutable graph 
//Schema types
#[derive(Debug, Clone, PartialEq)]
pub struct Change {
    typing: ChangeType,
    batch: Vec<Row>
}

impl Change {
    pub fn new(typing: ChangeType, batch: Vec<Row>) {
        Change { typing, batch }
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
    table: HashMap<DataType, Row>
}

//writing WITHOUT SCHEMA
impl fmt::Display for View {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name);
        for strings in self.columns.iter() {
            write!(f, "{}", strings);
        }
        for (key, row) in self.table.iter() {
            write!(f, "{:#?} \n", row);
        }

        //write!(f, "{:#?}", self)

        Ok(())
    }
}

//processing fnss to set columns and schema, view mod fns
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

            new_sch.push(SchemaType::from(x));
        }

        Ok(new_sch)
    }

    pub fn insert(&mut self, change_ins: Change, dfg: &DataFlowGraph) {
        for row in &(change_ins.batch) {
            let key = row[self.table_index].clone();
            self.table.insert(key, row.clone());
        }

        let mut graph_field = (*dfg).data;
        let parent_index = (*dfg).index_map.get(self.name).unwrap();

        for child_index in graph_field.neighbors(parent_index) {
             let edge_index = graph_field.find_edge(parent_index, child_index).unwrap();
             let edge_op = graph_field.edge_weight(edge_index);

             let next_change = edge_op.apply(change_ins);
             let child_view = graph_field.node_weight_mut(child_index);

            if next_change.batch.is_empty() {
               let child_view = graph_field.node_weight_mut(child_index);

               child_view.insert(next_change, dfg);
            }
        }
    }

    pub fn delete(&mut self, change_del: Change, dfg: &DataFlowGraph) {
        for row in &(change_del.batch) {
            let key = row[self.table_index].clone();
            self.table.remove(key);
        }

        let mut graph_field = (*dfg).data;
        let parent_index = (*dfg).index_map.get(self.name).unwrap();

        for child_index in graph_field.neighbors(parent_index) {
            let edge_index = graph_field.find_edge(parent_index, child_index).unwrap();
            let edge_op = graph_field.edge_weight(edge_index);

            let next_change = edge_op.apply(change_del);
             
            if next_change.batch.is_empty() {
                let child_view = graph_field.node_weight_mut(child_index);

                child_view.delete(next_change, dfg);
             }
        }
    }
}

//pageload view, view creation without a user 
#[wasm_bindgen]
impl View {
    pub fn new(name: String, table_index: usize, col_arr: &JsValue, 
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

    pub fn render(&self) -> String {
        self.to_string()
    }
}

pub trait Operator {
    fn apply(&mut self, prev_change: Change) -> Change; 
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Selection {
    col_ind: usize,
    condition: DataType
}

impl Operator for Selection {
    fn apply(&mut self, prev_change: Change) -> Change {
        let next_change = Change { typing: prev_change.typing, batch: Vec::new()};

        for row in &(prev_change.batch) {
            if row.data[self.col_ind] == self.condition {
                next_change.push(row);
            }
        }
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct DataFlowGraph {
    data: Graph<View, dyn Operator>,
    index_map: HashMap<String, NodeIndex> 
}

//writing WITHOUT SCHEMA
impl fmt::Display for DataFlowGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl DataFlowGraph { 
    pub fn process_into_row(inp_view: &View, some_iterable: &JsValue)
            -> Result<Row, JsValue> {
        let mut row_vec = Vec::new();
        let iterator = js_sys::try_iter(some_iterable)?.ok_or_else(|| {
            "need to pass iterable JS values!"
        })?;

        let mut count = 0;

        for x in iterator {
            let mut x = x?;

            let mut ind_row = DataType::None;
            
            if (*inp_view).schema[count]== SchemaType::Int {
                let insert = x.as_f64();
                if insert.is_some() {
                    let final_insert = insert.unwrap() as i32;
                    ind_row = DataType::Int(final_insert);
                }
            } else if (*inp_view).schema[count] == SchemaType::Text {
                let insert = x.as_string();
                if insert.is_some() {
                    ind_row = DataType::Text(insert.unwrap());
                }
            }

            row_vec.push(ind_row);
            count = count + 1;
        }

        Ok(Row::new(row_vec))
    }
}

#[wasm_bindgen]
impl DataFlowGraph { 
    pub fn new() -> DataFlowGraph {
        let data = Graph::new();
        let index_map = HashMap::new();

        DataFlowGraph { data, index_map }
    }

    pub fn extend(&mut self, parent: View, child: View, operator: dyn Operator) {
        let first = self.data.add_node(parent);
        self.index_map.insert(parent.name.clone(), first.clone());

        let second = self.data.add_node(child);
        self.index_map.insert(child.name.clone(), second.clone());

        self.data.add_edge(first, second, operator);
    }

    pub fn process_insert(&mut self, view_string: String, row_ins_js: &JsValue) {
        let mut row_ins_rust = match Self::process_into_row(row_ins_js) {
            Ok(row) => row,
            Err(err) => Row::new(Vec::new()),
        };  

        let view_to_edit = self.index_map.get(view_string).unwrap();
        let change_ins = Change::new(ChangeType::Insertion, Vec::new(row_ins_rust));
        
        view_to_edit.insert(change_ins, &self);
    }

    pub fn process_delete(&mut self, view_string: &View, row_del_js: &JsValue) {
        let mut row_del_rust = match Self::process_into_row(row_del_js) {
            Ok(row) => row,
            Err(err) => Row::new(Vec::new()),
        };  

        let view_to_edit = self.index_map.get(view_string).unwrap();
        let change_ins = Change::new(ChangeType::Deletion, Vec::new(row_del_rust));
        
        view_to_edit.delete(change_ins, &self); 
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}


