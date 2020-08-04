mod utils;
extern crate js_sys;

#[macro_use]
extern crate serde_derive;

extern crate wasm_timer;
use wasm_timer::Instant;

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate log;

use std::net::TcpListener;
use std::thread::spawn;  

use wasm_bindgen::prelude::*;
// use std::fmt;
// use std::collections::HashMap;
// use petgraph::graph::Graph;
// use petgraph::graph::NodeIndex;
// use std::cell::{RefCell, RefMut};
// use std::cell::Ref;
// use serde::{Deserialize, Serialize};
// use serde_json::Value;
// use web_sys::console;

pub mod operators;
pub mod types;
pub mod units;
pub mod viewsandgraphs;

use crate::viewsandgraphs::dfg::DataFlowGraph;
use petgraph::graph::NodeIndex;
use crate::types::datatype::DataType;
use crate::units::row::Row;
use crate::operators::operation::Operation::Leafor;
use web_sys::console;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

//WEBSOCKET CONNECTION ----------------------------------------------------------------------------


//#[wasm_bindgen(start)]
// pub fn ws_connect() -> Result<(), JsValue> {
//     // Connect to an echo server
//     let ws = WebSocket::new("ws://localhost:3012/socket")?;
//     // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
//     ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
//     // create callback
//     let cloned_ws = ws.clone();
//     let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
//         // Handle difference Text/Binary,...
//         if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
//             console_log!("message event, received arraybuffer: {:?}", abuf);
//             let array = js_sys::Uint8Array::new(&abuf);
//             let len = array.byte_length() as usize;
//             console_log!("Arraybuffer received {}bytes: {:?}", len, array.to_vec());
//             // here you can for example use Serde Deserialize decode the message
//             // for demo purposes we switch back to Blob-type and send off another binary message
//             cloned_ws.set_binary_type(web_sys::BinaryType::Blob);
//             match cloned_ws.send_with_u8_array(&vec![5, 6, 7, 8]) {
//                 Ok(_) => console_log!("binary message successfully sent"),
//                 Err(err) => console_log!("error sending message: {:?}", err),
//             }
//         } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
//             console_log!("message event, received blob: {:?}", blob);
//             // better alternative to juggling with FileReader is to use https://crates.io/crates/gloo-file
//             let fr = web_sys::FileReader::new().unwrap();
//             let fr_c = fr.clone();
//             // create onLoadEnd callback
//             let onloadend_cb = Closure::wrap(Box::new(move |_e: web_sys::ProgressEvent| {
//                 let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
//                 let len = array.byte_length() as usize;
//                 console_log!("Blob received {}bytes: {:?}", len, array.to_vec());
//                 // here you can for example use the received image/png data
//             })
//                 as Box<dyn FnMut(web_sys::ProgressEvent)>);
//             fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
//             fr.read_as_array_buffer(&blob).expect("blob not readable");
//             onloadend_cb.forget();
//         } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
//             console_log!("message event, received Text: {:?}", txt);
//         } else {
//             console_log!("message event, received Unknown: {:?}", e.data());
//         }
//     }) as Box<dyn FnMut(MessageEvent)>);
//     // set message event handler on WebSocket
//     ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
//     // forget the callback to keep it alive
//     onmessage_callback.forget();

//     let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
//         console_log!("error event: {:?}", e);
//     }) as Box<dyn FnMut(ErrorEvent)>);
//     ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
//     onerror_callback.forget();

//     let cloned_ws = ws.clone();
//     let onopen_callback = Closure::wrap(Box::new(move |_| {
//         console_log!("socket opened");
//         match cloned_ws.send_with_str("ping") {
//             Ok(_) => console_log!("message successfully sent"),
//             Err(err) => console_log!("error sending message: {:?}", err),
//         }
//         // send off binary message
//         match cloned_ws.send_with_u8_array(&vec![0, 1, 2, 3]) {
//             Ok(_) => console_log!("binary message successfully sent"),
//             Err(err) => console_log!("error sending message: {:?}", err),
//         }
//     }) as Box<dyn FnMut(JsValue)>);
//     ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
//     onopen_callback.forget();

