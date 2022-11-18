mod scaffolding;
mod input_utils;
mod yarn_utils;
mod ts_utils;

fn main() {
    let result = input_utils::get_input_string("Nome progetto");
    if result.is_err(){
        println!("{:#?}", result.err());
        return;
    }

    let project_name = result.unwrap();

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
}
