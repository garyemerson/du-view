extern crate regex;

use std::path::Path;
use std::collections::HashMap;
use regex::Regex;
use std::process::Command;
use std::str;

fn main() {
    match run_and_get_output() {
        Ok(output) => {
            // println!("{}", output);
            let mut html = "
<!doctype html>
<html lang=\"en-US\">
<head>
    <title>du view</title>
    <style>
        .item_row {
            width: 100%;
        }
        .item {
            padding-left: 20px;
            margin: 5px;
            border: 1px solid #444;
        }
        ul {
            padding: 0;
            list-style: none;
            display: none;
        }
        .size {
            font-family: monospace;
            color: #7c7c7c;
            padding-left: 7px;
        }
        p {
            display: inline-block;
            vertical-align: top;
        }
        svg {
            margin: 16px 0;
        }
    </style>    
</head>
".to_string();
            html.push_str("\n<body>\n");
            let parsed_info = parse_output(output);
            // html.push_str(&process_output_to_html(output));
            html.push_str(
                &get_html_elems(
                    &(".".to_string(), ".".to_string(), parsed_info.1),
                    &parsed_info.0,
                    0,
                    &mut 0));
            html.push_str(
                &format!(
"</body>

<script>
  function fillInParentProperties(tree, parent) {{
    tree.parent = parent;
    for (var i = 0; i < tree.children.length; i++) {{
      fillInParentProperties(tree.children[i], tree);
    }}
  }}
  var tree = {};
  fillInParentProperties(tree, null);
  console.log(tree);
</script>
</html>
",
                    get_hierarchy_obj(
                        &(".".to_string(), ".".to_string(), parsed_info.1),
                        &parsed_info.0,
                        1,
                        &mut 0)));
            println!("{}", html);
        },
        Err(err) => { println!("got err when running command and getting output: {}", err); },
    }
}

fn parse_output(output: String) -> (HashMap<String, Vec<(String, String, i32)>>, i32) {
    // full path -> (full path, relative path, byte size)
    let mut children: HashMap<String, Vec<(String, String, i32)>> = HashMap::new();
    let re = Regex::new(r"(\d+)\s+(.*)").unwrap();
    let mut root_size = 0;
    for line in output.lines() {
        for cap in re.captures_iter(line) {
            let item = &cap[2];
            let size = &cap[1];
            if item != "." {
                let parent = Path::new(item).parent().unwrap().to_str().unwrap();
                let children = children.entry(parent.to_string()).or_insert(Vec::new());
                let relative_item = &item[parent.len()..];
                // eprintln!("relative_item is {}, parent is {}, item is {}", relative_item, parent, item);
                children.push(
                    (item.to_string(),
                     relative_item.to_string().trim_matches('/').to_string(),
                     size.parse::<i32>().unwrap()));
            } else {
                root_size = cap[1].parse::<i32>().unwrap();
            }
        }
    }

    (children, root_size)
}

fn get_size_label(num_bytes: i32) -> String {
    let num_bytes_fl = num_bytes as f32;
    if num_bytes_fl >= 1000000000.0 {
        format!("{:.2} GB", num_bytes_fl / 1000000000.0)
    } else if num_bytes_fl >= 1000000.0 {
        format!("{:.2} MB", num_bytes_fl / 1000000.0)
    } else if num_bytes_fl >= 1000.0 {
        format!("{:.2} KB", num_bytes_fl / 1000.0)
    } else {
        format!("{} B", num_bytes_fl)
    }
}

fn get_hierarchy_obj(
    root: &(String, String, i32),
    children_map: &HashMap<String, Vec<(String, String, i32)>>,
    indent_level: usize,
    unique_id: &mut usize) -> String
{
    // let indent = "  ".to_string().repeat(indent_level);
    let indent = "";
    let mut obj_str = format!(
        // "{}{{\n{}  id: {},\n{}  parent: null,\n{}  expanded: false,\n{}  children: [",
        "{}{{{}id:{},{}parent:null,{}expanded:false,{}children:[",
        indent,
        indent,
        unique_id,
        indent,
        indent,
        indent);
    *unique_id += 1;
    let children_str = if let Some(children) = children_map.get(&root.0) {
        children
            .iter()
            .map(|c| get_hierarchy_obj(c, children_map, indent_level + 2, unique_id))
            .collect::<Vec<String>>()
            // .join(",\n")
            .join(",")
    } else {
        "".to_string()
    };
    obj_str.push_str(
        &format!(
            // "{maybe_nl}{children}{maybe_nl}{maybe_indent}]\n{indent}}}",
            "{maybe_nl}{children}{maybe_nl}{maybe_indent}]{indent}}}",
            maybe_nl = "", //if children_str != "" { "\n" } else { "" },
            maybe_indent = "", //if children_str != "" { format!("{}  ", indent) } else { "".to_string() },
            children = children_str,
            indent = indent));

    obj_str
}

fn get_html_elems(
    root: &(String, String, i32),
    children_map: &HashMap<String, Vec<(String, String, i32)>>,
    indent_level: usize,
    unique_id: &mut usize) -> String
{
    let indent = "  ".to_string().repeat(indent_level);
    let mut html = format!(
// {}  <svg width=\"20\" height=\"20\">   <polygon points=\"0,0 10,5 0,10\" />     <circle cx=\"10\" cy=\"10\" r=\"5\" stroke=\"green\" stroke-width=\"1\" fill=\"yellow\" /></svg>
"{}<div id=\"item{}\" class=\"item\">
{}  <div class=\"item_row\">
{}    <svg width=\"15\" height=\"20\"><polygon points=\"0,4 10,10 0,16\" fill=\"#b8b4b4\" /></svg>
{}    <p>{}<span class=\"size\">({})</span></p>
{}  </div>
",
        indent,
        unique_id,
        indent,
        indent,
        indent,
        root.1,
        get_size_label(root.2),
        indent);
    *unique_id += 1;
    if let Some(children) = children_map.get(&root.0) {
        html.push_str(&format!("{}  <ul>\n", indent));
        for child in children {
            html.push_str(
                &format!("{}    <li>\n{}{}    </li>\n",
                    indent,
                    &get_html_elems(child, children_map, indent_level + 3, unique_id),
                    indent));
        }
        html.push_str(&format!("{}  </ul>\n", indent))
    }
    html.push_str(&format!("{}</div>\n", "  ".to_string().repeat(indent_level)));

    html
}

// fn print_indented(root: &String, children: &HashMap<String, Vec<String>>, indent_level: usize) {
//     println!("{}{}",
//         "  ".to_string().repeat(indent_level),
//         root);
//     if let Some(cc) = children.get(root) {
//         for c in cc {
//             print_indented(c, children, indent_level + 1);
//         }
//     }
// }

fn run_and_get_output() -> Result<String, String> {
    let cmd = "du";
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", cmd])
            .output()
            .map_err(|e| format!("failed to execute process for command `{}`: {}", cmd, e))
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .map_err(|e| format!("failed to execute process for command `{}`: {}", cmd, e))
    };

    let output = output?;
    if output.status.success() {
        Result::Ok(str::from_utf8(&output.stdout).unwrap().to_string())
    } else {
        let mut err_info = format!("execution of cmd `{}` failed.\n", cmd);
        err_info.push_str(&format!("stderr:\n{}", str::from_utf8(&output.stderr).unwrap()));
        err_info.push_str(&format!("stdout:\n{}", str::from_utf8(&output.stdout).unwrap()));
        Result::Err(err_info)
    }
}
