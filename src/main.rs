extern crate regex;

use std::path::Path;
use std::collections::HashMap;
use regex::Regex;
use std::env;
use std::io;
use std::io::Read;

fn main() {
    let unit_size = match env::args().nth(1) {
        Some(ref arg) if arg == "-k" => {
            1024
        },
        _ => { 512 }
    };

    match read_input() {
        Ok(output) => {
            // println!("{}", output);
            let mut parsed_info = parse_output(output);
            sort_children_by_size(&mut parsed_info.0);
            // let root = get_root(parsed_info.0);
            let html_output = get_html_elems(
                &(parsed_info.2.clone(), parsed_info.2.clone(), parsed_info.1),
                &parsed_info.0,
                0,
                &mut 0,
                unit_size);
            let tree = get_hierarchy_obj(
                &(parsed_info.2.clone(), parsed_info.2.clone(), parsed_info.1),
                &parsed_info.0,
                0,
                1,
                &mut 0);

            println!("
<!doctype html>
<html lang=\"en-US\">
<head>
  <title>du</title>
  <link rel=\"icon\" type=\"image/png\" sizes=\"16x16\" href=\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAQAAAC1+jfqAAAABGdBTUEAALGPC/xhBQAAACBjSFJNAAB6JgAAgIQAAPoAAACA6AAAdTAAAOpgAAA6mAAAF3CculE8AAAAAmJLR0QAAKqNIzIAAAAJcEhZcwAADdcAAA3XAUIom3gAAAAHdElNRQfiAQ8UBgso/FxtAAABK0lEQVQoz13QMUtbUQAF4O++SCAapIraYCJCi4Oji1EX6ergUAftGjr0B7hpf4BraaHgLjq5iYOjk8RJcREKARcRFfEpkpB3Ozze4pk/OIcDdeeiKPPgypUHmSg6VwfmPYlu7VhQV7dgx63oyXwOmlIdK4IiwYqOVLMAd1rep+VOkwQlbYdg2KpVw+BQWykHwZlHJFqmTGlJ8OhMyEF0D4bU7NlTMwTuRQYQjINUx3d0pGCimL3kxBioWPbFIBhzYimv6JvzDUHVi1OvYN2cfl5B2ZZn1xoybQz4aluZAlyaVnPsg1effLZmQ3BRgMS+NyXBpll9IypebImS4oeuXT2/LfpoUkXqp196xQ+ZqhFH/klAzx8HRlVlBDT81ZApm1FG17WuxI0fbv4D6zhYOjj+Am8AAAAldEVYdGRhdGU6Y3JlYXRlADIwMTgtMDEtMTVUMjA6MDY6MTErMDE6MDDXNjdnAAAAJXRFWHRkYXRlOm1vZGlmeQAyMDE4LTAxLTE1VDIwOjA2OjExKzAxOjAwpmuP2wAAABl0RVh0U29mdHdhcmUAd3d3Lmlua3NjYXBlLm9yZ5vuPBoAAABXelRYdFJhdyBwcm9maWxlIHR5cGUgaXB0YwAAeJzj8gwIcVYoKMpPy8xJ5VIAAyMLLmMLEyMTS5MUAxMgRIA0w2QDI7NUIMvY1MjEzMQcxAfLgEigSi4A6hcRdPJCNZUAAAAASUVORK5CYII=\">
  <style>
{css}
  </style>
</head>
<body onkeydown=\"handleKey(event)\">
<div>
<div id=\"loading\">Loading</div>
<div class=\"credit\">Favicon made by <a href=\"http://www.freepik.com\" title=\"Freepik\">Freepik</a> from <a href=\"https://www.flaticon.com/\" title=\"Flaticon\">www.flaticon.com</a> is licensed by <a href=\"http://creativecommons.org/licenses/by/3.0/\" title=\"Creative Commons BY 3.0\" target=\"_blank\">CC 3.0 BY</a></div>
</div>
<div class=\"listing\">
{html_output}
</div>
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

// (children map, root size, root name)
fn parse_output(output: String) -> (HashMap<String, Vec<(String, String, u64)>>, u64, String) {
    // full path -> (full path, relative path, byte size)
    let mut children: HashMap<String, Vec<(String, String, u64)>> = HashMap::new();
    let re = Regex::new(r"(\d+)\s+(.*)").unwrap();
    let mut root_size = 0;
    let mut min = usize::max_value();
    let mut root = "".to_string();
    for line in output.lines() {
        for cap in re.captures_iter(line) {
            let item = &cap[2].trim_right_matches(|c| c == '/' || c == '\\');
            let size = &cap[1];

            if item.len() < min {
                min = item.len();
                root = item.to_string();
                root_size = cap[1].parse::<u64>().unwrap();
            }

            // eprintln!("found match with line {}, item is {} and size is {}", line, item, size);
            if let Some(parent) = Path::new(item).parent().map(|p| p.to_str().unwrap()) {
                let children = children.entry(parent.to_string()).or_insert(Vec::new());
                let relative_item = &item[parent.len()..];
                // eprintln!("relative_item is {}, parent is {}, item is {}", relative_item, parent, item);
                children.push(
                    (item.to_string(),
                     relative_item.to_string().trim_matches('/').to_string(),
                     size.parse::<u64>().unwrap()));
            }
        }
    }

    // eprintln!("root is {}", root);
    (children, root_size, root)
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

    // id: unique identifier
    // p: parent
    // x: expanded (bool)
    // ci: child index
    // cc: children
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
    unique_id: &mut usize,
    unit_size: i32) -> String
{
    let id = *unique_id;
    *unique_id += 1;

    let children_elems = if let Some(children) = children_map.get(&root.0) {
        children
            .iter()
            .map(|c|
                format!(
                    "<li>{cc}</li>",
                    cc = &get_html_elems(c, children_map, indent_level + 3, unique_id, unit_size)))
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
        size_label = get_size_label(root.2 * (unit_size as u64)),
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

fn read_input() -> Result<String, String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)
        .map_err(|e| format!("failed to read from stdin: {}", e))?;
    Ok(buffer)
}
