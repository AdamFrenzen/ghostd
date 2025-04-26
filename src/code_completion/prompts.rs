use std::{fs::File, io::Read};

pub fn get_prompts_from_file() -> anyhow::Result<Vec<String>> {
    let mut file = File::open("./src/code_completion/prompts.txt")?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let prompts: Vec<String> = buffer
        .split("\n===\n")
        .map(|s| s.trim().to_string())
        .collect();

    Ok(prompts)
}
