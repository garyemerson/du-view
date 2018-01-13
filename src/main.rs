use std::process::Command;
use std::io::{self, Write};
use std::str;

fn main() {
}

// fn open_file_skip_first_line(file_path: &str) -> Result<BufReader<File>, String> {
//     let file = File::open(file_path)
//         .map_err(|e| format!("Error opening file {}: {}", file_path, e))?;
//     let mut file_buf = BufReader::new(file);
//     file_buf.read_line(&mut String::new())
//         .map_err(|e| format!("Error reading from buf to file {}: {}", file_path, e))?;
//     Ok(file_buf)
// }

fn run_and_get_output() -> Result<String, String> {
    let cmd = "du";
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", cmd])
            .output()
            .map_err(|e| format!("failed to execute process for command `{}`: {}", cmd, e))
            // .expect(&format!("failed to execute process for command `{}`", cmd))
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .map_err(|e| format!("failed to execute process for command `{}`: {}", cmd, e))
            // .expect(&format!("failed to execute process for command `{}`", cmd))
    };

    let output = output?;
    if output.status.success() {
        // println!("execution of cmd `{}` succeeded.\n", cmd);
        // io::stdout().write(&output.stdout)
        //     .expect(&format!("failed to write stdout of command `{}` to stdout", cmd));
        
        Result::Ok(str::from_utf8(&output.stdout).unwrap().to_string())
    } else {
        let mut err_info = format!("execution of cmd `{}` failed.\n", cmd);
        err_info.push_str(&format!("stderr:\n{}", str::from_utf8(&output.stderr).unwrap()));
        err_info.push_str(&format!("stdout:\n{}", str::from_utf8(&output.stdout).unwrap()));
        Result::Err(err_info)
    }
}
