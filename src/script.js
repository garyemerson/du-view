function handleKey(event) {
    var arrowExpanded = "url(\"data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%27http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%27%20width%3D%2715%27%20height%3D%2720%27%3E%3Cpolygon%20points%3D%270%2C4%2012%2C4%206%2C14%27%20fill%3D%27%23b8b4b4%27%20%2F%3E%3C%2Fsvg%3E\") no-repeat";
    var arrowCollapsed = "url(\"data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%27http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%27%20width%3D%2715%27%20height%3D%2720%27%3E%3Cpolygon%20points%3D%270%2C4%2010%2C10%200%2C16%27%20fill%3D%27%23b8b4b4%27%20%2F%3E%3C%2Fsvg%3E\") no-repeat";

    console.log("got code ", event.code);
    switch (event.code) {
        case "ArrowUp":
            console.log("go Up with selectedNode ", selectedNode.id);
            var next = getNextUpNode(selectedNode);
            if (next !== null && next.id !== selectedNode.id) {
                document.getElementById("item_row" + selectedNode.id).style.background = "white";
                document.getElementById("item_row" + next.id).style.background = "lightblue";
                selectedNode = next;
            }
            event.preventDefault();
            break;

        case "ArrowDown":
            console.log("go Down with selectedNode ", selectedNode.id);
            var next = getNextDownNode(selectedNode);
            if (next !== null && next.id !== selectedNode.id) {
                document.getElementById("item_row" + selectedNode.id).style.background = "white";
                document.getElementById("item_row" + next.id).style.background = "lightblue";
                selectedNode = next;
            }
            event.preventDefault();
            break;

        case "ArrowLeft":
            console.log("go Left with selectedNode ", selectedNode.id);
            // TODO: if already collapsed, goto to parent
            if (!selectedNode.expanded && selectedNode.parent !== null) {
                document.getElementById("item_row" + selectedNode.id).style.background = "white";
                document.getElementById("item_row" + selectedNode.parent.id).style.background = "lightblue";
                selectedNode = selectedNode.parent;
            } else {
                selectedNode.expanded = false;
                document.getElementById("children" + selectedNode.id).style.display = "none";
                document.getElementById("arrow" + selectedNode.id).style.background = arrowCollapsed;
            }
            event.preventDefault();
            break;

        case "ArrowRight":
            console.log("go Right with selectedNode ", selectedNode.id);
            if (selectedNode.children.length > 0) {
                selectedNode.expanded = true;
                document.getElementById("children" + selectedNode.id).style.display = "initial";
                document.getElementById("arrow" + selectedNode.id).style.background = arrowExpanded;
            }
            event.preventDefault();
            break;

        case "Home":
            console.log("go home with selectedNode ", selectedNode.id);
            var rt = getRoot(selectedNode);
            document.getElementById("item_row" + selectedNode.id).style.background = "white";
            document.getElementById("item_row" + rt.id).style.background = "lightblue";
            selectedNode = rt;
            // event.preventDefault();
            break;

        case "End":
            console.log("go end with selectedNode ", selectedNode.id);
            var last = getLastNode(selectedNode);
            document.getElementById("item_row" + selectedNode.id).style.background = "white";
            document.getElementById("item_row" + last.id).style.background = "lightblue";
            selectedNode = last;
            // event.preventDefault();
            break;
    }
}
function getLastNode(node) {
    var curr = node;
    while (curr.children.length > 0 && curr.expanded) {
        curr = curr.children[curr.children.length - 1];
    }
    return curr;
}
function getRoot(node) {
    var curr = node;
    while (curr.parent !== null) {
        curr = curr.parent;
    }
    return curr;
}
function getNextUpNode(node) {
    if (node.parent === null) {
        return null;
    } else if (node.childIndex === 0) {
        return node.parent;
    } else {
        var curr = node.parent.children[node.childIndex - 1];
        while (curr.children.length > 0 && curr.expanded) {
            curr = curr.children[curr.children.length - 1];
        }
        return curr;
    }
}
function getNextDownNode(node) {
    if (node.children !== null && node.expanded) {
        return node.children[0];
    } else if (node.parent !== null && node.childIndex !== node.parent.children.length - 1) {
        return node.parent.children[node.childIndex + 1];
    } else {
        var curr = node;
        while (curr.parent !== null) {
            if (curr.childIndex !== curr.parent.children.length - 1) {
                return curr.parent.children[curr.childIndex + 1];
            }
            curr = curr.parent;
        }
        return null;
    }
}
function fillInParentProperties(node, parent) {
    node.parent = parent;
    for (var i = 0; i < node.children.length; i++) {
      fillInParentProperties(node.children[i], node);
    }
}

console.log("filling in parents...")
fillInParentProperties(treeRoot, null);
console.log("done")
console.log(treeRoot);

var selectedNode = treeRoot;
document.getElementById("item_row0").style.background = "lightblue";
