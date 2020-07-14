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

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


//-----------------------------Types--------------------------------//

//Data, basic unit of info in tables
#[derive(Debug)]
#[derive(Clone, Hash, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum DataType {
    None,
    Int(i32),
    Text(String)
}

//from conversion, &JsValue->DataType
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

//from conversion, JsValue->DataType
impl From<JsValue> for DataType {
    fn from(item: JsValue) -> Self {
        if (item).as_f64().is_some()  {
            DataType::Int(item.as_f64().unwrap() as i32)
        } else if ( item).as_string().is_some()  {
            DataType::Text(item.as_string().unwrap())
        } else {
            DataType::None
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

//Schema, for Views only
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum SchemaType {
    None,
    Int,
    Text
}

//from conversion, JsValue->SchemaType
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

//Change, delineates Insertion vs Deletion
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeType {
    Insertion,
    Deletion
}

//Supposed to be for Aggregation
// pub enum FuncType {
//     SUM(Vec<usize>),
//     COUNT
// }

//-----------------------------"Units"--------------------------------//

//Row, allows 2d representation in tables
#[wasm_bindgen]     
#[derive(Debug)]
#[derive(Hash, Eq, PartialEq, Clone)]
#[derive(Serialize, Deserialize)]
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

//Change, typing shows ChangeType, batch holds multiple potential changes
#[derive(Debug, Clone, PartialEq)]
pub struct Change {
    typing: ChangeType,
    batch: Vec<Row>
}

//Change functions
impl Change {
    //constructor
    pub fn new(typing: ChangeType, batch: Vec<Row>) -> Change {
        Change { typing, batch }
    }
}

//-----------------------------Views (from the 6)--------------------------------//

//ViewJSON
//View without table for graph construction
//I don't think this is needed, haven't tested without it though
#[derive(Serialize, Deserialize)]
pub struct ViewJSON {
    name: String,
    columns: Vec<String>,
    schema: Vec<SchemaType>,
    table_index: usize,
}

//from conversion, ViewJSON -> View
//ditto of newJSON function in View
impl From<ViewJSON> for View {
    fn from(item: ViewJSON) -> Self {
        let view = View::newJSON(item.name, item.table_index, item.columns, item.schema);

        view
    }
}

//View
//name: string name, assumed unique
#[wasm_bindgen]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct View {
    name: String,
    column_names: Vec<String>,
    schema: Vec<SchemaType>,
    key_index: usize,
    table: HashMap<DataType, Row>
}

//displays View
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

//View functions, unexposed
impl View {
    /// Returns View assuming empty table
    pub fn newJSON(name: String, key_index: usize, column_names: Vec<String>, 
        schema: Vec<SchemaType>) -> View {
        let table = HashMap::new();

        View {name, key_index, column_names, schema, table}
    }

    /// Changes View's table given a vector of Changes
    pub fn change_table(&mut self, change_vec: Vec<Change>) {
        for change in &change_vec {
            for row in &change.batch {
                match change.typing {
                    ChangeType::Insertion => {
                        let key = row.data[self.table_index].clone();
                        self.table.insert(key, row.clone());
                    },
                    ChangeType::Deletion => {
                        let key = row.data[self.table_index].clone();
                        self.table.remove(&key);
                    },
                }
            }
        }
    }
}

//View functions, exposed
#[wasm_bindgen]
impl View {
    /// Returns View as a String
    pub fn render(&self) -> String {
        self.to_string()
    }
}

//-----------------------------Operators and Operations--------------------------------//

//Operator trait
pub trait Operator {
    /// Returns Vec of Changes after operator conditions applied
    fn apply(&mut self, prev_change: Vec<Change>) -> Vec<Change>; 

    /// Takes a set of Changes and propogates the Changes recursively through nodes children
    /// calls apply to generate new Change to send downward
    fn process_change(&mut self, change: Vec<Change>, dfg: &mut DataFlowGraph, parent_index: NodeIndex) { 
        let processed_change = self.apply(change);
        let graph = (*dfg).data;
        let neighbors_iterator = graph.neighbors(parent_index);

        for child_index in neighbors_iterator {
            let mut child_op = graph.node_weight_mut(child_index).unwrap().borrow_mut();

            (*child_op).process_change(processed_change, dfg, child_index);
        }
    }
}

//Operation Enum, used for typing
//I think this was originally for exposing operators to JS, but now that operator stuff is handled
//Rust side I'm not sure if this still needs to exist, I can give it a try to switch
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum Operation {
    Selector(Selection),
    Projector(Projection),
    Aggregator(Aggregation),
    Rootor(Root),
    Leafor(Leaf),
}

//Operator Trait for Operation Enum
impl Operator for Operation {
    fn apply(&mut self, prev_change: Vec<Change>) -> Vec<Change> { 
        match self {
            Operation::Selector(op) => op.apply(prev_change),
            Operation::Projector(op) => op.apply(prev_change),
            Operation::Aggregator(op) => op.apply(prev_change),
            Operation::Rootor(op) => op.apply(prev_change),
            Operation::Leafor(op) => op.apply(prev_change),
        }
    }

    fn process_change(&mut self, change: Vec<Change>, dfg: &mut DataFlowGraph, parent_index: NodeIndex) { 
        match self {
            Operation::Selector(op) => op.process_change(change, dfg, parent_index),
            Operation::Projector(op) => op.process_change(change, dfg, parent_index),
            Operation::Aggregator(op) => op.process_change(change, dfg, parent_index),
            Operation::Rootor(op) => op.process_change(change, dfg, parent_index),
            Operation::Leafor(op) => op.process_change(change, dfg, parent_index),
        }
    }
}

//Root Operator
//root_id assumed unique, used for NodeIndex mapping to find in graph
#[wasm_bindgen]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Root {
    root_id: String,
}

//Operator Trait for Root
impl Operator for Root {
    /// Dummy apply, doesn't do anything :( feels very unnecessary
    fn apply(&mut self, prev_change_vec: Vec<Change>) -> Vec<Change> {
        prev_change_vec
    }

