//Operator trait
pub trait Operator {
    /// Returns Vec of Changes after operator conditions applied
    fn apply(&mut self, prev_change: Vec<Change>) -> Vec<Change>; 

    /// Takes a set of Changes and propogates the Changes recursively through nodes children
    /// calls apply to generate new Change to send downward
    fn process_change(&mut self, change: Vec<Change>, dfg: &DataFlowGraph, parent_index: NodeIndex) { 
        let next_change = self.apply(change);
        let graph = &(*dfg).data;
        let neighbors_iterator = graph.neighbors(parent_index);

        for child_index in neighbors_iterator {
            let child_cell = (*graph).node_weight(child_index).unwrap();
            let mut child_ref_mut = child_cell.borrow_mut();

            (*child_ref_mut).process_change(next_change.clone(), dfg, child_index);
        }
    }
}

//Operation Enum, used for typing
//I think this was originally for exposing operators to JS, but now that operator stuff is handled
//Rust side I'm not sure if this still needs to exist, I can give it a try to switch
#[derive(Debug, Clone)]
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

    fn process_change(&mut self, change: Vec<Change>, dfg: &DataFlowGraph, parent_index: NodeIndex) { 
        match self {
            Operation::Selector(op) => op.process_change(change, dfg, parent_index),
            Operation::Projector(op) => op.process_change(change, dfg, parent_index),
            Operation::Aggregator(op) => op.process_change(change, dfg, parent_index),
            Operation::Rootor(op) => op.process_change(change, dfg, parent_index),
            Operation::Leafor(op) => op.process_change(change, dfg, parent_index),
        }
    }
}
