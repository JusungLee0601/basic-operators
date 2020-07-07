//! Test suite for node.js

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use noria_clientside::DataFlowGraph;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn create_simple_graph() {
    let graph = r##"{
        "nodes": [{
            "name": "first",
            "columns": ["Article", "Count"],
            "schema": ["Text", "Int"],
            "table_index": 0
        }, {
            "name": "second",
            "columns": ["Article", "Count"],
            "schema": ["Text", "Int"],
            "table_index": 0
        }, {
            "name": "third",
            "columns": ["Article"],
            "schema": ["Text"],
            "table_index": 0
        }],
        "edges": [{
            "parentindex": 0,
            "childindex": 1,
            "operation": {
                "t": "Selector",
                "c": {
                    "col_ind": 0,
                    "condition": {
                        "t": "Text",
                        "c": "dummy"
                    }
                }
            }
        },
        {
            "parentindex": 1,
            "childindex": 2,
            "operation": {
                "t": "Projector",
                "c": {
                    "columns": [0]
                }
            }
        }]
    }"##;

    let g = DataFlowGraph::new(graph.to_owned());
    assert_eq!(g.node_count(), 3);
    assert_eq!(g.edge_count(), 2);
}
