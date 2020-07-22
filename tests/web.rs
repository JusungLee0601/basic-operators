  //! Test suite for node.js

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
extern crate js_sys;
use wasm_bindgen_test::*;
use noria_clientside::viewsandgraphs::dfg::DataFlowGraph;
use noria_clientside::units::change::Change;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn create_simple_graph() {
    let graph = r##"{
        "operators": [
            {
                "t": "Rootor",
                "c": {
                    "root_id": "first"
                }
            },
            {
                "t": "Selector",
                "c": {
                    "col_ind": 1,
                    "condition": {
                        "t": "Int",
                        "c": 50
                    } 
                }
            },
            {
                "t": "Leafor",
                "c": {
                    "mat_view": {
                        "name": "first",
                        "column_names": ["Article", "Count", "Agg count"],
                        "schema": ["Text", "Int", "Int"],
                        "key_index": 0
                    }
                }
            }
        ],
        "edges": [{
            "parentindex": 0,
            "childindex": 1,
        }, {
            "parentindex": 0,
            "childindex": 2,
        },
        {
            "parentindex": 1,
            "childindex": 3,
        },
        {
            "parentindex": 3,
            "childindex": 4,
        },
        {
            "parentindex": 2,
            "childindex": 5,
        }]
    };"##;

    //let g = DataFlowGraph::new(graph.to_owned());
    //assert_eq!(g.node_count(), 3);
    //assert_eq!(g.edge_count(), 2);
}

#[wasm_bindgen_test]
fn selection_unit() {
    let unit_graph = r##"{
        "operators": [
            {
                "t": "Rootor",
                "c": {
                    "root_id": "first"
                }
            },
            {
                "t": "Selector",
                "c": {
                    "col_ind": 1,
                    "condition": {
                        "t": "Int",
                        "c": 50
                    } 
                }
            },
            {
                "t": "Selector",
                "c": {
                    "col_ind": 0,
                    "condition": {
                        "t": "Text",
                        "c": "Doomsday"
                    } 
                }
            },
            {
                "t": "Leafor",
                "c": {
                    "mat_view": {
                        "name": "Int",
                        "column_names": ["Article", "Count"],
                        "schema": ["Text", "Int"],
                        "key_index": 0
                    }
                }
            },
            {
                "t": "Leafor",
                "c": {
                    "mat_view": {
                        "name": "Text",
                        "column_names": ["Article", "Count"],
                        "schema": ["Text", "Int"],
                        "key_index": 0
                    }
                }
            }
        ],
        "edges": [{
            "parentindex": 0,
            "childindex": 1
        }, {
            "parentindex": 0,
            "childindex": 2
        },
        {
            "parentindex": 1,
            "childindex": 3
        },
        {
            "parentindex": 2,
            "childindex": 4
        }]
    }"##;

    let g_unit = DataFlowGraph::new(unit_graph.to_owned());

    assert_eq!(g_unit.node_count(), 5);
    assert_eq!(g_unit.edge_count(), 4);

    let sin_row_ins = r##"{
        "typing": "Insertion",
        "batch": [
            {
                "data": [
                    {
                        "t": "Text",
                        "c": "Article 1"
                    },
                    {
                        "t": "Int",
                        "c": 49
                    }
                ]
            } 
        ]
    }"##;

    let mult_row_ins = r##"{
        "typing": "Insertion",
        "batch": [
            {
                "data": [
                    {
                        "t": "Text",
                        "c": "Doomsday"
                    },
                    {
                        "t": "Int",
                        "c": 50
                    }
                ]
            },
            {
                "data": [
                    {
                        "t": "Text",
                        "c": "Doomsday"
                    },
                    {
                        "t": "Int",
                        "c": 49
                    }
                ]
            },
            {
                "data": [
                    {
                        "t": "Text",
                        "c": "Thursday"
                    },
                    {
                        "t": "Int",
                        "c": 50
                    }
                ]
            }
        ]
    }"##;

    let sin_row_del = r##"{
        "typing": "Deletion",
        "batch": [
            {
                "data": [
                    {
                        "t": "Text",
                        "c": "Thursday"
                    },
                    {
                        "t": "Int",
                        "c": 50
                    }
                ]
            } 
        ]
    }"##;

    g_unit.change_to_root_json("first".to_owned(), sin_row_ins.to_owned());
    g_unit.change_to_root_json("first".to_owned(), mult_row_ins.to_owned());
    g_unit.change_to_root_json("first".to_owned(), sin_row_del.to_owned());

    assert_eq!(g_unit.leaf_counts(), vec![1, 2]);
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
    //let graph = r##"..."##;

    //let g = DataFlowGraph::new(graph.to_owned());
    //assert_eq!(g.node_count(), 3);
    //assert_eq!(g.edge_count(), 2);

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
    //let graph = r##"..."##;

    //let g = DataFlowGraph::new(graph.to_owned());
    //assert_eq!(g.node_count(), 3);
    //assert_eq!(g.edge_count(), 2);

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
