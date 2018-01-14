function handleKey(event) {
    var arrowExpanded = "url(\"data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%27http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%27%20width%3D%2715%27%20height%3D%2720%27%3E%3Cpolygon%20points%3D%270%2C4%2012%2C4%206%2C14%27%20fill%3D%27%23b8b4b4%27%20%2F%3E%3C%2Fsvg%3E\") no-repeat";
    var arrowCollapsed = "url(\"data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%27http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%27%20width%3D%2715%27%20height%3D%2720%27%3E%3Cpolygon%20points%3D%270%2C4%2010%2C10%200%2C16%27%20fill%3D%27%23b8b4b4%27%20%2F%3E%3C%2Fsvg%3E\") no-repeat";

    // console.log("got code ", event.code);
    switch (event.code) {
        case "ArrowUp":
            // console.log("go Up with selectedNode ", selectedNode.id);
            var next = getNextUpNode(selectedNode);
            if (next !== null && next.id !== selectedNode.id) {
                document.getElementById("item_row" + selectedNode.id).style.background = "white";
                document.getElementById("item_row" + next.id).style.background = "lightblue";
                scrollIfNeeded(next);
                selectedNode = next;
            }
            event.preventDefault();
            break;

        case "ArrowDown":
            // console.log("go Down with selectedNode ", selectedNode.id);
            var next = getNextDownNode(selectedNode);
            if (next !== null && next.id !== selectedNode.id) {
                document.getElementById("item_row" + selectedNode.id).style.background = "white";
                document.getElementById("item_row" + next.id).style.background = "lightblue";
                scrollIfNeeded(next);
                selectedNode = next;
            }
            event.preventDefault();
            break;

        case "ArrowLeft":
            // console.log("go Left with selectedNode ", selectedNode.id);
            if (!selectedNode.x && selectedNode.p !== null) {
                document.getElementById("item_row" + selectedNode.id).style.background = "white";
                document.getElementById("item_row" + selectedNode.p.id).style.background = "lightblue";
                scrollIfNeeded(selectedNode.p);
                selectedNode = selectedNode.p;
            } else {
                selectedNode.x = false;
                document.getElementById("children" + selectedNode.id).style.display = "none";
                document.getElementById("arrow" + selectedNode.id).style.background = arrowCollapsed;
            }
            event.preventDefault();
            break;

        case "ArrowRight":
            // console.log("go Right with selectedNode ", selectedNode.id);
            if (selectedNode.cc.length > 0) {
                selectedNode.x = true;
                document.getElementById("children" + selectedNode.id).style.display = "initial";
                document.getElementById("arrow" + selectedNode.id).style.background = arrowExpanded;
            }
            event.preventDefault();
            break;

        case "Home":
            // console.log("go home with selectedNode ", selectedNode.id);
            var rt = getRoot(selectedNode);
            document.getElementById("item_row" + selectedNode.id).style.background = "white";
            document.getElementById("item_row" + rt.id).style.background = "lightblue";
            scrollIfNeeded(rt);
            selectedNode = rt;
            event.preventDefault();
            break;

        case "End":
            // console.log("go end with selectedNode ", selectedNode.id);
            var rt = getRoot(selectedNode);
            var last = getLastNode(rt);
            document.getElementById("item_row" + selectedNode.id).style.background = "white";
            document.getElementById("item_row" + last.id).style.background = "lightblue";
            scrollIfNeeded(last);
            selectedNode = last;
            event.preventDefault();
            break;
    }
}
function scrollIfNeeded(node) {
    // console.log("scrollTop is ", document.documentElement.scrollTop);
    // console.log("offsetTop is ", document.getElementById("item_row" + node.id).offsetTop);
    var docBottom = Math.max(document.body.scrollTop, document.documentElement.scrollTop) + window.innerHeight;
    var elem = document.getElementById("item_row" + node.id);
    var elemBottom = elem.offsetTop + elem.offsetHeight;
    // console.log("docBottom " + docBottom);
    // console.log("elemBottom " + elemBottom);
    if (document.documentElement.scrollTop > document.getElementById("item_row" + node.id).offsetTop ||
        document.body.scrollTop > document.getElementById("item_row" + node.id).offsetTop)
    {
        document.getElementById("item_row" + node.id).scrollIntoView(true);
        // console.log("top is cut off");
    } else if (docBottom < elemBottom) {
        document.getElementById("item_row" + node.id).scrollIntoView(false);
        // console.log("bottom is cut off");
    }
}
function getLastNode(node) {
    var curr = node;
    while (curr.cc.length > 0 && curr.x) {
        curr = curr.cc[curr.cc.length - 1];
    }
    return curr;
}
function getRoot(node) {
    var curr = node;
    while (curr.p !== null) {
        curr = curr.p;
    }
    return curr;
}
function getNextUpNode(node) {
    if (node.p === null) {
        return null;
    } else if (node.ci === 0) {
        return node.p;
    } else {
        var curr = node.p.cc[node.ci - 1];
        while (curr.cc.length > 0 && curr.x) {
            curr = curr.cc[curr.cc.length - 1];
        }
        return curr;
    }
}
function getNextDownNode(node) {
    if (node.cc !== null && node.x) {
        return node.cc[0];
    } else if (node.p !== null && node.ci !== node.p.cc.length - 1) {
        return node.p.cc[node.ci + 1];
    } else {
        var curr = node;
        while (curr.p !== null) {
            if (curr.ci !== curr.p.cc.length - 1) {
                return curr.p.cc[curr.ci + 1];
            }
            curr = curr.p;
        }
        return null;
    }
}
function fillInParentProperties(node, parent) {
    node.p = parent;
    for (var i = 0; i < node.cc.length; i++) {
      fillInParentProperties(node.cc[i], node);
    }
}

console.log("filling in parents...")
var t0 = performance.now();
fillInParentProperties(treeRoot, null);
var t1 = performance.now();
console.log("Done. Time elapsed: " + (t1 - t0) + " milliseconds.");
console.log(treeRoot);

var selectedNode = treeRoot;
document.getElementById("item_row0").style.background = "lightblue";