//     Ok(())
// }

//IN CRATE TESTING --------------------------------------------------------------------------------

//#[wasm_bindgen_test]
fn selection_unit_test() {
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

    assert_eq!(g_unit.leaf_counts(), vec![1, 1]);

    let mut leaf_op = g_unit.data.node_weight(NodeIndex::new(3)).unwrap().borrow_mut();

    match &*leaf_op {
        Leafor(leaf) => {
    
            let lri_row = (*leaf).mat_view.table.get(&DataType::Text("Doomsday".to_string())).unwrap();
            let test_row = Row::new(vec![DataType::Text("Doomsday".to_string()), DataType::Int(50)]);
        
            assert_eq!(*lri_row, test_row); 
        },
        _ => (),
    };

    let mut leaf_op_two = g_unit.data.node_weight(NodeIndex::new(4)).unwrap().borrow_mut();

    match &*leaf_op_two {
        Leafor(leaf) => {
    
            let lri_row = (*leaf).mat_view.table.get(&DataType::Text("Doomsday".to_string())).unwrap();
            let test_row = Row::new(vec![DataType::Text("Doomsday".to_string()), DataType::Int(49)]);
        
            assert_eq!(*lri_row, test_row); 
        },
        _ => (),
    };

}

//#[wasm_bindgen_test]
fn projection_unit_test() {
    let unit_graph = r##"{
        "operators": [
                {
                    "t": "Rootor",
                    "c": {
                        "root_id": "first"
                    }
                },
                {
                    "t": "Projector",
                    "c": {
                        "columns": [0, 1]
                    }
                },
                {
                    "t": "Projector",
                    "c": {
                        "columns": [2]
                    }
                },
                {
                    "t": "Leafor",
                    "c": {
                        "mat_view": {
                            "name": "Outside",
                            "column_names": ["Article", "Author"],
                            "schema": ["Text", "Text"],
                            "key_index": 0
                        }
                    }
                },
                {
                    "t": "Leafor",
                    "c": {
                        "mat_view": {
                            "name": "Inside",
                            "column_names": ["Count"],
                            "schema": ["Int"],
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
                        "c": "Article 0"
                    },
                    {
                        "t": "Text",
                        "c": "Author 0"
                    },
                    {
                        "t": "Int",
                        "c": 48
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
                        "c": "Article 1"
                    },
                    {
                        "t": "Text",
                        "c": "Author 1"
                    },
                    {
                        "t": "Int",
                        "c": 51
                    }
                ]
            },
            {
                "data": [
                    {
                        "t": "Text",
                        "c": "Article 2"
                    },
                    {
                        "t": "Text",
                        "c": "Author 2"
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
                        "c": "Article 3"
                    },
                    {
                        "t": "Text",
                        "c": "Author 3"
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
                        "c": "Article 3"
                    },
                    {
                        "t": "Text",
                        "c": "Author 3"
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

    assert_eq!(g_unit.leaf_counts(), vec![3, 3]);

    let mut leaf_op = g_unit.data.node_weight(NodeIndex::new(3)).unwrap().borrow_mut();

    match &*leaf_op {
        Leafor(leaf) => {
    
            let lri_row = (*leaf).mat_view.table.get(&DataType::Text("Article 0".to_string())).unwrap();
            let test_row = Row::new(vec![DataType::Text("Article 0".to_string()), DataType::Text("Author 0".to_string())]);
        
            assert_eq!(*lri_row, test_row); 

            let lri_row2 = (*leaf).mat_view.table.get(&DataType::Text("Article 1".to_string())).unwrap();
            let test_row2 = Row::new(vec![DataType::Text("Article 1".to_string()), DataType::Text("Author 1".to_string())]);

            assert_eq!(*lri_row2, test_row2);

            let lri_row3 = (*leaf).mat_view.table.get(&DataType::Text("Article 2".to_string())).unwrap();
            let test_row3 = Row::new(vec![DataType::Text("Article 2".to_string()), DataType::Text("Author 2".to_string())]);

            assert_eq!(*lri_row3, test_row3);
        },
        _ => (),
    };

    let mut leaf_op_two = g_unit.data.node_weight(NodeIndex::new(4)).unwrap().borrow_mut();

    match &*leaf_op_two {
        Leafor(leaf) => {
    
            let lri_row = (*leaf).mat_view.table.get(&DataType::Int(48)).unwrap();
            let test_row = Row::new(vec![DataType::Int(48)]);

            assert_eq!(*lri_row, test_row);

            let lri_row2 = (*leaf).mat_view.table.get(&DataType::Int(51)).unwrap();
            let test_row2 = Row::new(vec![DataType::Int(51)]);

            assert_eq!(*lri_row2, test_row2);

            let lri_row3 = (*leaf).mat_view.table.get(&DataType::Int(49)).unwrap();
            let test_row3 = Row::new(vec![DataType::Int(49)]);
        
            assert_eq!(*lri_row3, test_row3); 
        },
        _ => (),
    };

        
}

//#[wasm_bindgen_test]
fn aggregation_unit_test() {
    let unit_graph = r##"{
        "operators": [
                {
                    "t": "Rootor",
                    "c": {
                        "root_id": "first"
                    }
                },
                {
                    "t": "Aggregator",
                    "c": {
                        "group_by_col": [0, 1]
                    }
                },
                {
                    "t": "Aggregator",
                    "c": {
                        "group_by_col": [2]
                    }
                },
                {
                    "t": "Leafor",
                    "c": {
                        "mat_view": {
                            "name": "Outside",
                            "column_names": ["Article", "Author", "Count"],
                            "schema": ["Text", "Text", "Int"],
                            "key_index": 0
                        }
                    }
                },
                {
                    "t": "Leafor",
                    "c": {
                        "mat_view": {
                            "name": "Inside",
                            "column_names": ["ACount", "Count"],
                            "schema": ["Int", "Int"],
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
                        "c": "Article 0"
                    },
                    {
                        "t": "Text",
                        "c": "Author 0"
                    },
                    {
                        "t": "Int",
                        "c": 48
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
                        "c": "Article 0"
                    },
                    {
                        "t": "Text",
                        "c": "Author 0"
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
                        "c": "Article 1"
                    },
                    {
                        "t": "Text",
                        "c": "Author 1"
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
                        "c": "Article 1"
                    },
                    {
                        "t": "Text",
                        "c": "Author 1"
                    },
                    {
                        "t": "Int",
                        "c": 51
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
                        "c": "Article 1"
                    },
                    {
                        "t": "Text",
                        "c": "Author 1"
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

    assert_eq!(g_unit.leaf_counts(), vec![2, 3]);

    let mut leaf_op = g_unit.data.node_weight(NodeIndex::new(3)).unwrap().borrow_mut();

    match &*leaf_op {
        Leafor(leaf) => {
    
            let lri_row = (*leaf).mat_view.table.get(&DataType::Text("Article 0".to_string())).unwrap();
            let test_row = Row::new(vec![DataType::Text("Article 0".to_string()), DataType::Text("Author 0".to_string()), DataType::Int(2)]);
        
            assert_eq!(*lri_row, test_row); 

            let lri_row2 = (*leaf).mat_view.table.get(&DataType::Text("Article 1".to_string())).unwrap();
            let test_row2 = Row::new(vec![DataType::Text("Article 1".to_string()), DataType::Text("Author 1".to_string()), DataType::Int(1)]);
        
            assert_eq!(*lri_row2, test_row2);
        },
        _ => (),
    };

    let mut leaf_op_two = g_unit.data.node_weight(NodeIndex::new(4)).unwrap().borrow_mut();

    match &*leaf_op_two {
        Leafor(leaf) => {
    
            let lri_row = (*leaf).mat_view.table.get(&DataType::Int(48)).unwrap();
            let test_row = Row::new(vec![DataType::Int(48), DataType::Int(1)]);

            assert_eq!(*lri_row, test_row);

            let lri_row2 = (*leaf).mat_view.table.get(&DataType::Int(51)).unwrap();
            let test_row2 = Row::new(vec![DataType::Int(51), DataType::Int(1)]);

            assert_eq!(*lri_row2, test_row2);

            let lri_row3 = (*leaf).mat_view.table.get(&DataType::Int(49)).unwrap();
            let test_row3 = Row::new(vec![DataType::Int(49), DataType::Int(1)]);
        
            assert_eq!(*lri_row3, test_row3); 
        },
        _ => (),
    };

        
}

//#[wasm_bindgen_test]
fn innerjoin_unit_test() {
    //first has [ArticleName, Author]
    //second has [PublisherName, ArticleName]

    let unit_graph = r##"{
        "operators": [
                {
                    "t": "Rootor",
                    "c": {
                        "root_id": "first"
                    }
                },
                {
                    "t": "Rootor",
                    "c": {
                        "root_id": "second"
                    }
                },
                {
                    "t": "InnerJoinor",
                    "c": {
                        "parent_ids": [0, 1],
                        "join_cols": [0, 1]
                    }
                },
                {
                    "t": "Leafor",
                    "c": {
                        "mat_view": {
                            "name": "Outside",
                            "column_names": ["Author", "PublisherName", "ArticleName"],
                            "schema": ["Text", "Text", "Text"],
                            "key_index": 0
                        }
                    }
                }
            ],
        "edges": [{
            "parentindex": 0,
            "childindex": 2
        }, {
            "parentindex": 1,
            "childindex": 2
        },
        {
            "parentindex": 2,
            "childindex": 3
        }]
    }"##;  

    let g_unit = DataFlowGraph::new(unit_graph.to_owned());

    assert_eq!(g_unit.node_count(), 4);
    assert_eq!(g_unit.edge_count(), 3);

    let sin_row_ins_left = r##"{
        "typing": "Insertion",
        "batch": [
            {
                "data": [
                    {
                        "t": "Text",
                        "c": "Article 1"
                    },
                    {
                        "t": "Text",
                        "c": "Author 0"
                    }
                ]
            } 
        ]
    }"##;

    let sin_row_ins_right = r##"{
        "typing": "Insertion",
        "batch": [
            {
                "data": [
                    {
                        "t": "Text",
                        "c": "Publisher 1"
                    },
                    {
                        "t": "Text",
                        "c": "Article 0"
                    }
                ]
            } 
        ]
    }"##;

    let mult_row_ins_left = r##"{
        "typing": "Insertion",
        "batch": [
            {
                "data": [
                    {
                        "t": "Text",
                        "c": "Article 1"
                    },
                    {
                        "t": "Text",
                        "c": "Author 1"
                    }
                ]
            },
            {
                "data": [
                    {
                        "t": "Text",
                        "c": "Article 2"
                    },
                    {
                        "t": "Text",
                        "c": "Author 2"
                    }
                ]
            }
        ]
    }"##;

    let mult_row_ins_right = r##"{
        "typing": "Insertion",
        "batch": [
            {
                "data": [
                    {
                        "t": "Text",
                        "c": "Publisher 2"
                    },
                    {
                        "t": "Text",
                        "c": "Article 1"
                    }
                ]
            },
            {
                "data": [
                    {
                        "t": "Text",
                        "c": "Publisher 3"
                    },
                    {
                        "t": "Text",
                        "c": "Article 2"
                    }
                ]
            }
        ]
    }"##;

    let sin_row_del_left = r##"{
        "typing": "Deletion",
        "batch": [
            {
                "data": [
                    {
                        "t": "Text",
                        "c": "Article 1"
                    },
                    {
                        "t": "Text",
                        "c": "Author 0"
                    }
                ]
            } 
        ]
    }"##;

    let sin_row_del_right = r##"{
        "typing": "Deletion",
        "batch": [
            {
                "data": [
                    {
                        "t": "Text",
                        "c": "Publisher 3"
                    },
                    {
                        "t": "Text",
                        "c": "Article 2"
                    }
                ]
            } 
        ]
    }"##;


    g_unit.change_to_root_json("first".to_owned(), sin_row_ins_left.to_owned());
    g_unit.change_to_root_json("second".to_owned(), sin_row_ins_right.to_owned());
    g_unit.change_to_root_json("first".to_owned(), mult_row_ins_left.to_owned());
    g_unit.change_to_root_json("second".to_owned(), mult_row_ins_right.to_owned());
    g_unit.change_to_root_json("first".to_owned(), sin_row_del_left.to_owned());
    g_unit.change_to_root_json("second".to_owned(), sin_row_del_right.to_owned());

    assert_eq!(g_unit.leaf_counts(), vec![1]);

    let mut leaf_op = g_unit.data.node_weight(NodeIndex::new(3)).unwrap().borrow_mut();

    match &*leaf_op {
        Leafor(leaf) => {
            let lri_row = (*leaf).mat_view.table.get(&DataType::Text("Author 1".to_string())).unwrap();
            let test_row = Row::new(vec![DataType::Text("Author 1".to_string()), DataType::Text("Publisher 2".to_string()), DataType::Text("Article 1".to_string())]);
        
            assert_eq!(*lri_row, test_row); 
        },
        _ => (),
    };  
}

//pub struct TestGenerator {}

//impl TestGenerator {
    pub fn author_story_inserts() -> Vec<String> {
        let mut str_vec = Vec::new();
        let mut story_count = 1;

        let template_one = r##"{
            "typing": "Insertion",
            "batch": [
                {
                    "data": [
                        {
                            "t": "Int",
                            "c": "##;                     
        let template_two = r##"
                        },{
                            "t": "Int",
                            "c": "##;            
        let template_three = r##"
                        }
                    ]
                } 
            ]
        }"##;

        for n in 1..401 {
            for _z in 1..11 {
                let story_id = &story_count.to_string();
                let user_id = &n.to_string();  

                let change_json = [template_one, &user_id, template_two, &story_id, template_three].concat();
                str_vec.push(change_json);
                story_count = story_count + 1;
            }
        }

        str_vec
    }

    pub fn author_story_deletes() -> Vec<String> {
        let mut str_vec = Vec::new();
        let mut story_count = 2001;

        let template_one = r##"{
            "typing": "Deletion",
            "batch": [
                {
                    "data": [
                        {
                            "t": "Int",
                            "c": "##;                     
        let template_two = r##"
                        },{
                            "t": "Int",
                            "c": "##;            
        let template_three = r##"
                        }
                    ]
                } 
            ]
        }"##;

        for n in 201..401 {
            for _z in 1..11 {
                let story_id = &story_count.to_string();
                let user_id = &n.to_string();  

                let change_json = [template_one, &user_id, template_two, &story_id, template_three].concat();
                str_vec.push(change_json);
                story_count = story_count + 1;
            }
        }

        str_vec
    }

    pub fn story_voter_inserts() -> Vec<String> {
        let mut str_vec = Vec::new();

        let template_one = r##"{
            "typing": "Insertion",
            "batch": [
                {
                    "data": [
                        {
                            "t": "Int",
                            "c": "##;  
        let template_two = r##"
                    },{
                        "t": "Int",
                        "c": "##;
        let template_three = r##"
                        }
                    ]
                } 
            ]
        }"##;


        for n in 1..2001 {
            for z in 1..6 {         
                let story_id = &n.to_string();
                let user_id = z.to_string();                   

                let change_json = [template_one, &story_id, template_two, &user_id, template_three].concat();
                str_vec.push(change_json);
            }
        }

        str_vec
    }

    pub fn story_voter_deletes() -> Vec<String> {
        let mut str_vec = Vec::new();

        let template_one = r##"{
            "typing": "Deletion",
            "batch": [
                {
                    "data": [
                        {
                            "t": "Int",
                            "c": "##;  
        let template_two = r##"
                    },{
                        "t": "Int",
                        "c": "##;
        let template_three = r##"
                        }
                    ]
                } 
            ]
        }"##;


        for n in 1..2001 {     
            let author_id = &n.to_string();
            let user_id = "1";                   

            let change_json = [template_one, &author_id, template_two, &user_id, template_three].concat();
            str_vec.push(change_json);
        }

        for n in 1..2001 {     
            let author_id = &n.to_string();
            let user_id = "2";                   

            let change_json = [template_one, &author_id, template_two, &user_id, template_three].concat();
            str_vec.push(change_json);
        }

        str_vec
    }

    pub fn user_email_inserts() -> Vec<String> {
        let mut str_vec = Vec::new();
        let mut story_count = 1;

        let template_one = r##"{
            "typing": "Insertion",
            "batch": [
                {
                    "data": [
                        {
                            "t": "Int",
                            "c": "##;  
        let template_two = r##"
                    },{
                        "t": "Text",
                        "c": "##;
        let template_three = r##"
                        }
                    ]
                } 
            ]
        }"##;


        for n in 1..4001 {
            for z in 1..6 {         
                let user_id = &n.to_string();
                let email = z.to_string();                   

                let change_json = [template_one, &user_id, template_two, &email, template_three].concat();
                str_vec.push(change_json);
                story_count = story_count + 1;
            }
        }

        str_vec
    }

    pub fn read_keys() -> Vec<String> {
        let mut keys = Vec::new();

        let template_one = r##"{
            "t": "Int",
            "c": "##;  
        let template_two = r##"
        }"##;


        for n in 1.. 501 {
            for k in 1..2001 {  
                let insert = k.to_string();
        
                let key = [template_one, &insert, template_two].concat();
                keys.push(key);
            }
        }

        keys
    }

    pub fn read_keys_latency() -> Vec<String> {
        let mut keys = Vec::new();

        let template_one = r##"{
            "t": "Int",
            "c": "##;  
        let template_two = r##"
        }"##;


        for n in 1.. 11 {
            for k in 1..2001 {  
                let insert = k.to_string();
        
                let key = [template_one, &insert, template_two].concat();
                keys.push(key);
            }
        }

        keys
    }

    pub fn write_sv_latency() -> Vec<String> {
        let mut str_vec = Vec::new();

        let template_one = r##"{
            "typing": "Insertion",
            "batch": [
                {
                    "data": [
                        {
                            "t": "Int",
                            "c": "##;  
        let template_two = r##"
                    },{
                        "t": "Int",
                        "c": "##;
        let template_three = r##"
                        }
                    ]
                } 
            ]
        }"##;


        for n in 1..2001 {
            for z in 6..9 {         
                let author_id = &n.to_string();
                let user_id = z.to_string();                   

                let change_json = [template_one, &author_id, template_two, &user_id, template_three].concat();
                str_vec.push(change_json);
            }
        }

        
        let template_one_d = r##"{
            "typing": "Deletion",
            "batch": [
                {
                    "data": [
                        {
                            "t": "Int",
                            "c": "##;  
        let template_two_d = r##"
                    },{
                        "t": "Int",
                        "c": "##;
        let template_three_d = r##"
                        }
                    ]
                } 
            ]
        }"##;


        for n in 1..2001 {     
            let author_id_d = &n.to_string();
            let user_id_d = "6";                   

            let change_json = [template_one_d, &author_id_d, template_two_d, &user_id_d, template_three_d].concat();
            str_vec.push(change_json);
        }

        for n in 1..2001 {     
            let author_id_d = &n.to_string();
            let user_id_d = "7";                   

            let change_json = [template_one_d, &author_id_d, template_two, &user_id_d, template_three_d].concat();
            str_vec.push(change_json);
        }

        str_vec
    }

    pub fn write_as_latency() -> Vec<String> {
        let mut str_vec = Vec::new();
        let mut story_count = 4001;

        let template_one = r##"{
            "typing": "Insertion",
            "batch": [
                {
                    "data": [
                        {
                            "t": "Int",
                            "c": "##;                     
        let template_two = r##"
                        },{
                            "t": "Int",
                            "c": "##;            
        let template_three = r##"
                        }
                    ]
                } 
            ]
        }"##;

        for n in 1..501 {
            for _z in 1..11 {
                let story_id = &story_count.to_string();
                let user_id = &n.to_string();  

                let change_json = [template_one, &user_id, template_two, &story_id, template_three].concat();
                str_vec.push(change_json);
                story_count = story_count + 1;
            }
        }

        let mut str_vec = Vec::new();
        let mut story_count = 4001;

        let template_one_d = r##"{
            "typing": "Deletion",
            "batch": [
                {
                    "data": [
                        {
                            "t": "Int",
                            "c": "##;                     
        let template_two_d = r##"
                        },{
                            "t": "Int",
                            "c": "##;            
        let template_three_d = r##"
                        }
                    ]
                } 
            ]
        }"##;

        for n in 1..501 {
            for _z in 1..11 {
                let story_id_d = &story_count.to_string();
                let user_id_d = &n.to_string();  

                let change_json = [template_one_d, &user_id_d, template_two_d, &story_id_d, template_three_d].concat();
                str_vec.push(change_json);
                story_count = story_count + 1;
            }
        }



        str_vec
    }

