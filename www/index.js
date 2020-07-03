import { View, Row, SchemaType, DataType, DataFlowGraph, Operation} from "noria-clientside";

// var schema = [SchemaType.Text, SchemaType.Int];
// var columns = ["Article Name", "Count"];
// const parent_view = View.newJS("Dummy", 0, columns, schema);

var dummygraph = {
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
            "t": "Selection",
            "c": {
                "col_ind": 0,
                "condition": {
                    "t": "Text",
                    "c": "dummy"
                } 
            }
        }
    }, {
        "parentindex": 1,
        "childindex": 2,
        "operation": {
            "t": "Projection",
            "c": {
                "columns": [0]
            }
        }
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

    graph.process_insert("first", row);
}

// const select = () => {
//     var searchCol = parseInt(document.getElementById("searchcol").value);
//     var searchData = document.getElementById("searchdata").value;
//     console.log("select");

//     var selector = Selection.newJS(searchCol, searchData);
//     var child_view = selector.select("child", parent_view);
//     console.log("c");
//     console.log(child_view.render());
//     console.log("parent");
//     console.log(parent_view.render());
//     console.log("s");
//     console.log(selector);
//     graph.extend(parent_view, child_view, selector);
// }


document.getElementById("refresh").addEventListener("click", event => {refreshEntries();});
document.getElementById("send").addEventListener("click", event => {addEntry();});
document.getElementById("select").addEventListener("click", event => {select();});

