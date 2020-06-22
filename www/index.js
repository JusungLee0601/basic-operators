import { View, Row, SchemaType, DataType } from "noria-clientside";

var schema = [SchemaType.Text, SchemaType.Int];
var columns = ["Article Name", "Count"];
const view = View.newJS("Dummy", 0, columns, schema);

//note assumptsions  
//const inserts = document.getElementById("inserts");

const refreshEntries = () => {
    console.log("Printing");
    console.log(view.render());
    document.getElementById("inserts").innerHTML = view.render();
}

const addEntry = () => {
    var articleString = document.getElementById("article").value;
    var count = parseInt(document.getElementById("count").value);

    var row = [articleString, count];

    view.insert(row);
}

// const select = () => {
//     var searchCol = document.getElementById("searchcol").value;
//     var searchData = document.getElementById("searchdata").value;
//     console.log("select");

//     view.selection(searchCol, searchData)
// }


document.getElementById("refresh").addEventListener("click", event => {refreshEntries();});
document.getElementById("send").addEventListener("click", event => {addEntry();});
//document.getElementById("select").addEventListener("click", event => {select();});