//}

//#[wasm_bindgen_test]
#[wasm_bindgen_test]
//114, 95, 98, 91, 98 => 99.2
//Throughput => 201,612
fn write_throughput_votecounts() {
    let graph_json = r##"{
        "operators": [
                {
                    "t": "Rootor",
                    "c": {
                        "root_id": "AuthorStory"
                    }
                },
                {
                    "t": "Rootor",
                    "c": {
                        "root_id": "StoryVoter"
                    }
                },
                {
                    "t": "Aggregator",
                    "c": {
                        "group_by_col": [0]
                    }
                },
                {
                    "t": "InnerJoinor",
                    "c": {
                        "parent_ids": [0, 1],
                        "join_cols": [1, 0]
                    }
                },
                {
                    "t": "Leafor",
                    "c": {
                        "mat_view": {
                            "name": "Users and VoteCounts",
                            "column_names": ["AuthorUserID", "StoryID", "StoryVoteCount"],
                            "schema": ["Int", "Int", "Int"],
                            "key_index": 1
                        }
                    }
                }
            ],
        "edges": [{
            "parentindex": 0,
            "childindex": 3
        }, {
            "parentindex": 1,
            "childindex": 2
        },
        {
            "parentindex": 2,
            "childindex": 3
        },
        {
            "parentindex": 3,
            "childindex": 4
        }]
    }"##;

    let graph = DataFlowGraph::new(graph_json.to_owned());

    assert_eq!(graph.node_count(), 5);
    assert_eq!(graph.edge_count(), 4);

    let author_story_inserts = author_story_inserts();
    let story_voter_inserts = story_voter_inserts();
    let author_story_deletes = author_story_deletes();
    let story_voter_deletes = story_voter_deletes();

    let now = Instant::now();

    for change in author_story_inserts.iter() {
        graph.change_to_root_json("AuthorStory".to_owned(), change.to_string());
    }

    for change in story_voter_inserts.iter() {
        graph.change_to_root_json("StoryVoter".to_owned(), change.to_string());
    }

    for change in author_story_deletes.iter() {
        graph.change_to_root_json("AuthorStory".to_owned(), change.to_string());
    }

    for change in story_voter_deletes.iter() {
        graph.change_to_root_json("StoryVoter".to_owned(), change.to_string());
    }

    let elapsed = now.elapsed();

    console::log_1(&format!("Elapsed: {:?}", elapsed).into());

    assert_eq!(graph.node_count(), 4);
}

