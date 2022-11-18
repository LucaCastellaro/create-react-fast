mod scaffolding;
mod input_utils;
mod yarn_utils;
mod ts_utils;
mod loaders_utils;

use std::env;

fn main() {
    let result = input_utils::get_input_string("Nome progetto");
    if result.is_err(){
        println!("{:#?}", result.err());
        return;
    }

    let project_name = result.unwrap();

    let current_dir = env::current_dir().unwrap();
    let current_dir = current_dir.as_path().to_str().unwrap();

    let result = scaffolding::create_folder(&project_name);
    if result.is_err(){
        println!("{:#?}", result.err());
        return;
    }

    let result = scaffolding::scaffold();
    if result.is_err(){
        println!("{:#?}", result.err());
        return;
    }

    println!("App inizializzata in {}/{} ", current_dir, project_name);
}
