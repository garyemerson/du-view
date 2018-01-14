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
            let mut parsed_info = parse_output(output);
            sort_children_by_size(&mut parsed_info.0);
            let html_output = get_html_elems(
                &(".".to_string(), ".".to_string(), parsed_info.1),
                &parsed_info.0,
                0,
                &mut 0);
            let tree = get_hierarchy_obj(
                &(".".to_string(), ".".to_string(), parsed_info.1),
                &parsed_info.0,
                0,
                1,
                &mut 0);

            println!("
<!doctype html>
<html lang=\"en-US\">
<head>
  <title>du view</title>
  <style>
{css}
  </style>
</head>
<body onkeydown=\"handleKey(event)\">
{html_output}
</body>
<script>
  var treeRoot = {tree};
{js}
</script>
</html>\n",
                css = include_str!("style.css"),
                html_output = html_output,
                tree = tree,
                js = include_str!("script.js"));
        },
        Err(err) => { println!("got err when running command and getting output: {}", err); },
    }
}

fn sort_children_by_size(children: &mut HashMap<String, Vec<(String, String, u64)>>) {
    for v in children.values_mut() {
        v.sort_by_key(|t| - (t.2 as i64));
    }
}

// (children map, root size)
fn parse_output(output: String) -> (HashMap<String, Vec<(String, String, u64)>>, u64) {
    // full path -> (full path, relative path, byte size)
    let mut children: HashMap<String, Vec<(String, String, u64)>> = HashMap::new();
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
                     size.parse::<u64>().unwrap()));
            } else {
                root_size = cap[1].parse::<u64>().unwrap();
            }
        }
    }

    (children, root_size)
}

fn get_size_label(num_bytes: u64) -> String {
    let num_bytes_fl = num_bytes as f64;
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
    root: &(String, String, u64),
    children_map: &HashMap<String, Vec<(String, String, u64)>>,
    child_index: usize,
    indent_level: usize,
    unique_id: &mut usize) -> String
{
    let indent = "";
    let mut obj_str = format!(
        "{i}{{{i}id:{id},{i}p:null,{i}x:false,{i}ci:{ci},{i}cc:[",
        i = indent,
        ci = child_index,
        id = unique_id);
    *unique_id += 1;
    let children_str = if let Some(children) = children_map.get(&root.0) {
        children
            .iter()
            .enumerate()
            .map(|(i, c)| get_hierarchy_obj(c, children_map, i, indent_level + 2, unique_id))
            .collect::<Vec<String>>()
            .join(",")
    } else {
        "".to_string()
    };
    obj_str.push_str(
        &format!(
            "{maybe_nl}{children}{maybe_nl}{maybe_indent}]{indent}}}",
            maybe_nl = "",
            maybe_indent = "",
            children = children_str,
            indent = indent));

    obj_str
}

fn get_html_elems(
    root: &(String, String, u64),
    children_map: &HashMap<String, Vec<(String, String, u64)>>,
    indent_level: usize,
    unique_id: &mut usize) -> String
{
    let id = *unique_id;
    *unique_id += 1;

    let children_elems = if let Some(children) = children_map.get(&root.0) {
        children
            .iter()
            .map(|c|
                format!(
                    "<li>{cc}</li>",
                    cc = &get_html_elems(c, children_map, indent_level + 3, unique_id)))
            .collect::<Vec<String>>()
            .join("")
    } else {
        "".to_string()
    };

    format!(
"<div id=\"item{id}\" class=\"item\"><div id=\"item_row{id}\" class=\"item_row\">\
<div id=\"arrow{id}\" class=\"{arrow_class}\"></div><p>{name}<span class=\"size\">\
({size_label})</span></p></div>{maybe_children}</div>",
        id = id,
        name = root.1,
        size_label = get_size_label(root.2 * 1024),
        arrow_class = if children_elems == "" { "empty-arrow" } else { "arrow" },
        maybe_children =
            if children_elems == "" {
                "".to_string()
            } else {
                format!(
                    "<ul id=\"children{id}\" class=\"children\">{cc}</ul>",
                    id = id,
                    cc = children_elems)
            })
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
    let cmd = "du -a -k";
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