#[wasm_bindgen_test]
//1.429, 2.104, 2.019, 1.296, 1,316 => 1.638
//Throughput => 610,501
fn read_throughput_votecounts() {
    let graph_json = r##"{
        "operators": [
                {
                    "t": "Rootor",
                    "c": {
                        "root_id": "AuthorStory"
                    }
                },
                {
                    "t": "Rootor",
                    "c": {
                        "root_id": "StoryVoter"
                    }
                },
                {
                    "t": "Aggregator",
                    "c": {
                        "group_by_col": [0]
                    }
                },
                {
                    "t": "InnerJoinor",
                    "c": {
                        "parent_ids": [0, 1],
                        "join_cols": [1, 0]
                    }
                },
                {
                    "t": "Leafor",
                    "c": {
                        "mat_view": {
                            "name": "Users and VoteCounts",
                            "column_names": ["AuthorUserID", "StoryID", "StoryVoteCount"],
                            "schema": ["Int", "Int", "Int"],
                            "key_index": 1
                        }
                    }
                }
            ],
        "edges": [{
            "parentindex": 0,
            "childindex": 3
        }, {
            "parentindex": 1,
            "childindex": 2
        },
        {
            "parentindex": 2,
            "childindex": 3
        },
        {
            "parentindex": 3,
            "childindex": 4
        }]
    }"##;

    let graph = DataFlowGraph::new(graph_json.to_owned());

    assert_eq!(graph.node_count(), 5);
    assert_eq!(graph.edge_count(), 4);

    let author_story_inserts = author_story_inserts();
    let story_voter_inserts = story_voter_inserts();
    let keys = read_keys();

    for change in author_story_inserts.iter() {
        graph.change_to_root_json("AuthorStory".to_owned(), change.to_string());
    }

    for change in story_voter_inserts.iter() {
        graph.change_to_root_json("StoryVoter".to_owned(), change.to_string());
    }

    let now2 = Instant::now();

    for change in keys.iter() {
        graph.read(4, change.to_string());
    }

    let elapsed2 = now2.elapsed();

    console::log_1(&format!("Elapsed2: {:?}", elapsed2).into());

    assert_eq!(graph.node_count(), 4);
}

