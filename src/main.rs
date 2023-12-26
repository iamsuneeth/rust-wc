use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs,
    io::{self, Read},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'c')]
    bytes: bool,
    #[arg(short = 'l')]
    lines: bool,
    #[arg(short = 'w')]
    word: bool,
    #[arg(short = 'm')]
    characters: bool,
    #[arg(required = false)]
    file_name: Option<String>,
}

fn get_content(file_name: &Option<String>) -> Result<String> {
    let mut input = String::new();
    if let Some(value) = file_name {
        fs::read_to_string(value).with_context(|| format!("Failed to read file {value}"))
    } else {
        io::stdin()
            .read_to_string(&mut input)
            .with_context(|| "Failed to read data from standard input")?;
        Ok(input)
    }
}

fn get_output(result: String, file_name: &Option<String>) -> String {
    if let Some(value) = file_name {
        format!("{result} {value}")
    } else {
        result
    }
}

fn main() -> Result<()> {
    let Args {
        bytes,
        lines,
        word,
        characters,
        file_name,
    } = Args::parse();

    let contents = get_content(&file_name)?;
    let byte_count = contents.bytes().count();
    let line_count = contents.lines().count();
    let word_count = contents.split_whitespace().count();
    let character_count = contents.chars().count();
    let format_string = if bytes {
        format!("{byte_count}")
    } else if lines {
        format!("{line_count}")
    } else if word {
        format!("{word_count}")
    } else if characters {
        format!("{character_count}")
    } else {
        format!("{line_count}\t{word_count}\t{byte_count}")
    };
    let output = get_output(format!("{}", format_string), &file_name);
    println!("{}", output);
    Ok(())
}