    /// For Root, process change does not "apply"/change the initial set of Changes as it is the Root
    fn process_change(&mut self, change: Vec<Change>, dfg: &mut DataFlowGraph, parent_index: NodeIndex) { 
        let graph = (*dfg).data;
        let neighbors_iterator = graph.neighbors(parent_index);

        for child_index in neighbors_iterator {
            let mut child_op = graph.node_weight_mut(child_index).unwrap().borrow_mut();

            child_op.process_change(change, dfg, child_index);
        }
    }
}

//Leaf Operator
//stored view is what is "accessed" by JS
#[wasm_bindgen]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Leaf {
    mat_view: View,
}

//Operator Trait for Leaf
impl Operator for Leaf {
    ///Apply doesn't actually modify Change, inserts into mat_view table, returns unchanged input
    fn apply(&mut self, prev_change_vec: Vec<Change>) -> Vec<Change> {
        self.mat_view.change_table(prev_change_vec);

        prev_change_vec
    }

    /// Doesn't apply to the rest of the operators as it is the Leaf
    fn process_change(&mut self, change: Vec<Change>, dfg: &mut DataFlowGraph, parent_index: NodeIndex) { 
        self.apply(change);
    }
}

//Selection Operator
#[wasm_bindgen]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Selection {
    col_ind: usize,
    condition: DataType,
}

