use serde::{Deserialize, Serialize};
pub use crate::units::Change as Change;
pub use crate::view as View;

//Leaf Operator
//stored view is what is "accessed" by JS
#[wasm_bindgen]
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Leaf {
    mat_view: View,
}

//Operator Trait for Leaf
impl Operator for Leaf {
    ///Apply doesn't actually modify Change, inserts into mat_view table, returns unchanged input
    fn apply(&mut self, prev_change_vec: Vec<Change>) -> Vec<Change> {
        self.mat_view.change_table(prev_change_vec);

        Vec::new()
    }

    /// Doesn't apply to the rest of the operators as it is the Leaf
    fn process_change(&mut self, change: Vec<Change>, dfg: &DataFlowGraph, parent_index: NodeIndex) { 
        self.apply(change);
    }
}