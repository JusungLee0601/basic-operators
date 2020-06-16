import { View } from "basic-operators";

const view = View.new();

//const inserts = document.getElementById("inserts");

const refreshEntries = () => {
    console.log("Printing");
    console.log(view.render());
    document.getElementById("inserts").innerHTML = view.render();
}

const addEntry = () => {
    var updateString = document.getElementById("data").value;
    console.log("send");
    view.update(updateString);
}

document.getElementById("refresh").addEventListener("click", event => {refreshEntries();});
document.getElementById("send").addEventListener("click", event => {addEntry();});

