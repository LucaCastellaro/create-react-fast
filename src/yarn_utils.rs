use std::{
    fs, 
    fs::{OpenOptions},
    io::{BufReader, BufRead},
    process::Command
};

use crate::loaders_utils;

struct Script {
    label: String,
    command: String,
    is_last: bool,
}

impl Script {
    fn new(label: &str, command: &str, is_last: bool) -> Script {
        return Script { label: label.to_string(), command: command.to_string(), is_last: is_last };
    }
}

pub fn init() -> Result<bool, String> {
    let spinner = loaders_utils::get_spinner("[YARN] Inizializzo ...");

    let output = Command::new("yarn")
        .arg("init")
        .arg("-y")
        .output();

    match output {
        Err(error) => return Err(error.to_string()),
        Ok(value) => {
            if value.status.success() {
                spinner.finish_with_message("[YARN] Inizializzato");
            }
            return Ok(value.status.success())
        }
    }
}

pub fn add_scripts() -> Result<bool, String> {
    let spinner = loaders_utils::get_spinner("[YARN] Creo scripts ...");

    let result = OpenOptions::new()
        .read(true)
        .write(true)
        .open("package.json");
    if result.is_err(){
        return Err(result.err().unwrap().to_string());
    }

    let file = result.unwrap();
    let mut lines = BufReader::new(file).lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();

    let closing_parenthesis = lines.pop().unwrap();

    let license = lines.pop().unwrap();
    lines.push(format!("{},",license));

    let mut scripts: Vec<Script> = Vec::new();
    scripts.push(Script::new("start", "react-scripts start", false));
    scripts.push(Script::new("build", "react-scripts build", false));
    scripts.push(Script::new("test", "react-scripts test", false));
    scripts.push(Script::new("eject", "react-scripts eject", true));

    lines.push("\t\"scripts\": {".to_string());
    for script in scripts {
        spinner.set_message(format!("Creo script '{}' ...", script.label));
        lines.push(get_formatted_script(script.label, script.command, script.is_last));

    }
    lines.push("\t}".to_string());
    lines.push(closing_parenthesis);

    let lines = lines.join("\n").replace("  ", "\t");

    let result = fs::write("package.json", lines);
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(_) => {
            spinner.finish_with_message("[YARN] Script creati");
            return Ok(true);
        }
    }
}

pub fn add_browsers() -> Result<bool, String> {
    let spinner = loaders_utils::get_spinner("[YARN] Aggiungo browsers ...");

    let result = OpenOptions::new()
        .read(true)
        .write(true)
        .open("package.json");
    if result.is_err(){
        return Err(result.err().unwrap().to_string());
    }

    let file = result.unwrap();
    let mut lines = BufReader::new(file).lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();

    let last_line = lines.pop().unwrap();
    let previous_line = lines.pop().unwrap();

    lines.push(format!("{},", previous_line));
    lines.push("\t\"browserslist\": {\n\t\t\"production\": [\n\t\t\t\">0.2%\",\n\t\t\t\"not dead\",\n\t\t\t\"not op_mini all\"\n\t\t],\n\t\t\"development\": [\n\t\t\t\"last 1 chrome version\",\n\t\t\t\"last 1 firefox version\",\n\t\t\t\"last 1 safari version\"\n\t\t]\n\t}".to_string());
    lines.push(last_line);

    let lines = lines.join("\n");

    let result = fs::write("package.json", lines);
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(_) => {
            spinner.finish_with_message("[YARN] Browsers aggiunti");
            return Ok(true);
        }
    }
}

pub fn add_packages() -> Result<bool, String> {
    let spinner = loaders_utils::get_spinner("[YARN] Installo pacchetti ...");

    let output = Command::new("yarn")
        .arg("add")
        .arg("react")
        .arg("react-dom")
        .arg("react-scripts")
        .arg("@types/node")
        .arg("@types/react")
        .arg("@types/react-dom")
        .arg("typescript")
        .output();

    match output {
        Err(error) => return Err(error.to_string()),
        Ok(value) => {
            if value.status.success() {
                spinner.finish_with_message("[YARN] Pacchetti installati");
            }
            return Ok(value.status.success());
        }
    }
}

fn get_formatted_script(label: String, command: String, is_last: bool) -> String {
    let mut script = format!("\t\t\"{label}\": \"{command}\"");
    if !is_last {
        script.push_str(",");
    }
    
    return script;
}