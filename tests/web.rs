//! Test suite for node.js

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use noria_clientside::graph::DataFlowGraph;
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

fn selection() {

}

#[wasm_bindgen_test]
fn aggregation_count() {
    // Graph:
    //  1) root node (article_id, article_tag, article_author, article_votecount)
    //  2) aggregation node, group by (article_tag, article_author), aggregate over
    //     article_votecount
    //  3) leaf node, keyed by article_author
    //
    // Purpose: look up the articles by a given author with vote counts by tag.
    // Equivalent SQL query: SELECT article_author, article_tag, count(article_votecount)
    //                          FROM table
    //                          WHERE article_author = ?
    //                          GROUP BY article_tag, article_author;
    let graph = r##"..."##;

    let g = DataFlowGraph::new(graph.to_owned());
    assert_eq!(g.node_count(), 3);
    assert_eq!(g.edge_count(), 2);

    // Inputs: (article_id, article_tag, article_author, article_votecount)
    // 1,"cats","bob",5
    // 2,"cats","alice",10
    // 3,"cats","alice",7
    // 4,"dogs","bob",2
    // 5,"dogs","alice",9
    //
    // Expected results:
    //
    // Look up for "alice":
    //  ("alice","cats",2)
    //  ("alice","dogs",1)
    //
    // Lookup for "bob":
    //  ("bob","cats",1)
    //  ("bob","dogs",1)
}

#[wasm_bindgen_test]
fn aggregation_sum() {
    // Graph:
    //  1) root node (article_id, article_tag, article_author, article_votecount)
    //  2) aggregation node, group by (article_tag, article_author), aggregate over
    //     article_votecount
    //  3) leaf node, keyed by article_author
    //
    // Purpose: look up the articles by a given author with vote summed by tag.
    // Equivalent SQL query: SELECT article_author, article_tag, sum(article_votecount)
    //                          FROM table
    //                          WHERE article_author = ?
    //                          GROUP BY article_tag, article_author;
    let graph = r##"..."##;

    let g = DataFlowGraph::new(graph.to_owned());
    assert_eq!(g.node_count(), 3);
    assert_eq!(g.edge_count(), 2);

    // Inputs: (article_id, article_tag, article_author, article_votecount)
    // 1,"cats","bob",5
    // 2,"cats","alice",10
    // 3,"cats","alice",7
    // 4,"dogs","bob",2
    // 5,"dogs","alice",9
    //
    // Expected results:
    //
    // Look up for "alice":
    //  ("alice","cats",17)
    //  ("alice","dogs",9)
    //
    // Lookup for "bob":
    //  ("bob","cats",5)
    //  ("bob","dogs",2)
}
