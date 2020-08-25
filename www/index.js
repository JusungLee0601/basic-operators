import { View, Row, SchemaType, DataType, DataFlowGraph, Operation} from "noria-clientside";

let socketreadleft = new WebSocket("ws://localhost:3012/dummytest");
let counterleft = 0; 

var global_graph_initial;
var graph;

const socketrl = () => {
    socketreadleft.onopen = function(e) {
        alert("[open] Connection established for left write");
        alert("Sending to server");
        //socket.send("John");
    };

    socketreadleft.onmessage = function(event) {
        //alert(`[message] Data received from left server: ${event.data}`);

        if (counterleft == 0) {
            graph = DataFlowGraph.new(event.data);
            counterleft++;
        } else {
            counterleft++;
            graph.change_to_root_sc(event.data);
        }
    };

    socketreadleft.onclose = function(event) {
        if (event.wasClean) {
            alert(`[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`);
        } else {
            // e.g. server process killed or network down
            // event.code is usually 1006 in this case
            alert('[close] Connection died');
        }
    };

    socketreadleft.onerror = function(error) {
        alert(`[error] ${error.message}`);
    };

    console.log("reading finished left");
};

socketrl();

let socketwrite = new WebSocket("ws://localhost:3012/dummytestread");

const socketw = () => {
    console.log("writing");
    socketwrite.onopen = function(e) {
        alert("[open] Connection established for write socket");
    };

    socketwrite.onmessage = function(event) {
        alert(`[message] Data received from read socket: ${event.data}`);
    };

    socketwrite.onclose = function(event) {
        if (event.wasClean) {
            alert(`[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`);
        } else {
            // e.g. server process killed or network down
            // event.code is usually 1006 in this case
            alert('[close] Connection died');
        }
    };

    socketwrite.onerror = function(error) {
        alert(`[error] ${error.message}`);
    };

    console.log("writing finished");
};

socketw();

const addEntry = () => {
    var articleString = document.getElementById("article").value;
    var count = parseInt(document.getElementById("count").value);

    var stories = {
        root_id: "OnlyServer", 
        changes: [
            {
                typing: "Insertion",
                batch: [
                    {
                        data: [
                            {
                                t: "Text"
                            },
                            {
                                t: "Int"
                            }
                        ]
                    }
                ]
            }
        ]
    };

    stories.changes[0].batch[0].data[0].c = articleString;
    stories.changes[0].batch[0].data[1].c = count;

    console.log(stories);

    socketwrite.send(JSON.stringify(stories));

    console.log("sent");
}

const refreshEntries = () => {
    console.log("Printing");
    document.getElementById("inserts").innerHTML = graph.render();
    console.log(graph.leaf_counts()[0]);
}

document.getElementById("refresh").addEventListener("click", event => {refreshEntries();});
document.getElementById("test").addEventListener("click", event => {test();});
document.getElementById("send").addEventListener("click", event => {addEntry();});

