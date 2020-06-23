import { View, Row, SchemaType, DataType, DataFlowGraph, Operator, Selection } from "noria-clientside";

var schema = [SchemaType.Text, SchemaType.Int];
var columns = ["Article Name", "Count"];
const parent_view = View.newJS("Dummy", 0, columns, schema);
const graph = DataFlowGraph.new()

//const inserts = document.getElementById("inserts");

const refreshEntries = () => {
    console.log("Printing");
    document.getElementById("inserts").innerHTML = graph.render();
}

const addEntry = () => {
    var articleString = document.getElementById("article").value;
    var count = parseInt(document.getElementById("count").value);

    var row = [articleString, count];
    console.log("send");

    parent_view.insert(row);
    console.log(parent_view.render());
}

const select = () => {
    var searchCol = parseInt(document.getElementById("searchcol").value);
    var searchData = document.getElementById("searchdata").value;
    console.log("select");

    var selector = Selection.newJS(searchCol, searchData);
    var child_view = selector.select("child", parent_view);
    console.log("c");
    console.log(child_view.render());
    console.log("parent");
    console.log(parent_view.render());
    console.log("s");
    console.log(selector);
    graph.extend(parent_view, child_view, selector);
}


document.getElementById("refresh").addEventListener("click", event => {refreshEntries();});
document.getElementById("send").addEventListener("click", event => {addEntry();});
document.getElementById("select").addEventListener("click", event => {select();});