//Operator Trait for Selection
impl Operator for Selection {
    fn apply(&mut self, prev_change_vec: Vec<Change>) -> Vec<Change> {
        let mut next_change_vec = Vec::new();

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


//Projection Operator
#[wasm_bindgen]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Projection {
    columns: Vec<usize>,
}

//Operator Trait for Projection
impl Operator for Projection {
    fn apply(&mut self, prev_change_vec: Vec<Change>) -> Vec<Change> {
        let mut next_change_vec = Vec::new();

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

//Aggregation Operator
//group_by_col is ordered lowest to highest
#[wasm_bindgen]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Aggregation {
    group_by_col: Vec<usize>,
    //function: FuncType,
    key_index: usize,
    state: HashMap<Vec<DataType>, Row>,
}

//Operator Trait for Aggregation
//implements hard coded length for count, no sum or func matching yet
//also does not group changes first, which would be a lot cleaner, but harder to implement
impl Operator for Aggregation {
    fn apply(&mut self, prev_change_vec: Vec<Change>) -> Vec<Change> {
        let mut next_change_vec = Vec::new();

        //multiple Insertions and Deletions
        for change in prev_change_vec {
            match change.typing {
                ChangeType::Insertion => {
                    //multiple rows in a single Change
                    for row in &(change.batch) {
                        //form key to access aggregates in state
                        let mut temp_key = Vec::new();
                        
                        for index in &self.group_by_col {
                            temp_key.push(row.data[*index].clone());
                        } 

                        match self.state.get_mut(&temp_key) {
                            None => {
                                //create new row to insert with only the group by columns
                                let mut new_row_vec = Vec::new();

                                for index in &self.group_by_col {
                                    new_row_vec.push(row.data[*index]);
                                } 

                                //copy for key in hashmap
                                let new_row_key = new_row_vec.clone();

                                //since its a new key, gets its own count
                                new_row_vec.push(DataType::Int(1));

                                let new_row = Row::new(new_row_vec);

                                //apply changes to operator's internal state
                                self.state.insert(new_row_key, new_row.clone());

                                let mut change_rows = Vec::new();
                                change_rows.push(new_row.clone());
                            
                                //send insertion change downstream
                                let new_group_change = Change::new(ChangeType::Insertion, change_rows);
                                next_change_vec.push(new_group_change); 
                            },
                            Some(row_to_incr) => {
                                //sends deletion change downstream
                                let mut change_rows_del = Vec::new();
                                change_rows_del.push(row_to_incr.clone());

                                let delete_old = Change::new(ChangeType::Deletion, change_rows_del);
                                next_change_vec.push(delete_old);

                                //increments count in state
                                let len = &row_to_incr.data.len();
                                let new_count = match &row_to_incr.data[len - 1] {
                                    DataType::Int(count) => count + 1,
                                    _ => 0,
                                };
                                row_to_incr.data[len - 1] = DataType::Int(new_count);

                                //sends insertion change downstream
                                let mut change_rows_ins = Vec::new();
                                change_rows_ins.push(row_to_incr.clone());

                                let insert_new = Change::new(ChangeType::Insertion, change_rows_ins);
                                next_change_vec.push(insert_new);
                            },
                        }
                    }
                }
                //In this model, we assume that deletions will always match with one aggregated row
                ChangeType::Deletion => {
                    //multiple rows in a single Change
                    for row in &(change.batch) {
                        let mut temp_key = Vec::new();
                        
                        for index in &self.group_by_col {
                            temp_key.push(row.data[*index].clone());
                        } 

                        match self.state.get_mut(&temp_key) {
                            Some(row_to_decr) => {
                                //sends deletion change downstream
                                let mut change_rows_del = Vec::new();
                                change_rows_del.push(row_to_decr.clone());

                                let delete_old = Change::new(ChangeType::Deletion, change_rows_del);
                                next_change_vec.push(delete_old);

                                //decrements count in state
                                let len = &row_to_decr.data.len();
                                let new_count = match &row_to_decr.data[len - 1] {
                                    DataType::Int(count) => count - 1,
                                    _ => 0,
                                };
                                row_to_decr.data[len - 1] = DataType::Int(new_count);

                                //sends insertion change downstream if not decremented to 0
                                if new_count > 0 {
                                    let mut change_rows_ins = Vec::new();
                                    change_rows_ins.push(row_to_decr.clone());

                                    let insert_new = Change::new(ChangeType::Insertion, change_rows_ins);
                                    next_change_vec.push(insert_new);
                                }
                            },
                            None => {}
                        }
                    }
                }
            }
        }

        next_change_vec
    }
}

//-----------------------------Graph--------------------------------//

//DataFlowGraph
//root_id_map: map of root_id's to their NodeIndexes
//leaf_id_vec: just a list of leaf ids, used for printing
#[wasm_bindgen]
#[derive(Debug)]
pub struct DataFlowGraph {
    data: Graph<RefCell<Operation>, ()>,
    root_id_map: HashMap<String, NodeIndex>,
    leaf_id_vec: Vec<NodeIndex>,
}

//Displays DFG
impl fmt::Display for DataFlowGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

//DFG functions, unexposed
impl DataFlowGraph { 
    /// Returns a Row from any JSValue, preferably an array
    pub fn process_into_row(some_iterable: &JsValue)
            -> Result<Row, JsValue> {
        let mut row_vec = Vec::new();

        let iterator = js_sys::try_iter(some_iterable)?.ok_or_else(|| {
            "need to pass iterable JS values!"
        })?;

        let mut count = 0;

        for x in iterator {
            let mut x = x?;

            row_vec.push(DataType::from(x));
        }

        Ok(Row::new(row_vec))
    }
}

//DFG Functions, exposed
#[wasm_bindgen]
impl DataFlowGraph { 
    /// Returns DFG from JSON input
    pub fn new(json: String) -> DataFlowGraph {
        let mut data = Graph::new();
        let mut root_id_map = HashMap::new();
        let mut leaf_id_vec = Vec::new();
        
        let obj: Value = serde_json::from_str(&json).unwrap();

        let operators: Vec<Value> = serde_json::from_value(obj["operators"].clone()).unwrap();

        //can't deserialize into struct map??
        for op_val in operators {
            let op: Operation = serde_json::from_value(op_val).unwrap();

            let index = data.add_node(RefCell::new(op));

            match op {
                Operation::Rootor(inner_op) => {
                    let option = root_id_map.insert(inner_op.root_id, index);
                },
                Operation::Leafor(inner_op) => leaf_id_vec.push(index),
                _ => (),
            }
        } 

        let edges: Vec<Value> = serde_json::from_value(obj["edges"].clone()).unwrap();

        for edge in &edges {
            let pi: usize = serde_json::from_value(edge["parentindex"].clone()).unwrap();
            let pni = NodeIndex::new(pi);
            let ci: usize = serde_json::from_value(edge["childindex"].clone()).unwrap();
            let cni = NodeIndex::new(ci);

            data.extend_with_edges(&[(pni, cni)]);
        }

        DataFlowGraph { data, root_id_map, leaf_id_vec }
    }

    /// Applies inserts and deletions sent to a specified Root, propogates them
    /// through graph relying on the recursive operator calls
    pub fn change_to_root(&mut self, root_string: String, row_ins_js: &JsValue) {
        let root_node_index = *(self.root_id_map.get(&root_string).unwrap());
        let mut root_op = self.data.node_weight(root_node_index).unwrap().borrow_mut();

        let mut row_ins_rust = match Self::process_into_row(&row_ins_js) {
            Ok(row) => row,
            Err(err) => Row::new(Vec::new()),
        };  

        let change_ins = Change::new(ChangeType::Insertion, vec![row_ins_rust]);
        let mut change_vec = vec![change_ins];
        
        root_op.process_change(change_vec, self, root_node_index);
    }

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


