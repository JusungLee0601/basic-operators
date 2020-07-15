import { View, Row, SchemaType, DataType, DataFlowGraph, Operation} from "noria-clientside";

// var schema = [SchemaType.Text, SchemaType.Int];
// var columns = ["Article Name", "Count"];
// const parent_view = View.newJS("Dummy", 0, columns, schema);

var dummygraph = {
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
            "t": "Projector",
            "c": {
                "columns": [0]
            }
        },
        {
            "t": "Aggregator",
            "c": {
                "group_by_col": [1]
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
        },
        {
            "t": "Leafor",
            "c": {
                "mat_view": {
                    "name": "second",
                    "column_names": ["Article"],
                    "schema": ["Text"],
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
};

const graph = DataFlowGraph.new(JSON.stringify(dummygraph));

const refreshEntries = () => {
    console.log("Printing");
    document.getElementById("inserts").innerHTML = graph.render();
}

const addEntry = () => {
    var articleString = document.getElementById("article").value;
    var count = parseInt(document.getElementById("count").value);

    var row = [articleString, count];
    console.log("send");

    graph.change_to_root("first", row);
}

document.getElementById("refresh").addEventListener("click", event => {refreshEntries();});
document.getElementById("send").addEventListener("click", event => {addEntry();});