#[wasm_bindgen_test]
//95, 161, 148, 103, 103 => 122
//Throughput => 327,869
//Calculated => 99 + 32 = 131
//Calculated Throughput => 303,121
fn latency_test() {
    let graph_json = r##"{
        "operators": [
                {
                    "t": "Rootor",
                    "c": {
                        "root_id": "AuthorStory"
                    }
                },
                {
                    "t": "Rootor",
                    "c": {
                        "root_id": "StoryVoter"
                    }
                },
                {
                    "t": "Aggregator",
                    "c": {
                        "group_by_col": [0]
                    }
                },
                {
                    "t": "InnerJoinor",
                    "c": {
                        "parent_ids": [0, 1],
                        "join_cols": [1, 0]
                    }
                },
                {
                    "t": "Leafor",
                    "c": {
                        "mat_view": {
                            "name": "Users and VoteCounts",
                            "column_names": ["AuthorUserID", "StoryID", "StoryVoteCount"],
                            "schema": ["Int", "Int", "Int"],
                            "key_index": 1
                        }
                    }
                }
            ],
        "edges": [{
            "parentindex": 0,
            "childindex": 3
        }, {
            "parentindex": 1,
            "childindex": 2
        },
        {
            "parentindex": 2,
            "childindex": 3
        },
        {
            "parentindex": 3,
            "childindex": 4
        }]
    }"##;

    let graph = DataFlowGraph::new(graph_json.to_owned());

    assert_eq!(graph.node_count(), 5);
    assert_eq!(graph.edge_count(), 4);

    let author_story_inserts = author_story_inserts();
    let story_voter_inserts = story_voter_inserts();
    let keys_latency = read_keys_latency();
    let as_latency = write_as_latency();
    let sv_latency = write_sv_latency();

    for change in author_story_inserts.iter() {
        graph.change_to_root_json("AuthorStory".to_owned(), change.to_string());
    }

    for change in story_voter_inserts.iter() {
        graph.change_to_root_json("StoryVoter".to_owned(), change.to_string());
    }

    let now3 = Instant::now();

    for change in keys_latency.iter() {
        graph.read(4, change.to_string());
    }

    for change in as_latency.iter() {
        graph.change_to_root_json("AuthorStory".to_owned(), change.to_string());
    }

    for change in sv_latency.iter() {
        graph.change_to_root_json("StoryVoter".to_owned(), change.to_string());
    }

    let elapsed3 = now3.elapsed();

    console::log_1(&format!("Elapsed3: {:?}", elapsed3).into());

    assert_eq!(graph.node_count(), 4);
}
