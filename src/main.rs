use std::process;
use serde_json::{self, Value};
use rofi;

fn get_json_input() ->String {
    let json_input = process::Command::new("dunstctl")
        .arg("history")
        .output()
        .expect("Failed to get Notifications history from dunst!");
    return String::from_utf8_lossy(&json_input.stdout).to_string();
}

fn get_value(json_input:String) -> (Vec<String>, Vec<i64>) {
    let mut output_vec_id:Vec<i64> = std::vec::Vec::new();
    let mut output_vec:Vec<String> = std::vec::Vec::new();
    let read_json: Value = serde_json::from_str(&json_input).unwrap();
    for data in read_json["data"].as_array().unwrap() {
    for item in data.as_array().unwrap() {
        let appname = item["appname"]["data"].as_str().unwrap().to_string();
        let summary = item["summary"]["data"].as_str().unwrap().to_string();
        let app_id = item["id"]["data"].as_i64().unwrap();
        let output:String = format!("{} - {}", appname, summary);
        output_vec.push(output);
        output_vec_id.push(app_id);
        }
    }
return (output_vec, output_vec_id);
}

fn spawn_rofi (output_vec:Vec<String>) -> Result<usize, rofi::Error> {
    let selected = rofi::Rofi::new(&output_vec)
        .prompt("  History ")
        .run_index();
    return selected;
}

fn main () {
    let output = get_value(get_json_input());
    let user_input:Result<usize, rofi::Error> = spawn_rofi(output.0);
    match &user_input {
        Ok(v) => {
            // v is the vector index Value
            let vec_id = output.1;
            process::Command::new("dunstctl")
                .arg("history-pop")
                .arg(format!("{}",vec_id[*v]))
                .spawn()
                .expect("Failed to pop the Notifications!");
        }
        Err(e) => {
            println!("Error : {e}");
            process::exit(1);
        }
    }
}
