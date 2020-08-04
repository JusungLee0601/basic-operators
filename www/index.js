import { View, Row, SchemaType, DataType, DataFlowGraph, Operation} from "noria-clientside";

let socket = new WebSocket("ws://localhost:3012/poopoo");

var global_graph_initial;
var graph;

socket.onopen = function(e) {
    alert("[open] Connection established");
    alert("Sending to server");
    //socket.send("John");
};

socket.onmessage = function(event) {
    alert(`[message] Data received from server: ${event.data}`);
    console.log("once");
    graph = DataFlowGraph.new(event.data);
};

socket.onclose = function(event) {
    if (event.wasClean) {
        alert(`[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`);
    } else {
        // e.g. server process killed or network down
        // event.code is usually 1006 in this case
        alert('[close] Connection died');
    }
};

socket.onerror = function(error) {
    alert(`[error] ${error.message}`);
};

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

