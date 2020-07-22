pub mod aggregation;
pub mod innerjoin;
pub mod projection;
pub mod leaf;
pub mod root;
pub mod selection;
pub mod operation;

use crate::units::change::Change;
use crate::viewsandgraphs::dfg::DataFlowGraph;
use petgraph::graph::NodeIndex;
use crate::types::changetype::ChangeType;

use web_sys::console;

//Operator trait
pub trait Operator {
    /// Returns Vec of Changes after operator conditions applied
    fn apply(&mut self, prev_change: Vec<Change>) -> Vec<Change>; 

    /// Takes a set of Changes and propogates the Changes recursively through nodes children
    /// calls apply to generate new Change to send downward
    fn process_change(&mut self, change: Vec<Change>, dfg: &DataFlowGraph, parent_index: NodeIndex, self_index: NodeIndex) { 
        console::log_1(&"pc".into()); 
        let next_change = self.apply(change);
        let graph = &(*dfg).data;
        let neighbors_iterator = graph.neighbors(self_index);

        console::log_1(&"pc2".into());

        for child_index in neighbors_iterator {
            console::log_1(&"pc3".into());
            let child_cell = (*graph).node_weight(child_index).unwrap();
            let mut child_ref_mut = child_cell.borrow_mut();
            console::log_1(&"pc4".into());

            (*child_ref_mut).process_change(next_change.clone(), dfg, self_index, child_index);
            console::log_1(&"pc5".into());
        }
    }
}
