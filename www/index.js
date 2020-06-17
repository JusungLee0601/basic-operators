import { View, Row } from "noria-clientside";

const view = View.new("Dummy");

//const inserts = document.getElementById("inserts");

const refreshEntries = () => {
    console.log("Printing");
    console.log(view.render());
    document.getElementById("inserts").innerHTML = view.render();
}

const addEntry = () => {
    var articleString = document.getElementById("article").value;
    var countString = document.getElementById("count").value;
    console.log("send");

    var row = Row.new(articleString, countString);

    view.insert(row);
}

const select = () => {
    var searchCol = document.getElementById("searchcol").value;
    var searchData = document.getElementById("searchdata").value;
    console.log("select");

    view.selection(searchCol, searchData)
}


document.getElementById("refresh").addEventListener("click", event => {refreshEntries();});
document.getElementById("send").addEventListener("click", event => {addEntry();});
document.getElementById("select").addEventListener("click", event => {select();});

