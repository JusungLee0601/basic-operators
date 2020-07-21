use serde::{Deserialize, Serialize};

use crate::units::change::Change;
use crate::viewsandgraphs::dfg::DataFlowGraph;
use petgraph::graph::NodeIndex;
use crate::operators::Operator;
use wasm_bindgen::prelude::*;

//Root Operator
//root_id assumed unique, used for NodeIndex mapping to find in graph
#[wasm_bindgen]
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Root {
    root_id: String,
}

//Operator Trait for Root
impl Operator for Root {
    /// Identity, doesn't need to modify change as Root
    fn apply(&mut self, prev_change_vec: Vec<Change>) -> Vec<Change> {
        prev_change_vec
    }

    /// For Root, process change does not "apply"/change the initial set of Changes as it is the Root
    fn process_change(&mut self, change: Vec<Change>, dfg: &DataFlowGraph, parent_index: NodeIndex, self_index: NodeIndex) { 
        let graph = &(*dfg).data;
        let neighbors_iterator = graph.neighbors(self_index);

        for child_index in neighbors_iterator {
            let child_cell = (*graph).node_weight(child_index).unwrap();
            let mut child_ref_mut = child_cell.borrow_mut();

            //the self become parent, child becomes self
            (*child_ref_mut).process_change(change.clone(), dfg, self_index, child_index);
        }
    }
}