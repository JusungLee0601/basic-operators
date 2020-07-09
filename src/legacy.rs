mod utils;
extern crate js_sys;

#[macro_use]
extern crate serde_derive;

//extern crate wasm_bindgen_test;
//use wasm_bindgen_test::*;

use wasm_bindgen::prelude::*;
use std::fmt;
use std::collections::HashMap;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use std::cell::{RefCell, RefMut};
use std::cell::Ref;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use web_sys::console;
//use serde_json::Result;


// SOME IMPORTANT ASSUMPTIONS

// + I don't differentiate between Tables and Views. 
// + Operators do not generate views. Instead, views are made first and 
//   then connected with an operator. This assumes that the entire graph is
//   built, and then filled in with relevant columns.
// + Honestly feels so foreign. Strange to have to "build" the structure, but
//   makes sense in what actually gets sent to a client (code), and I guess the 
//   the amount itself is relatively small compared to what's actually held
//   server side.
// + I think the above can be replaced with a JSON file building the relevant 
//   tree, and then adding data through calls?? 

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//Data
#[derive(Debug)]
#[derive(Clone, Hash, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
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
#[derive(Serialize, Deserialize)]
pub enum SchemaType {
    None,
    Int,
    Text
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
    pub fn new(typing: ChangeType, batch: Vec<Row>) -> Change {
        Change { typing, batch }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ViewJSON {
    name: String,
    columns: Vec<String>,
    schema: Vec<SchemaType>,
    table_index: usize,
}

//from conversion, f64->SchemaType
impl From<ViewJSON> for View {
    fn from(item: ViewJSON) -> Self {
        let view = View::newJSON(item.name, item.table_index, item.columns, item.schema);

        view
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
    pub fn newJSON(name: String, table_index: usize, columns: Vec<String>, 
        schema: Vec<SchemaType>) -> View {
        let table = HashMap::new();

        View {name, table_index, columns, schema, table}
    }

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

    pub fn change_table(&mut self, change_ins: Vec<Change>, dfg: &DataFlowGraph) {
        for row in &change_ins.batch {
            match change_ins.typing {
                ChangeType::Insertion => {
                    let key = row.data[self.table_index].clone();
                    self.table.insert(key, row.clone());
                },
                ChangeType::Deletion => {
                    let key = row.data[self.table_index].clone();
                    //self.table.remove(key);
                },
            }
        }

        let graph = &(*dfg).data;
        let parent_index = *(*dfg).index_map.get(&self.name).unwrap();

        //let mut child_indices = Vec::new();

        for child_index in graph.neighbors(parent_index) {
            let next_change = {
                let edge_index = graph.find_edge(parent_index, child_index).unwrap();
                let edge_op: &Operation = (*dfg).data.edge_weight(edge_index).unwrap();
                (*edge_op).apply(change_ins.clone())
            };

            if !next_change.batch.is_empty() {
                let mut child_view = (graph.node_weight(child_index).unwrap()).borrow_mut();
                (*child_view).change_table(next_change, dfg);
            }
        }
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

    pub fn render(&self) -> String {
        self.to_string()
    }
}

//use ids in JSOn object
//be careful with schema
pub trait Operator {
    fn apply(&self, prev_change: Change) -> Vec<Change>; 
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum Operation {
    Selector(Selection),
    Projector(Projection),
    Aggregator(Aggregation)
}

//match self
impl Operator for Operation {
    fn apply(&self, prev_change: Vec<Change>) -> Vec<Change> { 
        match self {
            Operation::Selector(op) => op.apply(prev_change),
            Operation::Projector(op) => op.apply(prev_change),
            Operation::Aggregator(op) => op.apply(prev_change),
        }
    }
}

#[wasm_bindgen]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Selection {
    col_ind: usize,
    condition: DataType,
}

impl Operator for Selection {
    fn apply(&self, prev_change_vec: Vec<Change>) -> Vec<Change> {
        let next_change_vec = Vec::new();

        for change in prev_change_vec {
            let mut next_change = Change { typing: change.typing, batch: Vec::new()};

            for row in &(change.batch) {
                if row.data[self.col_ind] == self.condition {
                    next_change.batch.push((*row).clone());
                }
            }

            next_change_vec.push(next_change);
        }

        next_change_vec
    }
}

impl Selection {
    fn new(col_ind: usize, condition_js: &JsValue) -> Selection {
        let condition = DataType::from(condition_js);
        Selection { col_ind, condition}
    }
}

#[wasm_bindgen]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Projection {
    columns: Vec<usize>,
}

impl Operator for Projection {
    fn apply(&self, prev_change_vec: Vec<Change>) -> Vec<Change> {
        let next_change_vec = Vec::new();

        for change in prev_change_vec {
            let mut next_change = Change { typing: change.typing, batch: Vec::new()};

            for row in &(change.batch) {
                let mut changed_row = Row::new(Vec::new());

                for index in &self.columns {
                    changed_row.data.push(row.data[*index].clone());
                }

                next_change.batch.push(changed_row);
            }

            next_change_vec.push(next_change);
        }

        next_change_vec
    }
}

impl Projection {
    fn process_into_col(some_iterable: &JsValue) -> Result<Vec<usize>, JsValue>  {
        let mut columns = Vec::new();
        let iterator = js_sys::try_iter(some_iterable)?.ok_or_else(|| {
            "need to pass iterable JS values!"
        })?;

        for x in iterator {
            let mut x = x?;

            let insert = x.as_f64();

            if insert.is_some() {
                let final_insert = insert.unwrap() as usize;
                columns.push(final_insert);
            }
        }

        Ok(columns)
    }

    fn new(some_iterable: &JsValue) -> Projection {
        let mut columns = match Self::process_into_col(&some_iterable) {
            Ok(proj) => proj,
            Err(err) => Vec::new(),
        };  

        Projection { columns }
    }
}

pub enum FuncType {
    SUM,
    COUNT
}

//ASSUMING ONLY SUM, could make a struct for functions
#[wasm_bindgen]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Aggregation {
    group_by_col: Vec<usize>,
    function: FuncType,
}

//incremental aggregation i guess
//what about duplication elimination??
impl Operator for Aggregation {
    fn apply(&self, prev_change: Vec<Change>) -> Change {


        let mut next_change = Change { typing: ChangeType::Update, batch: Vec::new()};
        
        for row in &(prev_change.batch) {
            match prev_change.typing {
                ChangeType::Insertion => {
                    self.current_sum = self.current_sum + row.data[self.col_ind];
                }
                ChangeType::Deletion => self.current_sum = self.current_sum - row.data[self.col_ind];
                ChangeType::Update => 
            }
            let mut changed_row = Row::new(Vec::new());

            // for index in &self.columns {
            //     changed_row.data.push(row.data[*index].clone());
            // }

            next_change.batch.push(changed_row);
        }

        next_change
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct DataFlowGraph {
    data: Graph<RefCell<View>, Operation>,
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
    pub fn new(json: String) -> DataFlowGraph {
        console::log_1(&"Hello using web-sys".into());
        let mut data = Graph::new();
        let mut index_map = HashMap::new();
        
        let obj: Value = serde_json::from_str(&json).unwrap();
        console::log_1(&"obj".into());

        let nodes: Vec<Value> = serde_json::from_value(obj["nodes"].clone()).unwrap();
        console::log_1(&"nodes".into());

        //can't deserialize into struct map??
        for node in nodes {
            console::log_1(&"begin node".into());
            console::log_1(&"viewJSON1".into());
            let v: ViewJSON = serde_json::from_value(node).unwrap();
            console::log_1(&"viewJSON".into());
            let name = v.name.clone();
            let view = View::from(v);
            let index = data.add_node(RefCell::new(view));
            index_map.insert(name, index.clone());
            console::log_1(&"node".into());
        } 

        let edges: Vec<Value> = serde_json::from_value(obj["edges"].clone()).unwrap();
        console::log_1(&"edges".into());

        for edge in &edges {
            let pi: usize = serde_json::from_value(edge["parentindex"].clone()).unwrap();
            console::log_1(&"pi".into());
            let pni = NodeIndex::new(pi);
            let ci: usize = serde_json::from_value(edge["childindex"].clone()).unwrap();
            console::log_1(&"ci".into());
            let cni = NodeIndex::new(ci);
            let op: Operation = serde_json::from_value(edge["operation"].clone()).unwrap();
            console::log_1(&"operators".into());

            data.add_edge(pni, cni, op);
            console::log_1(&"edge".into());
        }

        console::log_1(&"finished".into());

        DataFlowGraph { data, index_map }
    }

    // pub fn extend(&mut self, parent: View, child: View, operator: Operation) {
    //     let first_name = parent.name.clone();
    //     let first = self.data.add_node(RefCell::new(parent));
    //     self.index_map.insert(first_name, first.clone());

    //     let second_name = child.name.clone();
    //     let second = self.data.add_node(RefCell::new(child));
    //     self.index_map.insert(second_name, second.clone());

    //     self.data.add_edge(first, second, operator);
    // }

    //can you guarantee one operator between views 
    pub fn process_insert(&mut self, view_string: String, row_ins_js: &JsValue) {
        let view_name = *(self.index_map.get(&view_string).unwrap());
        let mut view_to_edit = self.data.node_weight(view_name).unwrap().borrow_mut();

        let mut row_ins_rust = match Self::process_into_row(&(*view_to_edit), row_ins_js) {
            Ok(row) => row,
            Err(err) => Row::new(Vec::new()),
        };  

        let change_ins = Change::new(ChangeType::Insertion, vec![row_ins_rust]);
        
        view_to_edit.change_table(change_ins, self);
    }

    // pub fn process_delete(&mut self, view_string: String, row_del_js: &JsValue) {
    //     let view_to_edit = self.data.node_weight(*self.index_map.get(&view_string).unwrap()).unwrap();

    //     let mut row_del_rust = match Self::process_into_row(view_to_edit, row_del_js) {
    //         Ok(row) => row,
    //         Err(err) => Row::new(Vec::new()),
    //     };  

    //     let change_ins = Change::new(ChangeType::Deletion, vec![row_del_rust]);
        
    //     view_to_edit.delete(change_ins, &mut self); 
    // }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn node_count(&self) -> usize {
        self.data.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.data.node_count()
    }
}

// #[wasm_bindgen_test]
// fn unit_test() {
//     // this test can access private members of structs defined in this module (file)
//     assert_eq!(1 + 1, 2);
// }


