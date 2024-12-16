//cli.rs

use std::io::{self, Write};
use anyhow::{Result, Context};

pub fn get_user_input(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush().context("Failed to flush stdout")?;
    let mut input = String::new();
    io::stdin().read_line(&mut input).context("Failed to read input")?;
    Ok(input.trim().to_string())
}