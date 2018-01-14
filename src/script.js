function handleKey(event) {
    // console.log("received key event ", event)
    switch (event.code) {
        case "ArrowUp":
            console.log("go Up");
            if (curr !== 0) {
                // document.getElementById("children" + curr).style.display = "none";
                document.getElementById("item_row" + curr).style.background = "white";
                document.getElementById("arrow" + curr).style.background =
                    "url(\"data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='15' height='20'><polygon points='0,4 10,10 0,16' fill='#b8b4b4' /></svg>\") no-repeat";

                curr -= 1;
                // document.getElementById("children" + curr).style.display = "initial";
                document.getElementById("item_row" + curr).style.background = "lightblue";
                document.getElementById("arrow" + curr).style.background =
                    "url(\"data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='15' height='20'><polygon points='0,4 12,4 6,14' fill='#b8b4b4' /></svg>\") no-repeat";
            }
            break;

        case "ArrowDown":
            console.log("go Down");
            if (curr !== 58) {
                // document.getElementById("children" + curr).style.display = "none";
                document.getElementById("item_row" + curr).style.background = "white";
                document.getElementById("arrow" + curr).style.background =
                    "url(\"data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='15' height='20'><polygon points='0,4 10,10 0,16' fill='#b8b4b4' /></svg>\") no-repeat";

                curr += 1;
                // document.getElementById("children" + curr).style.display = "initial";
                document.getElementById("item_row" + curr).style.background = "lightblue";
                document.getElementById("arrow" + curr).style.background =
                    "url(\"data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='15' height='20'><polygon points='0,4 12,4 6,14' fill='#b8b4b4' /></svg>\") no-repeat";
            }

            break;

        case "ArrowLeft":
            console.log("go Left");
            break;

        case "ArrowRight":
            console.log("go Right");
            break;

    }
}
function fillInParentProperties(tree, parent) {
    tree.parent = parent;
    for (var i = 0; i < tree.children.length; i++) {
      fillInParentProperties(tree.children[i], tree);
    }
}

fillInParentProperties(treeRoot, null);
console.log(treeRoot);
document.getElementById("children0").style.display = "initial";
document.getElementById("item_row0").style.background = "lightblue";
document.getElementById("arrow0").style.background =
    "url(\"data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='15' height='20'><polygon points='0,4 12,4 6,14' fill='#b8b4b4' /></svg>\") no-repeat";
var curr = treeRoot;
