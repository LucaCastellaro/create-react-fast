use dialoguer::Input;

pub fn get_input_string(label: &str) -> Result<String, String> {
    let input = Input::<String>::new()
        .with_prompt(label)
        .interact_text();

    match input {
        Err(error) => return Err(error.to_string()),
        Ok(value) => return Ok(value)
    }
}